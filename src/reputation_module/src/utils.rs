use candid::Principal;

pub fn build_principal_sum(identity_wallet: Principal, achievement: Principal) -> String {
    let mut principal_sum = String::from("");
    principal_sum.push_str(&identity_wallet.to_string());
    principal_sum.push_str(&achievement.to_string());

    principal_sum
}

