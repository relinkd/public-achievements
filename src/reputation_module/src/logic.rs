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

#[update(name = "issueAchievementToIdentityWallet")]
async fn issue_achievement_to_identity_wallet(achievement: Principal) -> Result<String, String> {
    let canister_permission = is_canister_allowed(achievement)?;

    if !canister_permission.0 {
        return Err(String::from("Achievement not allowed"));
    }

    let caller = ic_cdk::api::caller();
    let status: (Result<u8, String>, ) = ic_cdk::call(achievement, "getPrincipalToAchievementStatusValue", (caller,)).await.unwrap();
    let achievement_metadata = get_achievement_metadata(achievement).await.unwrap();

    let issued_status = get_principal_achievement_sum_status(caller, achievement);

    if(issued_status) {
        return Err(String::from("Achievement already issued"));
    }

    if status.0? == 1_u8 {
        let issue_result = format!("{:?}", issue_achievement(caller, achievement_metadata).await?);
        _change_principal_achievement_sum_status_to_issued(caller, achievement)?;
        Ok(issue_result)
    } else {
        Err(String::from("You`re not allowed"))
    }
}