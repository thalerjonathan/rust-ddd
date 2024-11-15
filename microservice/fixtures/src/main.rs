use axum::http::Method;
use axum::Extension;
use axum::{
    routing::{get, post},
    Router,
};
use clap::Parser;

use fixtures::config::AppConfig;
use fixtures::ports::kafka::domain_events_handler::DomainEventCallbacksImpl;
use fixtures::ports::rest::fixtures::{
    cancel_fixture_handler, create_fixture_handler, get_all_fixtures_handler,
    get_fixture_by_id_handler, update_fixture_date_handler, update_fixture_venue_handler,
};
use fixtures::AppState;
use log::error;
use microservices_shared::domain_event_repo::process_domain_events_outbox;
use microservices_shared::domain_events::{DomainEventConsumer, KafkaDomainEventProducer};
use opentelemetry::{
    trace::{Span, Tracer},
    KeyValue,
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    server_host: String,
    #[arg(short, long)]
    instance_id: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();
    let config = AppConfig::new_from_env();
    let instance_id = Uuid::parse_str(&args.instance_id).unwrap();

    let tracer = microservices_shared::init_tracing(&config.otlp_endpoint, "fixtures");
    let mut span = tracer.start("application_start");
    span.set_attribute(KeyValue::new("server_host", args.server_host.clone()));

    let tracer_arc = Arc::new(tracer);

    let connection_pool = PgPool::connect(&config.db_url).await.unwrap();
    let redis_client = redis::Client::open(config.redis_url).unwrap();

    let domain_event_producer = KafkaDomainEventProducer::new(
        &config.kafka_url,
        &config.kafka_domain_events_topic,
        &args.instance_id,
    );
    let domain_event_callbacks = Box::new(DomainEventCallbacksImpl::new(
        redis_client.get_connection().unwrap(),
        tracer_arc.clone(),
    ));
    let mut domain_event_consumer = DomainEventConsumer::new(
        &config.kafka_consumer_group,
        &config.kafka_url,
        &config.kafka_domain_events_topic,
        connection_pool.clone(),
        instance_id.clone(),
        domain_event_callbacks,
    );

    let app_state = AppState {
        connection_pool: connection_pool.clone(),
        redis_client,
        tracer: tracer_arc,
    };
    let state_arc = Arc::new(app_state);

    let cors = tower_http::cors::CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(tower_http::cors::Any)
        .allow_origin(tower_http::cors::Any);

    let app = Router::new()
        .route("/fixtures", post(create_fixture_handler))
        .route("/fixtures/:id", get(get_fixture_by_id_handler))
        .route("/fixtures/all", get(get_all_fixtures_handler))
        .route("/fixtures/:id/date", post(update_fixture_date_handler))
        .route("/fixtures/:id/venue", post(update_fixture_venue_handler))
        .route("/fixtures/:id/cancel", post(cancel_fixture_handler))
        .layer(cors)
        .layer(Extension(instance_id.clone()))
        .with_state(state_arc);

    let listener = tokio::net::TcpListener::bind(&args.server_host)
        .await
        .unwrap();

    tokio::spawn(async move {
        domain_event_consumer.run().await;
    });

    tokio::spawn({
        let instance = instance_id.clone();
        let pool = connection_pool.clone();
        let domain_event_publisher = Box::new(domain_event_producer);
        async move {
            if let Err(e) =
                process_domain_events_outbox(instance, pool, domain_event_publisher).await
            {
                // TODO: retry in case of DB disconnect
                error!("Error processing domain events: {e}");
            }
        }
    });

    span.end();

    axum::serve(listener, app).await.unwrap();
}
