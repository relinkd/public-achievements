//! This module manages the state of the achievement system, including metadata, hashes, and achievement statuses.

use ic_cdk::{query, update};
use candid::Principal;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use ic_stable_structures::{
    DefaultMemoryImpl, StableBTreeMap, StableCell
};
use std::cell::RefCell;

use crate::storable::{
    PrincipalStorable, AchievementStatus, Memory, Signature, AchievementMetadata
};
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

/// Updates the metadata of the achievement canister.
///
/// # Arguments
///
/// * `metadata` - The new metadata for the achievement canister.
///
/// # Returns
///
/// * `Result<AchievementMetadata, String>` - The result of the update operation.
pub fn _update_canister_metadata(metadata: AchievementMetadata) -> Result<AchievementMetadata, String> {
    Ok(METADATA.with(|m| {
        let mut metadata_module = m.borrow_mut();
        metadata_module.set(metadata)
    }).unwrap_or_else(|err| {
        ic_cdk::trap(&format!("{:?}", err))
    }))
}

/// Updates the metadata of the achievement canister.
///
/// # Arguments
///
/// * `metadata` - The new metadata for the achievement canister.
///
/// # Returns
///
/// * `Result<AchievementMetadata, String>` - The result of the update operation.
#[update(name = "updateAchivementMetadata")]
pub fn update_achievement_metadata(metadata: AchievementMetadata) -> Result<AchievementMetadata, String> {
    if(!is_controller()) {
        return Err(String::from("Access denied"));
    }
    _update_canister_metadata(metadata)
}

/// Updates the hash for a principal.
///
/// # Arguments
///
/// * `principal` - The principal to update.
/// * `hash` - The new hash for the principal.
///
/// # Returns
///
/// * `Result<(), String>` - The result of the update operation.
pub fn update_principal_to_hash(principal: Principal, hash: Signature) -> Result<(), String> {
    PRINCIPAL_TO_HASH.with(|p| p.borrow_mut().insert(PrincipalStorable(principal), hash));

    Ok(())
}

/// Updates the achievement status for a principal.
///
/// # Arguments
///
/// * `principal` - The principal to update.
/// * `achievement_status` - The new achievement status for the principal.
///
/// # Returns
///
/// * `Result<(), String>` - The result of the update operation.
pub fn update_principal_to_achievement_status(principal: Principal, achievement_status: AchievementStatus) -> Result<(), String> {
    PRINCIPAL_TO_ACHIEVEMENT_STATUS.with(|p| p.borrow_mut().insert(PrincipalStorable(principal), achievement_status));

    Ok(())
}

/// Retrieves the metadata of the achievement canister.
///
/// # Returns
///
/// * `AchievementMetadata` - The current metadata of the achievement canister.
#[query(name = "getAchievementMetadata")]
pub fn get_achievement_metadata() -> AchievementMetadata {
    METADATA.with(|m| {
        let metadata = m.borrow();
        metadata.get().clone()
    })
}

/// Retrieves the hash for a principal.
///
/// # Arguments
///
/// * `principal` - The principal to retrieve the hash for.
///
/// # Returns
///
/// * `Result<Signature, String>` - The hash for the principal.
#[query(name = "getPrincipalToHashValue")]
pub fn get_principal_to_hash_value(principal: Principal) -> Result<Signature, String> {
    if let Some(hash) = PRINCIPAL_TO_HASH.with(|p| p.borrow().get(&PrincipalStorable(principal))) {
        Ok(hash)
    } else {
        Err(String::from("Hash not found"))
    }
}

/// Retrieves the achievement status for a principal.
///
/// # Arguments
///
/// * `principal` - The principal to retrieve the achievement status for.
///
/// # Returns
///
/// * `Result<u8, String>` - The achievement status for the principal.
#[query(name = "getPrincipalToAchievementStatusValue")]
pub fn get_principal_to_achievement_status_value(principal: Principal) -> Result<u8, String> {
    if let Some(achievement_status) = PRINCIPAL_TO_ACHIEVEMENT_STATUS.with(|p| p.borrow().get(&PrincipalStorable(principal))) {
        Ok(achievement_status.0)
    } else {
        Err(String::from("Achievement status not found"))
    }
}

