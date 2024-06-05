pub mod ecdsa;

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use ic_cdk::{query, update};
use std::convert::TryFrom;
use std::str::FromStr;

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{
    storable::Bound, DefaultMemoryImpl, StableBTreeMap, Storable,
};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_VALUE_SIZE: u32 = 100;
const MAX_KEY_SIZE: u32 = 100;

enum AchievementStatusEnum {
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
struct IdentityWallet(Principal);

#[derive(CandidType, Deserialize, Clone)]
struct AchievementStatus(u8);

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


thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static ACHIEVEMENT_STATUS: RefCell<StableBTreeMap<IdentityWallet, AchievementStatus, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
}





ic_cdk::export_candid!();