use chrono::{DateTime, Utc};
use mockall::automock;
use uuid::Uuid;

use crate::domain_events::{DomainEvent, DomainEventMessage};

#[derive(Debug)]
pub struct DomainEventOutboxDb {
    pub id: Uuid,
    pub instance: Uuid,
    pub payload: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct DomainEventInboxDb {
    pub id: Uuid,
    pub instance: Uuid,
    pub payload: serde_json::Value,
    pub processed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[allow(async_fn_in_trait)]
#[automock(type Error = String; type TxCtx = ();)]
pub trait DomainEventOutboxRepository {
    type TxCtx;
    type Error;

    async fn store(&self, event: DomainEvent, tx_ctx: &mut Self::TxCtx) -> Result<(), Self::Error>;
}

pub struct DomainEventRepositoryPg {
    instance: Uuid,
}

impl DomainEventRepositoryPg {
    pub fn new(instance: Uuid) -> Self {
        Self { instance }
    }

    pub async fn mark_inbox_event_as_processed(
        &self,
        event_id: Uuid,
        tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    ) -> Result<(), String> {
        sqlx::query!(
            "UPDATE rustddd.domain_events_inbox SET processed_at = $1 WHERE id = $2",
            Utc::now(),
            event_id,
        )
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn is_inbox_event_processed(
        &self,
        event_id: Uuid,
        tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    ) -> Result<Option<DateTime<Utc>>, String> {
        let ret: Option<DomainEventInboxDb> = sqlx::query_as!(
            DomainEventInboxDb,
            "SELECT id, instance, payload, processed_at, created_at 
            FROM rustddd.domain_events_inbox 
            WHERE id = $1",
            event_id
        )
        .fetch_optional(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(ret.map(|e| e.processed_at).flatten())
    }

    pub async fn store_as_inbox(
        &self,
        msg: &DomainEventMessage,
        tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    ) -> Result<DomainEventInboxDb, String> {
        let domain_event_db = DomainEventInboxDb {
            id: msg.id,
            payload: serde_json::to_value(&msg.payload).map_err(|e| e.to_string())?,
            instance: self.instance,
            processed_at: None,
            created_at: Utc::now(),
        };

        sqlx::query!(
            "INSERT INTO rustddd.domain_events_inbox (id, payload, instance, created_at)
            VALUES ($1, $2, $3, $4)",
            domain_event_db.id,
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
    ) -> Result<(), String> {
        let event_id = Uuid::new_v4();
        let created_at = Utc::now();
        let payload = serde_json::to_value(&event).map_err(|e| e.to_string())?;

        let domain_event_outbox_db = DomainEventOutboxDb {
            id: event_id,
            instance: self.instance,
            payload,
            created_at,
        };

        sqlx::query!(
            "INSERT INTO rustddd.domain_events_outbox (id, instance, payload, created_at)
            VALUES ($1, $2, $3, $4)",
            domain_event_outbox_db.id,
            domain_event_outbox_db.instance,
            domain_event_outbox_db.payload,
            domain_event_outbox_db.created_at,
        )
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}
