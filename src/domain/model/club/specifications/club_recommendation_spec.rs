use crate::domain::model::club::entity::Club;

pub struct ClubRecommendationSpec;

impl ClubRecommendationSpec {
    pub fn new() -> Self {
        Self {}
    }

    pub fn is_satisfied_by(&self, club: &Club) -> bool {
        club.count_members() >= 3
        // TODO: ”設立１ヶ月以内”の条件も加える
    }
}
