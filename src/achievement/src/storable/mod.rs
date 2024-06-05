use std::convert::TryFrom;
use std::str::FromStr;

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{
    storable::Bound, DefaultMemoryImpl, StableBTreeMap, Storable,
};
use std::{borrow::Cow, cell::RefCell};

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_VALUE_SIZE: u32 = 100;
const MAX_KEY_SIZE: u32 = 100;

pub enum AchievementStatusEnum {
    NotAllowed,
    Allowed
}

impl AchievementStatusEnum {
    fn to_u8(&self) -> u8 {
        match self {
            AchievementStatusEnum::NotAllowed => 0,
            AchievementStatusEnum::Allowed => 1,
        }
    }

    fn from_u8(value: u8) -> Self {
        match value {
            0 => AchievementStatusEnum::NotAllowed,
            1 => AchievementStatusEnum::Allowed,
            _ => panic!("Invalid value for AchievementStatusEnum"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct IdentityWallet(pub Principal);

#[derive(CandidType, Deserialize, Clone)]
pub struct AchievementStatus(pub u8);

impl Storable for IdentityWallet {
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
