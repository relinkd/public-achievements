pub mod ecdsa;
pub mod storable;

use ic_cdk::{query, update};
use candid::{Principal};

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use ic_stable_structures::{
    DefaultMemoryImpl, StableBTreeMap
};
use std::cell::RefCell;

use storable::{PrincipalStorable, AchievementStatus, Memory, Signature, AchievementStatusEnum};
use ecdsa::{sign, public_key, verify, build_principals_message};


thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static PRINCIPAL_TO_ACHIEVEMENT_STATUS: RefCell<StableBTreeMap<PrincipalStorable, AchievementStatus, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );

    // HASH = HASH_FROM_PRINCIPAL_TO_IDENTITY_WALLET

    static PRINCIPAL_TO_HASH: RefCell<StableBTreeMap<PrincipalStorable, Signature, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        )
    );
}

#[query(name = "caller")]
fn caller() -> Principal {
    let id = ic_cdk::api::caller();

    return id;
}

#[query(name = "checkAchievementEligibility")]
fn check_achievement_eligibility(principal: Principal, blob: Vec<u8>) -> Result<bool, String> {

    // Your conditions for achievement

    Ok(true)
}

#[update(name = "generateHashToIdentityWallet")]
async fn generate_hash_to_identity_wallet(identity_wallet: Principal, blob: Vec<u8>) -> Result<String, String> {
    let caller = ic_cdk::api::caller();
    let eligibility = check_achievement_eligibility(caller, blob).unwrap();

    if eligibility {
        let message = build_principals_message(caller, identity_wallet);
        let signature = sign(message).await?;

        PRINCIPAL_TO_HASH.with(|p| p.borrow_mut().insert(PrincipalStorable(caller), Signature(signature.clone().signature_hex)));

        Ok(String::from(format!("Succesfully generate hash for Identity Wallet. Signature {}", signature.signature_hex)))
    } else {
        Err(String::from("Caller principal is not eligible"))
    }
}

#[query(name = "getPrincipalToHashValue")]
fn get_principal_to_hash_value(principal: Principal) -> Result<Signature, String> {
    if let Some(hash) = PRINCIPAL_TO_HASH.with(|p| p.borrow().get(&PrincipalStorable(principal))) {
        Ok(hash)
    } else {
        Err(String::from("Hash not found"))
    }
}

#[update(name = "receiveAchievementFromIdentityWallet")]
async fn receive_achievement_from_identity_wallet(blob: Vec<u8>) -> Result<String, String> {
    let caller = ic_cdk::api::caller();
    let eligibility = check_achievement_eligibility(caller, blob).unwrap();

    if eligibility {
        let allowed_status = AchievementStatusEnum::Allowed;
        PRINCIPAL_TO_ACHIEVEMENT_STATUS.with(|p| p.borrow_mut().insert(PrincipalStorable(caller), AchievementStatus(allowed_status.to_u8())));

        Ok(String::from("Achievement status changed to allowed"))
    } else {
        Err(String::from("Caller principal is not eligible"))
    }
}

#[update(name = "receiveAchievementFromIdentityWalletWithHash")]
async fn receive_achievement_from_identity_wallet_with_hash(principal: Principal) -> Result<String, String> {
    let caller = ic_cdk::api::caller();
    let hash = get_principal_to_hash_value(principal)?;
    let public_key = public_key().await?;
    let message = build_principals_message(principal, caller);
    let eligibility = verify(hash.0, message, public_key.public_key_hex).await?;

    if eligibility.is_signature_valid {
        let allowed_status = AchievementStatusEnum::Allowed;
        PRINCIPAL_TO_ACHIEVEMENT_STATUS.with(|p| p.borrow_mut().insert(PrincipalStorable(caller), AchievementStatus(allowed_status.to_u8())));

        Ok(String::from("Achievement status changed to allowed"))
    } else {
        Err(String::from("Principal is not eligible or hash mismatch"))
    }
}

#[query(name = "getPrincipalToAchievementStatusValue")]
fn get_principal_to_achievement_status_value(principal: Principal) -> Result<u8, String> {
    if let Some(achievement_status) = PRINCIPAL_TO_ACHIEVEMENT_STATUS.with(|p| p.borrow().get(&PrincipalStorable(principal))) {
        Ok(achievement_status.0)
    } else {
        Err(String::from("Achievement status not found"))
    }
}

ic_cdk::export_candid!();