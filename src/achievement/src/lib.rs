pub mod ecdsa;
pub mod storable;

use ic_cdk::{query, update};
use candid::{Principal};

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use ic_stable_structures::{
    DefaultMemoryImpl, StableBTreeMap
};
use std::cell::RefCell;

use storable::{PrincipalStorable, AchievementStatus, Memory, Signature};
use ecdsa::{sign};


thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static ACHIEVEMENT_STATUS: RefCell<StableBTreeMap<PrincipalStorable, AchievementStatus, Memory>> = RefCell::new(
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

#[query(name = "checkAchievementEligibility")]
fn check_achievement_eligibility(principal: Principal, blob: Vec<u8>) -> Result<bool, String> {

    // Your conditions for achievement

    Ok(true)
}

#[update(name = "receiveToIdentityWallet")]
async fn receive_to_identity_wallet(identity_wallet: Principal, blob: Vec<u8>) -> Result<String, String> {
    let caller = ic_cdk::api::caller();
    let eligibility = check_achievement_eligibility(caller, blob).unwrap();

    if eligibility {
        let mut message = String::from("");
        message.push_str(&caller.to_string());
        message.push_str(&identity_wallet.to_string());
        let signature = sign(message).await?;

        PRINCIPAL_TO_HASH.with(|p| p.borrow_mut().insert(PrincipalStorable(caller), Signature(signature.clone().signature_hex)));

        Ok(String::from(format!("Succesfully generate hash for Identity Wallet. Signature {}", signature.signature_hex)))
    } else {
        return Err(String::from("Caller principal is not eligible"))
    }
}

ic_cdk::export_candid!();