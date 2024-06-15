use ic_cdk::query;
use candid::Principal;

#[query(name = "caller")]
pub fn caller() -> Principal {
    let id = ic_cdk::api::caller();

    return id;
}

#[query(name = "isController")]
pub fn is_controller() -> bool {
    let id = ic_cdk::api::caller();
    let is_controller = ic_cdk::api::is_controller(&id);

    return is_controller;
}
