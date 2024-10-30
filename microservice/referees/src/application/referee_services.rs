use log::debug;
use microservices_shared::{
    domain_events::{DomainEvent, DomainEventPublisher},
    domain_ids::RefereeId,
};

use crate::domain::{aggregates::referee::Referee, repositories::referee_repo::RefereeRepository};

pub async fn create_referee<TxCtx>(
    name: &str,
    club: &str,
    repo: &impl RefereeRepository<TxCtx = TxCtx, Error = String>,
    domain_event_publisher: &Box<dyn DomainEventPublisher + Send + Sync>,
    tx_ctx: &mut TxCtx,
) -> Result<Referee, String> {
    let referee = Referee::new(name, club);

    repo.save(&referee, tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

    domain_event_publisher
        .publish_domain_event(DomainEvent::RefereeCreated {
            referee_id: referee.id().clone(),
        })
        .await
        .map_err(|e| e.to_string())?;

    debug!("Referee created: {:?}", referee);

    Ok(referee)
}

pub async fn update_referee_club<TxCtx>(
    referee_id: RefereeId,
    club: &str,
    repo: &impl RefereeRepository<TxCtx = TxCtx, Error = String>,
    domain_event_publisher: &Box<dyn DomainEventPublisher + Send + Sync>,
    tx_ctx: &mut TxCtx,
) -> Result<(), String> {
    let referee = repo.find_by_id(referee_id, tx_ctx).await?;
    let mut referee = referee.ok_or("Referee not found")?;
    referee.change_club(club);

    repo.save(&referee, tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

    domain_event_publisher
        .publish_domain_event(DomainEvent::RefereeClubChanged {
            referee_id: referee.id().clone(),
            club_name: club.to_string(),
        })
        .await
        .map_err(|e| e.to_string())?;

    // NOTE: the cache entry is invalidated by the domain event listener of a Referee service instance
    // to keep the application layer free from caching logic

    debug!("Referee updated: {:?}", referee);

    Ok(())
}

#[cfg(test)]
mod tests {
    use axum::async_trait;
    use microservices_shared::{
        domain_events::{DomainEvent, DomainEventPublisher},
        domain_ids::RefereeId,
    };

    use crate::{
        application::referee_services::{create_referee, update_referee_club},
        domain::{aggregates::referee::Referee, repositories::referee_repo::RefereeRepository},
    };
    use std::cell::RefCell;
    use std::collections::HashMap;

    struct TestDomainEventPublisher {}

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

    #[async_trait]
    impl DomainEventPublisher for TestDomainEventPublisher {
        async fn publish_domain_event(&self, _event: DomainEvent) -> Result<(), String> {
            Ok(())
        }

        async fn begin_transaction(&self) -> Result<(), String> {
            Ok(())
        }

        async fn commit_transaction(&self) -> Result<(), String> {
            Ok(())
        }

        async fn rollback(&self) -> Result<(), String> {
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
        let domain_event_publisher: Box<dyn DomainEventPublisher + Send + Sync> =
            Box::new(TestDomainEventPublisher {});

        let referee = create_referee(
            "John Doe",
            "Club A",
            &repo,
            &domain_event_publisher,
            &mut (),
        )
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
        let domain_event_publisher: Box<dyn DomainEventPublisher + Send + Sync> =
            Box::new(TestDomainEventPublisher {});

        let referee = create_referee(
            "John Doe",
            "Club A",
            &repo,
            &domain_event_publisher,
            &mut (),
        )
        .await
        .unwrap();
        assert_eq!(referee.club(), "Club A");
        assert_eq!(referee.name(), "John Doe");

        update_referee_club(
            referee.id().into(),
            "Club B",
            &repo,
            &domain_event_publisher,
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
