use axum::http::Method;
use axum::{
    routing::{get, post},
    Router,
};
use clap::Parser;

use log::{info, warn};
use microservices_shared::domain_events::{DomainEvent, KafkaDomainEventProducer};
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance, StreamConsumer};
use rdkafka::error::KafkaResult;
use rdkafka::producer::FutureProducer;
use rdkafka::util::get_rdkafka_version;
use rdkafka::{ClientConfig, ClientContext, Message, TopicPartitionList};
use redis::Commands;
use referees::config::AppConfig;
use referees::ports::rest::referee::{
    create_referee_handler, get_all_referees_handler, get_referee_by_id_handler,
    update_referee_club_handler,
};
use referees::AppState;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    server_host: String,
}

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}

type DomainEventConsumer = StreamConsumer<CustomContext>;

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();
    let config = AppConfig::new_from_env();

    let connection_pool = PgPool::connect(&config.db_url).await.unwrap();
    let redis_client = redis::Client::open(config.redis_url).unwrap();

    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let kafka_producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", config.kafka_url.clone())
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Kafka producer creation error");

    let context = CustomContext;
    let kafka_consumer: DomainEventConsumer = ClientConfig::new()
        .set("group.id", config.kafka_consumer_group)
        .set("bootstrap.servers", config.kafka_url.clone())
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Kafka consumer creation failed");

    kafka_consumer
        .subscribe(&[&config.kafka_domain_events_topic as &str])
        .expect(&format!(
            "Can't subscribe to Domain Events topic {}",
            config.kafka_domain_events_topic
        ));

    let domain_event_producer =
        KafkaDomainEventProducer::new(kafka_producer, &config.kafka_domain_events_topic);

    let app_state = AppState {
        connection_pool,
        redis_client: redis_client.clone(),
        domain_event_publisher: Box::new(domain_event_producer),
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
        .route("/referees", post(create_referee_handler))
        .route("/referees/:id", get(get_referee_by_id_handler))
        .route("/referees/all", get(get_all_referees_handler))
        .route("/referees/:id/club", post(update_referee_club_handler))
        .layer(cors)
        .with_state(state_arc);

    let listener = tokio::net::TcpListener::bind(&args.server_host)
        .await
        .unwrap();

    tokio::spawn(async move {
        run_kafka_consumer(kafka_consumer, redis_client).await;
    });

    axum::serve(listener, app).await.unwrap();
}

async fn run_kafka_consumer(consumer: DomainEventConsumer, redis_client: redis::Client) {
    let mut redis_conn = redis_client.get_connection().unwrap();
    loop {
        match consumer.recv().await {
            Err(e) => warn!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };

                let domain_event = DomainEvent::deserialize_from_str(&payload).unwrap();
                match domain_event {
                    DomainEvent::RefereeCreated { referee_id } => {
                        info!("Received Domain Event: Referee created: {:?}", referee_id);
                    }
                    DomainEvent::RefereeClubChanged {
                        referee_id,
                        club_name,
                    } => {
                        info!(
                            "Received Domain Event: Referee club changed: {:?} -> {}",
                            referee_id, club_name
                        );

                        info!("Invalidating cache entry for referee: {:?}", referee_id);

                        // NOTE: invalidate the cache entry for the referee
                        let key = format!("referee_{}", referee_id.0.to_string());
                        let _result: Result<(), redis::RedisError> = redis_conn.del(key);
                    }
                }
                consumer.commit_message(&m, CommitMode::Sync).unwrap();
            }
        };
    }
}
