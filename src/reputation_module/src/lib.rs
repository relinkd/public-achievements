pub mod storable;

use candid::Principal;
use ic_cdk::{query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use ic_stable_structures::{
    DefaultMemoryImpl, StableBTreeMap, StableVec
};
use std::cell::RefCell;

use storable::{Memory, CanisterPermission, CanisterPrincipal};

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static ACHIEVEMENT_CANISTER_TO_BOOL: RefCell<StableBTreeMap<CanisterPrincipal, CanisterPermission, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );

    static ACHIEVEMENTS: RefCell<StableVec<CanisterPrincipal, Memory>> = RefCell::new(
        StableVec::init(
            MEMORY_MANAGER.with(|a| a.borrow().get(MemoryId::new(1))),
        ).unwrap()
    );

    static COUNTER: RefCell<u64> = RefCell::new(0_u64);
}

#[query(name = "caller")]
fn caller() -> Principal {
    let id = ic_cdk::api::caller();

    return id;
}

#[query(name = "isController")]
fn is_controller() -> bool {
    let id = ic_cdk::api::caller();
    let is_controller = ic_cdk::api::is_controller(&id);

    return is_controller;
}

#[update(name = "changePermissionCanister")]
fn change_permission_canister(canister: Principal, permission: bool) -> Result<String, String> {
    let id = ic_cdk::api::caller();
    let is_controller = ic_cdk::api::is_controller(&id);

    if is_controller {
        ACHIEVEMENT_CANISTER_TO_BOOL.with(|p| p.borrow_mut().insert(CanisterPrincipal(canister), CanisterPermission(permission)));
        Ok(String::from("Granted permissions to canister"))
    } else {
        Err(String::from("Access denied"))
    }
}

#[query(name = "isCanisterAllowed")]
fn is_canister_allowed(canister: Principal) -> Result<CanisterPermission, String> {
    if let Some(permission) = ACHIEVEMENT_CANISTER_TO_BOOL.with(|p| p.borrow().get(&CanisterPrincipal(canister))) {
        Ok(permission)
    } else {
        Err(String::from("Canister not found"))
    }
}

#[query(name = "issueAchievementToIdentityWallet")]
async fn issue_achievement_to_identity_wallet(achievement: Principal) -> Result<String, String> {
    let canister_permission = is_canister_allowed(achievement)?;

    if !canister_permission.0 {
        return Err(String::from("Achievement not allowed"));
    }

    let caller = ic_cdk::api::caller();
    let status: (u8, ) = ic_cdk::call(achievement, "getPrincipalToAchievementStatusValue", (caller,)).await.unwrap();

    if status.0 == 1_u8 {
        // your logic to issue achievement
        Ok(String::from("Achievement issued"))
    } else {
        Err(String::from("You`re not allowed"))
    }
}

ic_cdk::export_candid!();