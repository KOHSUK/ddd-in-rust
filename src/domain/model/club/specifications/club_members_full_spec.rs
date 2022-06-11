use crate::domain::model::club::entity::ClubMembers;

pub struct ClubMembersFullSpec;

impl ClubMembersFullSpec {
    pub fn new() -> Self {
        Self {}
    }

    pub fn is_satisfied_by(&self, members: ClubMembers) -> bool {
        let premium_num = members.count_premium_members(false);
        let limit = if premium_num < 1 { 3 } else { 50 };
        members.count_members() >= limit
    }
}
