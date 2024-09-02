//! This module provides utility functions for the reputation module.

use candid::Principal;

/// Builds a string representation of a principal sum.
///
/// This function concatenates the string representations of an identity wallet and an achievement principal.
///
/// # Arguments
///
/// * `identity_wallet` - The principal of the identity wallet.
/// * `achievement` - The principal of the achievement canister.
///
/// # Returns
///
/// * `String` - The concatenated string representation of the principal sum.
pub fn build_principal_sum(identity_wallet: Principal, achievement: Principal) -> String {
    let mut principal_sum = String::from("");
    principal_sum.push_str(&identity_wallet.to_string());
    principal_sum.push_str(&achievement.to_string());

    principal_sum
}

