pub mod ecdsa;
pub mod storable;

use ic_cdk::{query, update};

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use ic_stable_structures::{
    DefaultMemoryImpl, StableBTreeMap
};
use std::cell::RefCell;

use storable::{IdentityWallet, AchievementStatus, Memory};


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