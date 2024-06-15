use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Clone)]
pub struct AchievementMetadata {
    pub achievement_name: String,
    pub achievement_description: String
}
