use std::time::Duration;

use crate::domain_ids::{FixtureId, RefereeId, TeamId, VenueId};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use log::{info, warn};
use mockall::automock;
use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{CommitMode, Consumer, ConsumerContext, Rebalance, StreamConsumer},
    error::KafkaResult,
    producer::{FutureProducer, Producer},
    util::get_rdkafka_version,
    ClientConfig, ClientContext, Message, TopicPartitionList,
};
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
    TeamCreated {
        team_id: TeamId,
    },
    VenueCreated {
        venue_id: VenueId,
    },
    FixtureCreated {
        fixture_id: FixtureId,
    },
    FixtureDateChanged {
        fixture_id: FixtureId,
        date: DateTime<Utc>,
    },
    FixtureVenueChanged {
        fixture_id: FixtureId,
        venue_id: VenueId,
    },
    FixtureCancelled {
        fixture_id: FixtureId,
    },
}

impl DomainEvent {
    pub fn deserialize_from_str(s: &str) -> Result<Self, String> {
        serde_json::from_str(s).map_err(|e| e.to_string())
    }
}

#[async_trait]
#[automock]
pub trait DomainEventPublisher {
    async fn begin_transaction(&self) -> Result<(), String>;
    async fn commit_transaction(&self) -> Result<(), String>;
    async fn publish_domain_event(&self, event: DomainEvent) -> Result<(), String>;
}

pub struct KafkaDomainEventProducer {
    kafka_producer: FutureProducer,
    topic: String,
}

impl KafkaDomainEventProducer {
    pub fn new(kafka_url: &str, topic: &str, transactional_id: &str) -> Self {
        let (version_n, version_s) = get_rdkafka_version();
        info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

        let kafka_producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", kafka_url)
            .set("message.timeout.ms", "5000")
            .set("transactional.id", transactional_id)
            .create()
            .expect("Kafka producer creation error");

        kafka_producer
            .init_transactions(Duration::from_secs(10))
            .unwrap();

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

    async fn begin_transaction(&self) -> Result<(), String> {
        self.kafka_producer
            .begin_transaction()
            .map_err(|e| e.to_string())
    }

    async fn commit_transaction(&self) -> Result<(), String> {
        self.kafka_producer
            .commit_transaction(Duration::from_secs(10))
            .map_err(|e| e.to_string())
    }
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

#[async_trait]
pub trait DomainEventCallbacks {
    async fn on_referee_created(&mut self, referee_id: RefereeId);
    async fn on_referee_club_changed(&mut self, referee_id: RefereeId, club_name: String);
    async fn on_team_created(&mut self, team_id: TeamId);
    async fn on_venue_created(&mut self, venue_id: VenueId);
    async fn on_fixture_created(&mut self, fixture_id: FixtureId);
    async fn on_fixture_date_changed(&mut self, fixture_id: FixtureId, date: DateTime<Utc>);
    async fn on_fixture_venue_changed(&mut self, fixture_id: FixtureId, venue_id: VenueId);
    async fn on_fixture_cancelled(&mut self, fixture_id: FixtureId);
}

pub struct DomainEventConsumer {
    kafka_consumer: StreamConsumer<CustomContext>,
    callbacks: Box<dyn DomainEventCallbacks + Send + Sync>,
}

impl DomainEventConsumer {
    pub fn new(
        consumer_group: &str,
        broker_url: &str,
        domain_events_topic: &str,
        callbacks: Box<dyn DomainEventCallbacks + Send + Sync>,
    ) -> Self {
        let context = CustomContext;
        let kafka_consumer: StreamConsumer<CustomContext> = ClientConfig::new()
            .set("group.id", consumer_group)
            .set("bootstrap.servers", broker_url)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "10000")
            .set("enable.auto.commit", "false")
            .set("auto.offset.reset", "latest") // NOTE: if a new service joins the party, it will start consuming from the latest offset, as it wont make sense to replay all events
            .set_log_level(RDKafkaLogLevel::Debug)
            .create_with_context(context)
            .expect("Kafka consumer creation failed");

        kafka_consumer
            .subscribe(&[&domain_events_topic as &str])
            .expect(&format!(
                "Can't subscribe to Domain Events topic {}",
                domain_events_topic
            ));

        Self {
            kafka_consumer,
            callbacks,
        }
    }

    pub async fn run(&mut self) {
        loop {
            match self.kafka_consumer.recv().await {
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
                            self.callbacks.on_referee_created(referee_id).await;
                        }
                        DomainEvent::RefereeClubChanged {
                            referee_id,
                            club_name,
                        } => {
                            self.callbacks
                                .on_referee_club_changed(referee_id, club_name)
                                .await;
                        }
                        DomainEvent::TeamCreated { team_id } => {
                            self.callbacks.on_team_created(team_id).await;
                        }
                        DomainEvent::VenueCreated { venue_id } => {
                            self.callbacks.on_venue_created(venue_id).await;
                        }
                        DomainEvent::FixtureCreated { fixture_id } => {
                            self.callbacks.on_fixture_created(fixture_id).await;
                        }
                        DomainEvent::FixtureDateChanged { fixture_id, date } => {
                            self.callbacks
                                .on_fixture_date_changed(fixture_id, date)
                                .await;
                        }
                        DomainEvent::FixtureVenueChanged {
                            fixture_id,
                            venue_id,
                        } => {
                            self.callbacks
                                .on_fixture_venue_changed(fixture_id, venue_id)
                                .await;
                        }
                        DomainEvent::FixtureCancelled { fixture_id } => {
                            self.callbacks.on_fixture_cancelled(fixture_id).await;
                        }
                    }
                    self.kafka_consumer
                        .commit_message(&m, CommitMode::Sync)
                        .unwrap();
                }
            };
        }
    }
}
