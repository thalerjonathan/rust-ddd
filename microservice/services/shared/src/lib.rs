use std::time::Duration;

use opentelemetry::{
    global::{self, BoxedTracer},
    KeyValue,
};
use opentelemetry_otlp::{ExportConfig, Protocol, WithExportConfig};
use opentelemetry_sdk::{
    metrics::reader::DefaultTemporalitySelector,
    trace::{Config, RandomIdGenerator, Sampler},
    Resource,
};

pub mod domain_event_repo;
pub mod domain_events;
pub mod domain_ids;
pub mod resolvers;
pub mod token;

pub fn init_tracing(otlp_endpoint: &str, service_name: &str) -> BoxedTracer {
    let tracer_provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(otlp_endpoint)
                .with_timeout(Duration::from_secs(3)),
        )
        .with_trace_config(
            Config::default()
                .with_sampler(Sampler::AlwaysOn)
                .with_id_generator(RandomIdGenerator::default())
                .with_max_events_per_span(64)
                .with_max_attributes_per_span(16)
                .with_max_events_per_span(16)
                .with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    service_name.to_string(),
                )])),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .unwrap();
    global::set_tracer_provider(tracer_provider);
    let tracer = global::tracer(service_name.to_string());

    let export_config = ExportConfig {
        endpoint: otlp_endpoint.to_string(),
        timeout: Duration::from_secs(3),
        protocol: Protocol::Grpc,
    };

    let _meter = opentelemetry_otlp::new_pipeline()
        .metrics(opentelemetry_sdk::runtime::Tokio)
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_export_config(export_config),
            // can also config it using with_* functions like the tracing part above.
        )
        .with_resource(Resource::new(vec![KeyValue::new(
            "service.name",
            service_name.to_string(),
        )]))
        .with_period(Duration::from_secs(3))
        .with_timeout(Duration::from_secs(10))
        .with_temporality_selector(DefaultTemporalitySelector::new())
        .build();

    tracer
}
