pub mod storable;
pub mod icrc_7;
pub mod types;

use candid::{Principal};
use ic_cdk::{query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use ic_stable_structures::{
    DefaultMemoryImpl, StableBTreeMap, StableVec, StableCell
};
use icrc_ledger_types::icrc1::account::Account;
use std::cell::RefCell;

use storable::{Memory, CanisterPermission, CanisterPrincipal, ReputationModuleMetadata};

use icrc_7::types::{MintArg, MintResult};
use types::AchievementMetadata;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static ACHIEVEMENT_CANISTER_TO_BOOL: RefCell<StableBTreeMap<CanisterPrincipal, CanisterPermission, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );

    static METADATA: RefCell<StableCell<ReputationModuleMetadata, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))), ReputationModuleMetadata::default(),
        ).unwrap()
    );

    static ACHIEVEMENTS: RefCell<StableVec<CanisterPrincipal, Memory>> = RefCell::new(
        StableVec::init(
            MEMORY_MANAGER.with(|a| a.borrow().get(MemoryId::new(1))),
        ).unwrap()
    );
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

async fn issue_achievement(principal: Principal, achievement_metadata: AchievementMetadata) -> Result<MintResult, String> {
    let reputation_metadata = METADATA.with(|m| {
        let metadata = m.borrow();
        metadata.get().clone()
    });

    let mint_result: (MintResult, ) = ic_cdk::call(reputation_metadata.achievement_collection, "icrc7_mint", (MintArg {
        from_subaccount: None,
        token_id: reputation_metadata.total_issued + 1,
        token_logo: None,
        token_name: Some(achievement_metadata.achievement_name),
        memo: None,
        token_description: Some(achievement_metadata.achievement_description),
        to: Account {
            owner: principal,
            subaccount: None
        }
    },)).await.unwrap();

    increment_total_issued()?;

    Ok(mint_result.0)
}

fn increment_total_issued() -> Result<(), String> {
    let mut reputation_module_metadata = get_reputation_module_metadata();
    reputation_module_metadata.total_issued += 1;

    _update_canister_metadata(reputation_module_metadata)?;

    Ok(())
}

fn _update_canister_metadata(metadata: ReputationModuleMetadata) -> Result<ReputationModuleMetadata, String> {
    Ok(METADATA.with(|m| {
        let mut metadata_module = m.borrow_mut();
        metadata_module.set(metadata)
    }).unwrap_or_else(|err| {
        ic_cdk::trap(&format!("{:?}", err))
    }))
}

#[update(name = "updateReputationModuleMetadata")]
fn update_reputation_canister_metadata(metadata: ReputationModuleMetadata) -> Result<ReputationModuleMetadata, String> {
    if(!is_controller()) {
        return Err(String::from("Access denied"));
    }
    _update_canister_metadata(metadata)
}

#[query(name = "getReputationModuleMetadata")]
fn get_reputation_module_metadata() -> ReputationModuleMetadata {
    METADATA.with(|m| {
        let metadata = m.borrow();
        metadata.get().clone()
    })
}

#[query(name = "getAchievementMetadata")]
async fn get_achievement_metadata(achievement: Principal) -> Result<AchievementMetadata, String> {
    let achievement_metadata: (AchievementMetadata, ) = ic_cdk::call(achievement, "getReputationModuleMetadata", ()).await.unwrap();

    Ok(achievement_metadata.0)
}

#[update(name = "issueAchievementToIdentityWallet")]
async fn issue_achievement_to_identity_wallet(achievement: Principal) -> Result<String, String> {
    let canister_permission = is_canister_allowed(achievement)?;

    if !canister_permission.0 {
        return Err(String::from("Achievement not allowed"));
    }

    let caller = ic_cdk::api::caller();
    let status: (Result<u8, String>, ) = ic_cdk::call(achievement, "getPrincipalToAchievementStatusValue", (caller,)).await.unwrap();
    let achievement_metadata = get_achievement_metadata(achievement).await.unwrap();

    if status.0? == 1_u8 {
        Ok(format!("{:?}", issue_achievement(caller, achievement_metadata).await?))
    } else {
        Err(String::from("You`re not allowed"))
    }
}

ic_cdk::export_candid!();