dfx canister call reputation_module updateReputationModuleMetadata "record {achievement_collection=principal \"bkyz2-fmaaa-aaaaa-qaaaq-cai\"; issuer_name=\"test\"; issuer_description=\"test\"; total_issued=0}"
dfx canister call reputation_module changePermissionCanister "(principal \"$(dfx canister id achievement)\", true)"
dfx --identity pa_identity_wallet canister call achievement receiveAchievementFromIdentityWallet "(vec {})"
dfx --identity pa_identity_wallet canister call reputation_module issueAchievementToIdentityWallet "(principal \"$(dfx canister id achievement)\")"