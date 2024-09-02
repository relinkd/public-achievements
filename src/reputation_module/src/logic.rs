//! This module contains the logic for issuing achievements and managing reputation.

use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;
use ic_cdk::update;

use crate::icrc_7::types::{MintArg, MintResult};
use crate::types::AchievementMetadata;
use crate::state::{
    get_reputation_module_metadata,
    increment_total_issued,
    is_canister_allowed,
    get_achievement_metadata,
    get_principal_achievement_sum_status,
    _change_principal_achievement_sum_status_to_issued
};

/// Issues an achievement to a principal.
///
/// This function mints a new achievement token and assigns it to the specified principal.
///
/// # Arguments
///
/// * `principal` - The principal to whom the achievement will be issued.
/// * `achievement_metadata` - Metadata of the achievement to be issued.
///
/// # Returns
///
/// * `Result<MintResult, String>` - The result of the minting operation.
async fn issue_achievement(principal: Principal, achievement_metadata: AchievementMetadata) -> Result<MintResult, String> {
    let reputation_metadata = get_reputation_module_metadata();

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

/// Issues an achievement to the caller's identity wallet.
///
/// This function checks if the achievement is allowed and if it has not been issued already,
/// then issues the achievement to the caller's identity wallet.
///
/// # Arguments
///
/// * `achievement` - The principal of the achievement canister.
///
/// # Returns
///
/// * `Result<u128, String>` - The result of the issuance operation.
#[update(name = "issueAchievementToIdentityWallet")]
async fn issue_achievement_to_identity_wallet(achievement: Principal) -> Result<u128, String> {
    let canister_permission = is_canister_allowed(achievement)?;

    if !canister_permission.0 {
        return Err(String::from("Achievement not allowed"));
    }

    let caller = ic_cdk::api::caller();
    let status: (Result<u8, String>, ) = ic_cdk::call(achievement, "getPrincipalToAchievementStatusValue", (caller,)).await.unwrap();
    let status_result = status.0.unwrap();
    let achievement_metadata = get_achievement_metadata(achievement).await.unwrap();

    let issued_status = get_principal_achievement_sum_status(caller, achievement);

    if(issued_status) {
        return Err(String::from("Achievement already issued"));
    }

    if status_result == 1_u8 {
        let result = issue_achievement(caller, achievement_metadata).await.unwrap();
        match result {
            Ok(n) => {
                _change_principal_achievement_sum_status_to_issued(caller, achievement)?;
                return Ok(n)
            },
            Err(_) => return Err(String::from("Mint Error"))
        }
    } else {
        Err(String::from("You`re not allowed"))
    }
}