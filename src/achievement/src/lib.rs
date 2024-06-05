pub mod ecdsa;
pub mod storable;

use ic_cdk::{query, update};
use candid::{Principal};

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use ic_stable_structures::{
    DefaultMemoryImpl, StableBTreeMap
};
use std::cell::RefCell;

use storable::{PrincipalStorable, AchievementStatus, Memory, Signature};


thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static ACHIEVEMENT_STATUS: RefCell<StableBTreeMap<PrincipalStorable, AchievementStatus, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );

    static HASH_FROM_PRINCIPAL_TO_IDENTITY_WALLET: RefCell<StableBTreeMap<PrincipalStorable, Signature, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        )
    );
}

#[query(name = "checkAchievementEligibility")]
fn check_achievement_eligibility(principal: Principal, blob: Vec<u8>) -> Result<bool, String> {

    // Your conditions for achievement

    Ok(true)
}

ic_cdk::export_candid!();