use crate::domain_ids::RefereeId;
use async_trait::async_trait;
use rdkafka::producer::FutureProducer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum DomainEvent {
    RefereeCreated {
        referee_id: RefereeId,
    },
    RefereeClubChanged {
        referee_id: RefereeId,
        club_name: String,
    },
}

impl DomainEvent {
    pub fn deserialize_from_str(s: &str) -> Result<Self, String> {
        serde_json::from_str(s).map_err(|e| e.to_string())
    }
}

#[async_trait]
pub trait DomainEventPublisher {
    async fn publish_domain_event(&self, event: DomainEvent) -> Result<(), String>;
}

pub struct KafkaDomainEventProducer {
    kafka_producer: FutureProducer,
    topic: String,
}

impl KafkaDomainEventProducer {
    pub fn new(kafka_producer: FutureProducer, topic: &str) -> Self {
        Self {
            kafka_producer,
            topic: topic.to_string(),
        }
    }
}

#[async_trait]
impl DomainEventPublisher for KafkaDomainEventProducer {
    async fn publish_domain_event(&self, event: DomainEvent) -> Result<(), String> {
        let msg = serde_json::to_string(&event).unwrap();

        let sr = self.kafka_producer.send_result(
            rdkafka::producer::FutureRecord::to(&self.topic)
                .payload(&msg)
                .key(""),
        );

        match sr {
            Ok(delivery_fut) => {
                let res = delivery_fut.await;
                match res {
                    Err(err) => Err(format!("Kafka delivery canceled: {:?}", err)),
                    Ok(Ok(_)) => Ok(()),
                    Ok(Err(err)) => Err(format!("Kafka error: {:?}", err)),
                }
            }
            Err((err, _)) => Err(format!("Kafka error: {:?}", err)),
        }
    }
}
