//! This module provides access control functions for checking if the caller is a controller
//! and for retrieving the caller's principal ID.

use ic_cdk::query;
use candid::Principal;

/// Checks if the caller is a controller.
///
/// This function retrieves the caller's ID and checks if the caller is a controller.
///
/// # Returns
///
/// * `bool` - `true` if the caller is a controller, `false` otherwise.
#[query(name = "isController")]
pub fn is_controller() -> bool {
    let id = ic_cdk::api::caller();
    let is_controller = ic_cdk::api::is_controller(&id);

    return is_controller;
}

/// Retrieves the caller's principal ID.
///
/// This function returns the principal ID of the caller.
///
/// # Returns
///
/// * `Principal` - The principal ID of the caller.
#[query(name = "caller")]
pub fn caller() -> Principal {
    let id = ic_cdk::api::caller();

    return id;
}