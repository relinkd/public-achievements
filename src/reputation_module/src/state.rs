//! This module manages the state of the reputation module, including metadata, achievements, and permissions.

use ic_stable_structures::{
    DefaultMemoryImpl, StableBTreeMap, StableVec, StableCell
};
use candid::Principal;
use ic_cdk::{query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use std::cell::RefCell;

use crate::utils::build_principal_sum;
use crate::types::AchievementMetadata;
use crate::access::is_controller;
use crate::storable::{
    Memory, CanisterPermission, StorablePrincipal, ReputationModuleMetadata, PrincipalSum
};
use crate::Standard;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static ACHIEVEMENT_CANISTER_TO_BOOL: RefCell<StableBTreeMap<StorablePrincipal, CanisterPermission, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );

    static METADATA: RefCell<StableCell<ReputationModuleMetadata, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))), ReputationModuleMetadata::default(),
        ).unwrap()
    );

    static ACHIEVEMENTS: RefCell<StableVec<StorablePrincipal, Memory>> = RefCell::new(
        StableVec::init(
            MEMORY_MANAGER.with(|a| a.borrow().get(MemoryId::new(1))),
        ).unwrap()
    );

    static SUPPORTED_STANDARDS: RefCell<StableVec<Standard, Memory>> = RefCell::new(
        StableVec::init(
            MEMORY_MANAGER.with(|a| a.borrow().get(MemoryId::new(4))),
        ).unwrap()
    );

    static PRINCIPAL_PLUS_ACHIEVEMENT_TO_IS_ISSUED: RefCell<StableBTreeMap<PrincipalSum, bool, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))),
        )
    );

}

/// Changes the status of a principal's achievement to issued.
///
/// This function updates the status of a principal's achievement to indicate that it has been issued.
///
/// # Arguments
///
/// * `identity_wallet` - The principal of the identity wallet.
/// * `achievement` - The principal of the achievement canister.
///
/// # Returns
///
/// * `Result<(), String>` - The result of the update operation.
pub fn _change_principal_achievement_sum_status_to_issued(identity_wallet: Principal, achievement: Principal) -> Result<(), String> {
    let principal_sum = build_principal_sum(identity_wallet, achievement);

    PRINCIPAL_PLUS_ACHIEVEMENT_TO_IS_ISSUED.with(|p| p.borrow_mut().insert(PrincipalSum(principal_sum), true));

    Ok(())
}

/// Retrieves the status of a principal's achievement.
///
/// This function checks if a principal's achievement has been issued.
///
/// # Arguments
///
/// * `identity_wallet` - The principal of the identity wallet.
/// * `achievement` - The principal of the achievement canister.
///
/// # Returns
///
/// * `bool` - `true` if the achievement has been issued, `false` otherwise.
#[query(name = "getPrincipalAchievementSumStatus")]
pub fn get_principal_achievement_sum_status(identity_wallet: Principal, achievement: Principal) -> bool {
    let principal_sum = build_principal_sum(identity_wallet, achievement);

    if let Some(issued_status) = PRINCIPAL_PLUS_ACHIEVEMENT_TO_IS_ISSUED.with(|p| p.borrow().get(&PrincipalSum(principal_sum))) {
        issued_status
    } else {
        false
    }
}

/// Sets the supported standards for the reputation module.
///
/// This function updates the list of supported standards for the reputation module.
///
/// # Arguments
///
/// * `standards` - A vector of supported standards.
///
/// # Returns
///
/// * `Result<(), String>` - The result of the update operation.
#[update(name = "setSupportedStandards")]
pub fn set_supported_standards(standards: Vec<Standard>) -> Result<(), String> {
    let id = ic_cdk::api::caller();
    let is_controller = ic_cdk::api::is_controller(&id);

    if !is_controller {
        return Err(String::from("Access denied"))
    }

    SUPPORTED_STANDARDS.with(|p| {
        let b_p = p.borrow_mut();

        for (_, _) in b_p.iter().enumerate() {
            b_p.pop();
        }
        for (_, e) in standards.iter().enumerate() {
            b_p.push(e);
        }
    });
    Ok(())
}

/// Retrieves the supported standards for the reputation module.
///
/// This function returns the list of supported standards for the reputation module.
///
/// # Returns
///
/// * `Vec<Standard>` - A vector of supported standards.
#[query(name = "getSupportedStandards")]
pub fn get_supported_standards() -> Vec<Standard> {
    SUPPORTED_STANDARDS.with(|p| {
        let mut standards: Vec<Standard> = vec![];
        for (_, e) in p.borrow().iter().enumerate() {
            standards.push(e)
        }
        standards
    })
}

