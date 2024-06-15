
use ic_cdk::{query, update};
use candid::{Principal};

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use ic_stable_structures::{
    DefaultMemoryImpl, StableBTreeMap, StableVec, StableCell
};
use std::cell::RefCell;

use crate::storable::{PrincipalStorable, AchievementStatus, Memory, Signature, AchievementStatusEnum, AchievementMetadata};
use crate::ecdsa::{sign, public_key, verify, build_principals_message};
use crate::access::is_controller;


thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub static PRINCIPAL_TO_ACHIEVEMENT_STATUS: RefCell<StableBTreeMap<PrincipalStorable, AchievementStatus, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );

    pub static METADATA: RefCell<StableCell<AchievementMetadata, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))), AchievementMetadata::default(),
        ).unwrap()
    );

    pub static PRINCIPAL_TO_HASH: RefCell<StableBTreeMap<PrincipalStorable, Signature, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        )
    );
}

pub fn _update_canister_metadata(metadata: AchievementMetadata) -> Result<AchievementMetadata, String> {
    Ok(METADATA.with(|m| {
        let mut metadata_module = m.borrow_mut();
        metadata_module.set(metadata)
    }).unwrap_or_else(|err| {
        ic_cdk::trap(&format!("{:?}", err))
    }))
}

#[update(name = "updateAchivementMetadata")]
pub fn update_achievement_metadata(metadata: AchievementMetadata) -> Result<AchievementMetadata, String> {
    if(!is_controller()) {
        return Err(String::from("Access denied"));
    }
    _update_canister_metadata(metadata)
}

pub fn update_principal_to_hash(principal: Principal, hash: Signature) -> Result<(), String> {
    PRINCIPAL_TO_HASH.with(|p| p.borrow_mut().insert(PrincipalStorable(principal), hash));

    Ok(())
}

pub fn update_principal_to_achievement_status(principal: Principal, achievement_status: AchievementStatus) -> Result<(), String> {
    PRINCIPAL_TO_ACHIEVEMENT_STATUS.with(|p| p.borrow_mut().insert(PrincipalStorable(principal), achievement_status));

    Ok(())
}


#[query(name = "getAchievementMetadata")]
pub fn get_achievement_metadata() -> AchievementMetadata {
    METADATA.with(|m| {
        let metadata = m.borrow();
        metadata.get().clone()
    })
}

#[query(name = "getPrincipalToHashValue")]
pub fn get_principal_to_hash_value(principal: Principal) -> Result<Signature, String> {
    if let Some(hash) = PRINCIPAL_TO_HASH.with(|p| p.borrow().get(&PrincipalStorable(principal))) {
        Ok(hash)
    } else {
        Err(String::from("Hash not found"))
    }
}

#[query(name = "getPrincipalToAchievementStatusValue")]
pub fn get_principal_to_achievement_status_value(principal: Principal) -> Result<u8, String> {
    if let Some(achievement_status) = PRINCIPAL_TO_ACHIEVEMENT_STATUS.with(|p| p.borrow().get(&PrincipalStorable(principal))) {
        Ok(achievement_status.0)
    } else {
        Err(String::from("Achievement status not found"))
    }
}

