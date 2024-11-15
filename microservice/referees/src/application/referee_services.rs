use crate::domain::{aggregates::referee::Referee, repositories::referee_repo::RefereeRepository};
use microservices_shared::{
    domain_event_repo::DomainEventOutboxRepository, domain_events::DomainEvent,
    domain_ids::RefereeId,
};

pub async fn create_referee<TxCtx>(
    name: &str,
    club: &str,
    repo: &impl RefereeRepository<TxCtx = TxCtx, Error = String>,
    domain_event_repo: &impl DomainEventOutboxRepository<TxCtx = TxCtx, Error = String>,
    tx_ctx: &mut TxCtx,
) -> Result<Referee, String> {
    let referee = Referee::new(name, club);

    repo.save(&referee, tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

    domain_event_repo
        .store(
            DomainEvent::RefereeCreated {
                referee_id: referee.id().clone(),
            },
            tx_ctx,
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(referee)
}

pub async fn update_referee_club<TxCtx>(
    referee_id: RefereeId,
    club: &str,
    repo: &impl RefereeRepository<TxCtx = TxCtx, Error = String>,
    domain_event_repo: &impl DomainEventOutboxRepository<TxCtx = TxCtx, Error = String>,
    tx_ctx: &mut TxCtx,
) -> Result<(), String> {
    let referee = repo.find_by_id(referee_id, tx_ctx).await?;
    let mut referee = referee.ok_or("Referee not found")?;
    referee.change_club(club);

    repo.save(&referee, tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

    domain_event_repo
        .store(
            DomainEvent::RefereeClubChanged {
                referee_id: referee.id().clone(),
                club_name: club.to_string(),
            },
            tx_ctx,
        )
        .await
        .map_err(|e| e.to_string())?;

    // NOTE: the cache entry is invalidated by the domain event listener of a Referee service instance
    // to keep the application layer free from caching logic

    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use microservices_shared::{
        domain_event_repo::{DomainEventDb, DomainEventOutboxRepository, DomainEventTypeDb},
        domain_events::DomainEvent,
        domain_ids::RefereeId,
    };
    use uuid::Uuid;

    use crate::{
        application::referee_services::{create_referee, update_referee_club},
        domain::{aggregates::referee::Referee, repositories::referee_repo::RefereeRepository},
    };
    use std::cell::RefCell;
    use std::collections::HashMap;

    struct TestDomainEventRepository {}

    struct TestRepo {
        // NOTE: we use a RefCell to allow for interior mutability because RefereeRepository trait does not pass by mutable reference
        data: RefCell<HashMap<RefereeId, Referee>>,
    }

    impl TestRepo {
        fn new() -> Self {
            Self {
                data: RefCell::new(HashMap::new()),
            }
        }
    }

    impl DomainEventOutboxRepository for TestDomainEventRepository {
        type TxCtx = ();
        type Error = String;

        async fn store(
            &self,
            _event: DomainEvent,
            _tx_ctx: &mut Self::TxCtx,
        ) -> Result<DomainEventDb, Self::Error> {
            let event_db = DomainEventDb {
                id: Uuid::new_v4(),
                event_type: DomainEventTypeDb::Outbox,
                payload: serde_json::to_value(&_event).unwrap(),
                instance: Uuid::new_v4(),
                created_at: Utc::now(),
                processed_at: None,
            };
            Ok(event_db)
        }

        async fn get_unprocessed_outbox_events(
            &self,
            _tx_ctx: &mut Self::TxCtx,
        ) -> Result<Vec<DomainEventDb>, Self::Error> {
            Ok(vec![])
        }

        async fn mark_event_as_processed(
            &self,
            _event_id: Uuid,
            _tx_ctx: &mut Self::TxCtx,
        ) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    impl RefereeRepository for TestRepo {
        type Error = String;
        type TxCtx = ();

        async fn find_by_id(
            &self,
            id: RefereeId,
            _tx_ctx: &mut Self::TxCtx,
        ) -> Result<Option<Referee>, Self::Error> {
            Ok(self.data.borrow().get(&id).cloned())
        }

        async fn save(
            &self,
            referee: &Referee,
            _tx_ctx: &mut Self::TxCtx,
        ) -> Result<(), Self::Error> {
            let mut data = self.data.borrow_mut();
            data.insert(referee.id().clone(), referee.clone());
            Ok(())
        }

        async fn get_all(&self, _tx_ctx: &mut Self::TxCtx) -> Result<Vec<Referee>, Self::Error> {
            Ok(self.data.borrow().values().cloned().collect())
        }
    }

    #[tokio::test]
    async fn test_create_referee() {
        let repo = TestRepo::new();
        let domain_event_repo = TestDomainEventRepository {};

        let referee = create_referee("John Doe", "Club A", &repo, &domain_event_repo, &mut ())
            .await
            .unwrap();
        assert_eq!(referee.club(), "Club A");
        assert_eq!(referee.name(), "John Doe");

        let all_referees = repo.get_all(&mut ()).await.unwrap();
        assert_eq!(all_referees.len(), 1);
        assert_eq!(all_referees[0].club(), "Club A");
        assert_eq!(all_referees[0].name(), "John Doe");
    }

    #[tokio::test]
    async fn test_update_referee_club() {
        let repo = TestRepo::new();
        let domain_event_repo = TestDomainEventRepository {};

        let referee = create_referee("John Doe", "Club A", &repo, &domain_event_repo, &mut ())
            .await
            .unwrap();
        assert_eq!(referee.club(), "Club A");
        assert_eq!(referee.name(), "John Doe");

        update_referee_club(
            referee.id().into(),
            "Club B",
            &repo,
            &domain_event_repo,
            &mut (),
        )
        .await
        .unwrap();

        let all_referees = repo.get_all(&mut ()).await.unwrap();
        assert_eq!(all_referees.len(), 1);
        assert_eq!(all_referees[0].club(), "Club B");
        assert_eq!(all_referees[0].name(), "John Doe");
    }
}
