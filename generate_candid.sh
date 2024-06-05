cargo build --release --target wasm32-unknown-unknown --package achievement
candid-extractor target/wasm32-unknown-unknown/release/achievement.wasm > src/achievement/achievement.did
cargo build --release --target wasm32-unknown-unknown --package reputation_module
candid-extractor target/wasm32-unknown-unknown/release/reputation_module.wasm > src/reputation_module/reputation_module.did