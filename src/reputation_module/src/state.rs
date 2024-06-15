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

    static PRINCIPAL_PLUS_ACHIEVEMENT_TO_IS_ISSUED: RefCell<StableBTreeMap<PrincipalSum, bool, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))),
        )
    );

}

pub fn _change_principal_achievement_sum_status_to_issued(identity_wallet: Principal, achievement: Principal) -> Result<(), String> {
    let principal_sum = build_principal_sum(identity_wallet, achievement);

    PRINCIPAL_PLUS_ACHIEVEMENT_TO_IS_ISSUED.with(|p| p.borrow_mut().insert(PrincipalSum(principal_sum), true));

    Ok(())
}

#[query(name = "getPrincipalAchievementSumStatus")]
pub fn get_principal_achievement_sum_status(identity_wallet: Principal, achievement: Principal) -> bool {
    let principal_sum = build_principal_sum(identity_wallet, achievement);

    if let Some(issued_status) = PRINCIPAL_PLUS_ACHIEVEMENT_TO_IS_ISSUED.with(|p| p.borrow().get(&PrincipalSum(principal_sum))) {
        issued_status
    } else {
        false
    }
}

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

#[query(name = "isCanisterAllowed")]
pub fn is_canister_allowed(canister: Principal) -> Result<CanisterPermission, String> {
    if let Some(permission) = ACHIEVEMENT_CANISTER_TO_BOOL.with(|p| p.borrow().get(&StorablePrincipal(canister))) {
        Ok(permission)
    } else {
        Err(String::from("Canister not found"))
    }
}

pub fn increment_total_issued() -> Result<(), String> {
    let mut reputation_module_metadata = get_reputation_module_metadata();
    reputation_module_metadata.total_issued += 1;

    _update_canister_metadata(reputation_module_metadata)?;

    Ok(())
}

pub fn _update_canister_metadata(metadata: ReputationModuleMetadata) -> Result<ReputationModuleMetadata, String> {
    Ok(METADATA.with(|m| {
        let mut metadata_module = m.borrow_mut();
        metadata_module.set(metadata)
    }).unwrap_or_else(|err| {
        ic_cdk::trap(&format!("{:?}", err))
    }))
}

#[update(name = "updateReputationModuleMetadata")]
pub fn update_reputation_canister_metadata(metadata: ReputationModuleMetadata) -> Result<ReputationModuleMetadata, String> {
    if(!is_controller()) {
        return Err(String::from("Access denied"));
    }
    _update_canister_metadata(metadata)
}

#[query(name = "getReputationModuleMetadata")]
pub fn get_reputation_module_metadata() -> ReputationModuleMetadata {
    METADATA.with(|m| {
        let metadata = m.borrow();
        metadata.get().clone()
    })
}

#[query(name = "getAchievementMetadata")]
pub async fn get_achievement_metadata(achievement: Principal) -> Result<AchievementMetadata, String> {
    let achievement_metadata: (AchievementMetadata, ) = ic_cdk::call(achievement, "getAchievementMetadata", ()).await.unwrap();

    Ok(achievement_metadata.0)
}