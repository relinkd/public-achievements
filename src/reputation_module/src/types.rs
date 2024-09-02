//! This module defines the types used in the reputation module.

use candid::CandidType;
use serde::Deserialize;

/// Metadata for an achievement.
#[derive(CandidType, Deserialize, Clone)]
pub struct AchievementMetadata {
    pub achievement_name: String,
    pub achievement_description: String
}
