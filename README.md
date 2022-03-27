## Deploy contract

```sh
cargo build --all --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm ./res/
near dev-deploy --wasmFile res/near_fm.wasm 
source neardev/dev-account.env
echo $ID
```

## Init contract
```
near call $ID new '' --accountId $ID
```

## Create new project
```
near call $ID create_project '{"metadata": {"title": "Crowdfunding demo", "description":
"Crowdfunding description", "target": "1000000000000000000000000000", "minimum_pledge":
"1000000000000"}}' --accountId crowdfunding_creator.testnet
```

## Donate project
```
near call $ID donate_project '{"project_id": "$PROJ_ID"}' --accountId donate_account.testnet
--amount 1
```


## Claim 
```
near call $ID claim '{"project_id": "$PROJ_ID"}' --accountId donate_account.testnet --attatchYocto
1
```