/// Changes the permission of a canister.
///
/// This function updates the permission of a specified canister.
///
/// # Arguments
///
/// * `canister` - The principal of the canister.
/// * `permission` - The new permission status.
///
/// # Returns
///
/// * `Result<String, String>` - The result of the update operation.
#[update(name = "changePermissionCanister")]
pub fn change_permission_canister(canister: Principal, permission: bool) -> Result<String, String> {
    let id = ic_cdk::api::caller();
    let is_controller = ic_cdk::api::is_controller(&id);

    if is_controller {
        ACHIEVEMENT_CANISTER_TO_BOOL.with(|p| p.borrow_mut().insert(StorablePrincipal(canister), CanisterPermission(permission)));
        Ok(String::from("Granted permissions to canister"))
    } else {
        Err(String::from("Access denied"))
    }
}

/// Checks if a canister is allowed.
///
/// This function checks if a specified canister has the required permissions.
///
/// # Arguments
///
/// * `canister` - The principal of the canister.
///
/// # Returns
///
/// * `Result<CanisterPermission, String>` - The permission status of the canister.
#[query(name = "isCanisterAllowed")]
pub fn is_canister_allowed(canister: Principal) -> Result<CanisterPermission, String> {
    if let Some(permission) = ACHIEVEMENT_CANISTER_TO_BOOL.with(|p| p.borrow().get(&StorablePrincipal(canister))) {
        Ok(permission)
    } else {
        Err(String::from("Canister not found"))
    }
}

/// Increments the total number of issued achievements.
///
/// This function increments the total count of issued achievements in the reputation module.
///
/// # Returns
///
/// * `Result<(), String>` - The result of the increment operation.
pub fn increment_total_issued() -> Result<(), String> {
    let mut reputation_module_metadata = get_reputation_module_metadata();
    reputation_module_metadata.total_issued += 1;

    _update_canister_metadata(reputation_module_metadata)?;

    Ok(())
}

/// Updates the metadata of the reputation module.
///
/// This function updates the metadata of the reputation module with the provided metadata.
///
/// # Arguments
///
/// * `metadata` - The new metadata for the reputation module.
///
/// # Returns
///
/// * `Result<ReputationModuleMetadata, String>` - The result of the update operation.
pub fn _update_canister_metadata(metadata: ReputationModuleMetadata) -> Result<ReputationModuleMetadata, String> {
    Ok(METADATA.with(|m| {
        let mut metadata_module = m.borrow_mut();
        metadata_module.set(metadata)
    }).unwrap_or_else(|err| {
        ic_cdk::trap(&format!("{:?}", err))
    }))
}

/// Updates the metadata of the reputation module.
///
/// This function updates the metadata of the reputation module with the provided metadata.
///
/// # Arguments
///
/// * `metadata` - The new metadata for the reputation module.
///
/// # Returns
///
/// * `Result<ReputationModuleMetadata, String>` - The result of the update operation.
#[update(name = "updateReputationModuleMetadata")]
pub fn update_reputation_canister_metadata(metadata: ReputationModuleMetadata) -> Result<ReputationModuleMetadata, String> {
    if(!is_controller()) {
        return Err(String::from("Access denied"));
    }
    _update_canister_metadata(metadata)
}

/// Retrieves the metadata of the reputation module.
///
/// This function returns the current metadata of the reputation module.
///
/// # Returns
///
/// * `ReputationModuleMetadata` - The current metadata of the reputation module.
#[query(name = "getReputationModuleMetadata")]
pub fn get_reputation_module_metadata() -> ReputationModuleMetadata {
    METADATA.with(|m| {
        let metadata = m.borrow();
        metadata.get().clone()
    })
}

/// Retrieves the metadata of an achievement.
///
/// This function returns the metadata of a specified achievement.
///
/// # Arguments
///
/// * `achievement` - The principal of the achievement canister.
///
/// # Returns
///
/// * `Result<AchievementMetadata, String>` - The metadata of the achievement.
#[query(name = "getAchievementMetadata")]
pub async fn get_achievement_metadata(achievement: Principal) -> Result<AchievementMetadata, String> {
    let achievement_metadata: (AchievementMetadata, ) = ic_cdk::call(achievement, "getAchievementMetadata", ()).await.unwrap();

    Ok(achievement_metadata.0)
}