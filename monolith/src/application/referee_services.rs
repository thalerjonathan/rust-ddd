use log::debug;

use crate::domain::{
    aggregates::referee::{Referee, RefereeId},
    repositories::referee_repo::RefereeRepository,
};

pub async fn create_referee<TxCtx>(
    name: &str,
    club: &str,
    repo: &impl RefereeRepository<TxCtx = TxCtx, Error = String>,
    tx_ctx: &mut TxCtx,
) -> Result<Referee, String> {
    let referee = Referee::new(name, club);

    repo.save(&referee, tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

    debug!("Referee created: {:?}", referee);

    Ok(referee)
}

pub async fn update_referee_club<TxCtx>(
    referee_id: RefereeId,
    club: &str,
    repo: &impl RefereeRepository<TxCtx = TxCtx, Error = String>,
    tx_ctx: &mut TxCtx,
) -> Result<(), String> {
    let referee = repo.find_by_id(referee_id, tx_ctx).await?;
    let mut referee = referee.ok_or("Referee not found")?;
    referee.change_club(club);

    repo.save(&referee, tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

    debug!("Referee updated: {:?}", referee);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::HashMap;

    use crate::{
        application::referee_services::{create_referee, update_referee_club},
        domain::{
            aggregates::referee::{Referee, RefereeId},
            repositories::referee_repo::RefereeRepository,
        },
    };

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

        let referee = create_referee("John Doe", "Club A", &repo, &mut ())
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

        let referee = create_referee("John Doe", "Club A", &repo, &mut ())
            .await
            .unwrap();
        assert_eq!(referee.club(), "Club A");
        assert_eq!(referee.name(), "John Doe");

        update_referee_club(referee.id().into(), "Club B", &repo, &mut ())
            .await
            .unwrap();

        let all_referees = repo.get_all(&mut ()).await.unwrap();
        assert_eq!(all_referees.len(), 1);
        assert_eq!(all_referees[0].club(), "Club B");
        assert_eq!(all_referees[0].name(), "John Doe");
    }
}
