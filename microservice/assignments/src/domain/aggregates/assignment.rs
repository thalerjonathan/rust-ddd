use microservices_shared::domain_ids::{FixtureId, RefereeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignmentRefereeRole {
    First,
    Second,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignmentStatus {
    Committed,
    Staged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment {
    status: AssignmentStatus,
    referee_role: AssignmentRefereeRole,
    fixture_id: FixtureId,
    referee_id: RefereeId,
}

impl Assignment {
    pub fn staged(
        fixture_id: FixtureId,
        referee_id: RefereeId,
        referee_role: AssignmentRefereeRole,
    ) -> Self {
        Self {
            status: AssignmentStatus::Staged,
            fixture_id,
            referee_id,
            referee_role,
        }
    }

    pub fn new(
        fixture_id: FixtureId,
        referee_id: RefereeId,
        referee_role: AssignmentRefereeRole,
        status: AssignmentStatus,
    ) -> Self {
        Self {
            status,
            fixture_id,
            referee_id,
            referee_role,
        }
    }

    pub fn fixture_id(&self) -> FixtureId {
        self.fixture_id
    }

    pub fn referee_id(&self) -> RefereeId {
        self.referee_id
    }

    pub fn referee_role(&self) -> AssignmentRefereeRole {
        self.referee_role
    }

    pub fn status(&self) -> AssignmentStatus {
        self.status
    }

    pub fn is_staged(&self) -> bool {
        self.status == AssignmentStatus::Staged
    }

    pub fn is_committed(&self) -> bool {
        self.status == AssignmentStatus::Committed
    }

    pub fn change_referee_role(&mut self, referee_role: AssignmentRefereeRole) {
        self.referee_role = referee_role;
    }

    pub fn commit(&mut self) {
        if self.status == AssignmentStatus::Staged {
            self.status = AssignmentStatus::Committed;
        } else {
            panic!("Assignment already committed")
        }
    }
}
