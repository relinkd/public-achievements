use ic_cdk::{query, update};
use candid::{Principal};

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use ic_stable_structures::{
    DefaultMemoryImpl, StableBTreeMap, StableVec, StableCell
};
use std::cell::RefCell;

use crate::state::{get_principal_to_hash_value, update_principal_to_hash, update_principal_to_achievement_status};
use crate::ecdsa::{public_key, build_principals_message, sign, verify};
use crate::storable::{PrincipalStorable, Signature, AchievementStatusEnum, AchievementStatus};


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

        update_principal_to_hash(caller, Signature(signature.clone().signature_hex))?;

        Ok(String::from(format!("Succesfully generate hash for Identity Wallet. Signature {}", signature.signature_hex)))
    } else {
        Err(String::from("Caller principal is not eligible"))
    }
}

#[update(name = "receiveAchievementFromIdentityWallet")]
async fn receive_achievement_from_identity_wallet(blob: Vec<u8>) -> Result<String, String> {
    let caller = ic_cdk::api::caller();
    let eligibility = check_achievement_eligibility(caller, blob).unwrap();

    if eligibility {
        let allowed_status = AchievementStatusEnum::Allowed;
        update_principal_to_achievement_status(caller, AchievementStatus(allowed_status.to_u8()))?;

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
        update_principal_to_achievement_status(caller, AchievementStatus(allowed_status.to_u8()))?;

        Ok(String::from("Achievement status changed to allowed"))
    } else {
        Err(String::from("Principal is not eligible or hash mismatch"))
    }
}