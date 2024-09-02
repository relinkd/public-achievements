//! This module defines storable types and their implementations for use with stable structures.

use candid::{CandidType, Principal, Encode, Decode};
use serde::{Deserialize, Serialize};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{
    storable::Bound, DefaultMemoryImpl, Storable
};
use std::borrow::Cow;

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_VALUE_SIZE: u32 = 130;
const MAX_KEY_SIZE: u32 = 130;

/// Enum representing the status of an achievement.
pub enum AchievementStatusEnum {
    NotAllowed,
    Allowed
}

/// Metadata for an achievement.
#[derive(CandidType, Deserialize, Clone)]
pub struct AchievementMetadata {
    pub achievement_name: String,
    pub achievement_description: String
}

impl AchievementMetadata {
    /// Creates a default instance of `AchievementMetadata`.
    pub fn default() -> Self {
        Self {
            achievement_description: String::default(),
            achievement_name: String::default()
        }   
    }
}

impl Storable for AchievementMetadata {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}

impl AchievementStatusEnum {
    /// Converts the enum variant to a `u8` value.
    pub fn to_u8(&self) -> u8 {
        match self {
            AchievementStatusEnum::NotAllowed => 0,
            AchievementStatusEnum::Allowed => 1,
        }
    }

    /// Converts a `u8` value to the corresponding enum variant.
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => AchievementStatusEnum::NotAllowed,
            1 => AchievementStatusEnum::Allowed,
            _ => panic!("Invalid value for AchievementStatusEnum"),
        }
    }

    /// Converts a `u8` value to a string representation of the enum variant.
    pub fn to_string_from_u8(value: u8) -> String {
        match Self::from_u8(value) {
            AchievementStatusEnum::NotAllowed => String::from("not_allowed"),
            AchievementStatusEnum::Allowed => String::from("allowed"),
            _ => panic!("Invalid value for AchievementStatusEnum"),
        }
    }
}

/// A wrapper for `Principal` to make it storable.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, CandidType)]
pub struct PrincipalStorable(pub Principal);

/// Represents the status of an achievement.
#[derive(CandidType, Deserialize, Clone)]
pub struct AchievementStatus(pub u8);

impl Storable for PrincipalStorable {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        self.0.to_bytes()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self(Principal::from_bytes(bytes))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_KEY_SIZE,
        is_fixed_size: false,
    };
}

impl Storable for AchievementStatus {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        self.0.to_bytes()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self(u8::from_bytes(bytes))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_KEY_SIZE,
        is_fixed_size: false,
    };
}

/// Represents a signature.
#[derive(CandidType, Serialize, Debug, Deserialize)]
pub struct Signature(pub String);

impl Storable for Signature {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        self.0.to_bytes()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self(String::from_bytes(bytes))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_KEY_SIZE,
        is_fixed_size: false,
    };
}
