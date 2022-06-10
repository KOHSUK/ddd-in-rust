use crate::domain::model::user::entity::User;

use super::ClubId;

pub struct ClubMembers {
    id: ClubId,
    owner: User,
    members: Vec<User>,
}

impl ClubMembers {
    pub fn new(id: ClubId, owner: User, members: Vec<User>) -> Self {
        Self { id, owner, members }
    }

    pub fn count_premium_members(&self, contains_owner: bool) -> usize {
        let count = self
            .members
            .iter()
            .filter(|m| m.get_is_premium().to_inner())
            .count();

        count
            + if contains_owner && self.owner.get_is_premium().to_inner() {
                1
            } else {
                0
            }
    }

    pub fn count_members(&self) -> usize {
        self.members.len()
    }
}
