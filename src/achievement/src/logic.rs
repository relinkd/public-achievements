//! This module contains the logic for checking achievement eligibility and managing achievements.

use ic_cdk::{query, update};
use candid::Principal;

use crate::state::{get_principal_to_hash_value, update_principal_to_hash, update_principal_to_achievement_status};
use crate::ecdsa::{public_key, build_principals_message, sign, verify};
use crate::storable::{Signature, AchievementStatusEnum, AchievementStatus};

/// Checks if a principal is eligible for an achievement.
///
/// # Arguments
///
/// * `principal` - The principal to check.
/// * `blob` - Additional data for eligibility check.
///
/// # Returns
///
/// * `Result<bool, String>` - `true` if the principal is eligible, `false` otherwise.
/// 
/// # Example
/// 
/// ## IC-reactor usage
/// ```typescript
///     const { data: eligible, call: fetchEligigble }: { data: any, call: any } = useUpdateCall({
///         functionName: "checkAchievementEligibility",
///         args: [
///             identity?.getPrincipal(),
///             []
///         ]
///     })
/// ```
#[update(name = "checkAchievementEligibility")]
fn check_achievement_eligibility(principal: Principal, blob: Vec<u8>) -> Result<bool, String> {

    // Your conditions for achievement

    Ok(true)
}

/// Generates a hash for the caller's identity wallet.
///
/// # Arguments
///
/// * `identity_wallet` - The principal of the identity wallet.
/// * `blob` - Additional data for hash generation.
///
/// # Returns
///
/// * `Result<String, String>` - The result of the hash generation.
///
/// # Example
///
/// ```
/// dfx --identity pa_local_wallet canister call achievement generateHashToIdentityWallet "(principal \"$(dfx --identity pa_identity_wallet identity get-principal)\", vec {})"
///
/// (
///   variant {
///     Ok = "Succesfully generate hash for Identity Wallet. Signature 5ac9cae0bd534ee09eea7bf9ddd85a53ba13efe9a416fb13155b46fa2af2f3f0671b2b79c534a29ade73811098cb947ccbd606b935aa1e0610093eac3b3ddc00"
///   },
/// )
/// ```
///
/// ## IC-reactor usage
/// ```typescript
/// const { call: generateHash }: { call: any} = useUpdateCall({
///     functionName: "generateHashToIdentityWallet",
///     args: [
///         Principal.fromText(identity_wallet as string || identity!.getPrincipal()!.toText()),
///         []
///     ]
/// })
/// ```
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

/// Receives an achievement for the caller's identity wallet.
///
/// # Arguments
///
/// * `blob` - Additional data for receiving the achievement.
///
/// # Returns
///
/// * `Result<String, String>` - The result of the achievement reception.
///
/// # Example
///
/// ```
/// dfx --identity pa_identity_wallet canister call reputation_module issueAchievementToIdentityWallet "(principal \"$(dfx canister id achievement)\")"
///
/// (variant { Ok = "Achievement issued" })
/// ```
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

/// Receives an achievement for the caller's identity wallet using a hash.
///
/// # Arguments
///
/// * `principal` - The principal of the identity wallet.
///
/// # Returns
///
/// * `Result<String, String>` - The result of the achievement reception.
///
/// # Example
///
/// ```
/// dfx --identity pa_identity_wallet canister call achievement receiveAchievementFromIdentityWalletWithHash "(principal \"$(dfx --identity pa_local_wallet identity get-principal)\")"
///
/// (variant { Ok = "Achievement status changed to allowed" })
/// ```
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