# Public-achievement

## Generate candid for canisters

```bash
sh generate_candid.sh
```

## Achievement test hash generate

local_wallet 2vxsx-fae
identity_wallet 2vxsx-fae

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd example/
dfx help
dfx canister --help
```

## Achievement flow

```bash
dfx --identity pa_local_wallet canister call achievement checkAchievementEligibility "(principal \"zdunl-kt7k5-ob2sc-mbplm-za3de-ilig5-rmzst-vk4xc-ljb6a-mmki4-jqe\", vec {})"
dfx --identity pa_identity_wallet canister call achievement checkAchievementEligibility "(principal \"265xa-mybwx-ttdsp-fmlbc-ooy4e-zly4z-zckoz-3ukod-5gutk-jdf4h-hae\", vec {})"

dfx --identity pa_local_wallet canister call achievement generateHashToIdentityWallet "(principal \"265xa-mybwx-ttdsp-fmlbc-ooy4e-zly4z-zckoz-3ukod-5gutk-jdf4h-hae\", vec {})"
 
(
  variant {
    Ok = "Succesfully generate hash for Identity Wallet. Signature 3091a23ffdc93967debb655fadc1926be866e611be7c4051e25b500699012718175c3975a80c7d6b1b5f7c20f88ff8922261e30f98d315e8dc45b61ef006e9fd"
  },
)

dfx --identity pa_identity_wallet canister call achievement receiveAchievementFromIdentityWalletWithHash "(principal \"zdunl-kt7k5-ob2sc-mbplm-za3de-ilig5-rmzst-vk4xc-ljb6a-mmki4-jqe\")"
 
(variant { Ok = "Achievement status changed to allowed" })

dfx --identity pa_identity_wallet canister call achievement receiveAchievementFromIdentityWallet "(vec {})"
 
(variant { Ok = "Achievement status changed to allowed" })
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `ic` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor
