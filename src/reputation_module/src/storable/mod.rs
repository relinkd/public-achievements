use candid::{Principal, CandidType, Deserialize, Encode, Decode};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{
    storable::Bound, DefaultMemoryImpl, Storable,
};
use std::borrow::Cow;

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

pub const MAX_VALUE_SIZE: u32 = 100;
pub const MAX_KEY_SIZE: u32 = 100;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CanisterPrincipal(pub Principal);

#[derive(CandidType, Deserialize, Clone)]
pub struct CanisterPermission(pub bool);

#[derive(CandidType, Deserialize, Clone)]
pub struct ReputationModuleMetadata {
    pub achievement_canister: Principal,
    pub issuer_name: String,
    pub issuer_description: String,
    pub total_issued: u128
}

impl Storable for CanisterPrincipal {
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

impl ReputationModuleMetadata {
    pub fn default() -> Self {
        Self {
            achievement_canister: Principal::anonymous(),
            issuer_description: String::default(),
            issuer_name: String::default(),
            total_issued: u128::default()
        }   
    }
}

impl Storable for ReputationModuleMetadata {
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
impl Storable for CanisterPermission {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        self.0.to_bytes()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self(bool::from_bytes(bytes))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_KEY_SIZE,
        is_fixed_size: false,
    };
}