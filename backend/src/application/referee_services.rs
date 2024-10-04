use log::debug;
use uuid::Uuid;

use crate::domain::{
    aggregates::referee::{Referee, RefereeId},
    repositories::referee_repo::RefereeRepository,
};

pub async fn create_referee(
    name: &str,
    club: &str,
    repo: &impl RefereeRepository<Error = String>,
) -> Result<Referee, String> {
    let referee = Referee::new(name, club);

    repo.save(&referee).await.map_err(|e| e.to_string())?;

    debug!("Referee created: {:?}", referee);

    Ok(referee)
}

pub async fn update_referee_club(
    referee_id: &Uuid,
    club: &str,
    repo: &impl RefereeRepository<Error = String>,
) -> Result<(), String> {
    let referee_id = RefereeId::from(*referee_id);

    let referee = repo.find_by_id(&referee_id).await?;
    let mut referee = referee.ok_or("Referee not found")?;
    referee.change_club(club);

    // TODO: the ultimate goal is to make this disappear by some kind of unit-of-work implementation
    repo.save(&referee).await.map_err(|e| e.to_string())?;

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

        async fn find_by_id(&self, id: &RefereeId) -> Result<Option<Referee>, Self::Error> {
            Ok(self.data.borrow().get(id).cloned())
        }

        async fn save(&self, referee: &Referee) -> Result<(), Self::Error> {
            let mut data = self.data.borrow_mut();
            data.insert(referee.id().clone(), referee.clone());
            Ok(())
        }

        async fn get_all(&self) -> Result<Vec<Referee>, Self::Error> {
            Ok(self.data.borrow().values().cloned().collect())
        }
    }

    #[tokio::test]
    async fn test_create_referee() {
        let repo = TestRepo::new();

        let referee = create_referee("John Doe", "Club A", &repo).await.unwrap();
        assert_eq!(referee.club(), "Club A");
        assert_eq!(referee.name(), "John Doe");

        let all_referees = repo.get_all().await.unwrap();
        assert_eq!(all_referees.len(), 1);
        assert_eq!(all_referees[0].club(), "Club A");
        assert_eq!(all_referees[0].name(), "John Doe");
    }

    #[tokio::test]
    async fn test_update_referee_club() {
        let repo = TestRepo::new();

        let referee = create_referee("John Doe", "Club A", &repo).await.unwrap();
        assert_eq!(referee.club(), "Club A");
        assert_eq!(referee.name(), "John Doe");

        update_referee_club(&referee.id().0, "Club B", &repo)
            .await
            .unwrap();

        let all_referees = repo.get_all().await.unwrap();
        assert_eq!(all_referees.len(), 1);
        assert_eq!(all_referees[0].club(), "Club B");
        assert_eq!(all_referees[0].name(), "John Doe");
    }
}
