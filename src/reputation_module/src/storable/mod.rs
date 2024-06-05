use candid::{Principal, CandidType, Deserialize};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{
    storable::Bound, DefaultMemoryImpl, Storable,
};

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

pub const MAX_VALUE_SIZE: u32 = 100;
pub const MAX_KEY_SIZE: u32 = 100;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CanisterPrincipal(pub Principal);

#[derive(CandidType, Deserialize, Clone)]
pub struct CanisterPermission(pub bool);

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