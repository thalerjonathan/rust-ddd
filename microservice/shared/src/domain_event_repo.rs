use chrono::{DateTime, Utc};
use log::{error, info};
use mockall::automock;
use serde_json::Value;
use sqlx::{postgres::PgListener, PgPool};
use uuid::Uuid;

use crate::domain_events::{
    run_domain_event_publisher_transactional, DomainEvent, DomainEventMessage, DomainEventPublisher,
};

#[derive(sqlx::Type, Debug, Copy, Clone, serde::Deserialize)]
#[sqlx(type_name = "rustddd.domain_event_type")]
pub enum DomainEventTypeDb {
    Inbox,
    Outbox,
}

#[derive(Debug)]
pub struct DomainEventDb {
    pub id: Uuid,
    pub event_type: DomainEventTypeDb,
    pub payload: serde_json::Value,
    pub instance: Uuid,
    pub processed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[allow(async_fn_in_trait)]
#[automock(type Error = String; type TxCtx = ();)]
pub trait DomainEventOutboxRepository {
    type TxCtx;
    type Error;

    async fn store(
        &self,
        event: DomainEvent,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<DomainEventDb, Self::Error>;
}

pub struct DomainEventRepositoryPg {
    instance: Uuid,
}

impl DomainEventRepositoryPg {
    pub fn new(instance: Uuid) -> Self {
        Self { instance }
    }

    pub async fn mark_event_as_processed(
        &self,
        event_id: Uuid,
        tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    ) -> Result<(), String> {
        sqlx::query!(
            "UPDATE rustddd.domain_events SET processed_at = $1 WHERE id = $2",
            Utc::now(),
            event_id,
        )
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_unprocessed_outbox_events(
        &self,
        tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    ) -> Result<Vec<DomainEventDb>, String> {
        let outbox_event_type = DomainEventTypeDb::Outbox;
        let events = sqlx::query_as!(
            DomainEventDb,
            "SELECT id, event_type as \"event_type: DomainEventTypeDb\", payload, instance, processed_at, created_at 
            FROM rustddd.domain_events 
            WHERE event_type = $1
            AND instance = $2
            AND processed_at IS NULL",
            outbox_event_type as DomainEventTypeDb,
            self.instance,
        )
        .fetch_all(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(events)
    }

    pub async fn is_event_processed(
        &self,
        event_id: Uuid,
        tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    ) -> Result<Option<DateTime<Utc>>, String> {
        let ret = sqlx::query_as!(
            DomainEventDb,
            "SELECT id, event_type as \"event_type: DomainEventTypeDb\", payload, instance, processed_at, created_at FROM rustddd.domain_events WHERE id = $1",
            event_id,
        )
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(ret.processed_at)
    }

    pub async fn store_as_inbox(
        &self,
        event: &DomainEventMessage,
        tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    ) -> Result<DomainEventDb, String> {
        let domain_event_db = DomainEventDb {
            id: event.event_id,
            event_type: DomainEventTypeDb::Inbox,
            payload: serde_json::to_value(&event).map_err(|e| e.to_string())?,
            instance: self.instance,
            processed_at: None,
            created_at: Utc::now(),
        };

        sqlx::query!(
            "INSERT INTO rustddd.domain_events (id, event_type, payload, instance, created_at)
            VALUES ($1, $2, $3, $4, $5)",
            domain_event_db.id,
            domain_event_db.event_type as DomainEventTypeDb,
            domain_event_db.payload,
            domain_event_db.instance,
            domain_event_db.created_at,
        )
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(domain_event_db)
    }
}

impl DomainEventOutboxRepository for DomainEventRepositoryPg {
    type TxCtx = sqlx::Transaction<'static, sqlx::Postgres>;
    type Error = String;

    async fn store(
        &self,
        event: DomainEvent,
        tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    ) -> Result<DomainEventDb, String> {
        let domain_event_db = DomainEventDb {
            id: Uuid::new_v4(),
            event_type: DomainEventTypeDb::Outbox,
            payload: serde_json::to_value(&event).map_err(|e| e.to_string())?,
            instance: self.instance,
            processed_at: None,
            created_at: Utc::now(),
        };

        sqlx::query!(
            "INSERT INTO rustddd.domain_events (id, event_type, payload, instance, created_at)
            VALUES ($1, $2, $3, $4, $5)",
            domain_event_db.id,
            domain_event_db.event_type as DomainEventTypeDb,
            domain_event_db.payload,
            domain_event_db.instance,
            domain_event_db.created_at,
        )
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(domain_event_db)
    }
}

#[derive(Debug, serde::Deserialize)]
struct DomainEventInsertedNotificationPayload {
    event_id: Uuid,
    event_type: DomainEventTypeDb,
    payload: Value,
    instance: Uuid,
    created_at: DateTime<Utc>,
}

pub async fn process_domain_events_outbox(
    instance: Uuid,
    connection_pool: PgPool,
    domain_event_publisher: Box<dyn DomainEventPublisher + Send + Sync>,
) -> Result<(), String> {
    // NOTE: we fetch all unprocessed events for the instance and process them - these are the events that failed to be processed upon notification, due to some infrastructure failure -
    // so we need to process them again because this point because this consistutes a retry
    let domain_event_repository = DomainEventRepositoryPg::new(instance);
    let mut tx = connection_pool.begin().await.map_err(|e| e.to_string())?;
    let unprocessed_events = domain_event_repository
        .get_unprocessed_outbox_events(&mut tx)
        .await
        .map_err(|e| e.to_string())?;

    // NOTE: the events are already filtered for the instance and are only outbox events, so we don't need to filter them here
    for event in unprocessed_events {
        info!("Processing unprocessed outbox event: {event:?}");
        process_domain_event(
            event,
            &connection_pool,
            &domain_event_repository,
            &domain_event_publisher,
        )
        .await?;
    }

    let mut listener = PgListener::connect_with(&connection_pool)
        .await
        .map_err(|e| e.to_string())?;
    listener
        .listen_all(vec!["domain_event_inserted"])
        .await
        .map_err(|e| e.to_string())?;

    loop {
        info!("Waiting for domain event notification for instance {instance}...");
        let notification = listener.recv().await.map_err(|e| e.to_string())?;
        info!("Received notification in processing_domain_events: {notification:?}");

        let payload_result: Result<DomainEventInsertedNotificationPayload, String> =
            serde_json::from_str(&notification.payload()).map_err(|e| e.to_string());

        match payload_result {
            Ok(payload) => {
                info!("DomainEvent notification payload: {payload:?}");

                // NOTE: only process outbox events
                if let DomainEventTypeDb::Outbox = payload.event_type {
                    // NOTE: only process events put into the outbox by this instance, that is, every instance should only process its own events
                    if payload.instance == instance {
                        let domain_event_db = DomainEventDb {
                            id: payload.event_id,
                            event_type: payload.event_type,
                            payload: payload.payload,
                            instance: payload.instance,
                            processed_at: None,
                            created_at: payload.created_at,
                        };

                        process_domain_event(
                            domain_event_db,
                            &connection_pool,
                            &domain_event_repository,
                            &domain_event_publisher,
                        )
                        .await?;
                    }
                }
            }
            Err(e) => {
                error!("Error deserializing domain event notification payload: {e} - skipping");
            }
        }
    }
}

async fn process_domain_event(
    domain_event_db: DomainEventDb,
    connection_pool: &PgPool,
    domain_event_repository: &DomainEventRepositoryPg,
    domain_event_publisher: &Box<dyn DomainEventPublisher + Send + Sync>,
) -> Result<(), String> {
    let domain_event_result: Result<DomainEvent, String> =
        serde_json::from_value(domain_event_db.payload).map_err(|e| e.to_string());

    match domain_event_result {
        Ok(domain_event) => {
            // NOTE: if this fails, the function exists, and will be retried, so we don't need to handle the failure here
            run_domain_event_publisher_transactional(&domain_event_publisher, async {
                let domain_event_message = DomainEventMessage {
                    event: domain_event,
                    event_id: domain_event_db.id,
                    instance_id: domain_event_db.instance,
                    created_at: domain_event_db.created_at,
                };

                domain_event_publisher
                    .publish(domain_event_message)
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(())
            })
            .await?;

            // NOTE: if any of these fail, the function exists, and will be retried, which will have the effect,
            // that the domain event will be published again, resulting in a duplicate message (at-least-once semantics),
            // which has then to be handled by the consuming part via dedup mechanism or idempotency
            let mut tx = connection_pool.begin().await.map_err(|e| e.to_string())?;
            domain_event_repository
                .mark_event_as_processed(domain_event_db.id, &mut tx)
                .await
                .map_err(|e| e.to_string())?;
            tx.commit().await.map_err(|e| e.to_string())?;

            Ok(())
        }
        Err(e) => {
            error!("Error deserializing domain event while processing: {e} - skipping");
            Err(e)
        }
    }
}
