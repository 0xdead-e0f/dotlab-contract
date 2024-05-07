export CHAIN_ID="atlantic-2"
export ACCOUNT_ADDRESS="sei1gg0sdu9j3u2zpc4ewqfgaxx6d6s7fsw7taqkuj"
export ACCOUNT_NAME="sei-test-1"
export ENDPOINT="https://rpc.atlantic-2.seinetwork.io/"


# Building Cargo 
printf "Building Cargo "
$(printf $password | sudo cargo build )


# Optimizing Contracts With Latest Build
printf "Optimizing Contracts With Latest Build"
$(printf $password | sudo docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/rust-optimizer:0.12.11 )


# Uploading Wasm Codes
printf "Uploading Wasm Codes - Registry %s\n"
codeRegistry=$(printf $password | seid tx wasm store artifacts/registry.wasm -y --from=$ACCOUNT_ADDRESS --chain-id=$CHAIN_ID --gas=10000000 --fees=2000000usei --broadcast-mode=block --node $ENDPOINT | grep -A 1 "code_id" | sed -n 's/.*value: "//p' | sed -n 's/"//p')
printf "Code id is %s\n" $codeRegistry
printf  "Registry CodeId : $codeRegistry\n" >> contracts.txt


printf "Uploading Wasm Codes - Registrar %s\n"
codeRegistrar=$(printf $password | seid tx wasm store artifacts/registrar.wasm -y --from=$ACCOUNT_ADDRESS --chain-id=$CHAIN_ID --gas=10000000 --fees=2000000usei --broadcast-mode=block --node $ENDPOINT | grep -A 1 "code_id" | sed -n 's/.*value: "//p' | sed -n 's/"//p')
printf "Code id is %s\n" $codeRegistrar
printf  "Registrar CodeId : $codeRegistrar\n" >> contracts.txt


printf "Uploading Wasm Codes - Reverse %s\n"
codeReverse=$(printf $password | seid tx wasm store artifacts/reverse_registar.wasm -y --from=$ACCOUNT_ADDRESS --chain-id=$CHAIN_ID --gas=10000000 --fees=2000000usei --broadcast-mode=block --node $ENDPOINT | grep -A 1 "code_id" | sed -n 's/.*value: "//p' | sed -n 's/"//p')
printf "Code id is %s\n" $codeReverse
printf  "Reverse CodeId : $codeReverse\n" >> contracts.txt


printf "Uploading Wasm Codes - Controller %s\n"
codeController=$(printf $password | seid tx wasm store artifacts/controller.wasm -y --from=$ACCOUNT_ADDRESS --chain-id=$CHAIN_ID --gas=10000000 --fees=2000000usei --broadcast-mode=block --node $ENDPOINT | grep -A 1 "code_id" | sed -n 's/.*value: "//p' | sed -n 's/"//p')
printf "Code id is %s\n" $codeController
printf  "Controller CodeId : $codeController\n" >> contracts.txt


printf "Uploading Wasm Codes - Resolver %s\n"
codeResolver=$(printf $password | seid tx wasm store artifacts/resolver.wasm -y --from=$ACCOUNT_ADDRESS --chain-id=$CHAIN_ID --gas=10000000 --fees=2000000usei --broadcast-mode=block --node $ENDPOINT | grep -A 1 "code_id" | sed -n 's/.*value: "//p' | sed -n 's/"//p')
printf "Code id is %s\n" $codeResolver
printf  "Resolver CodeId : $codeResolver\n" >> contracts.txt


# Deploying Registry
printf "Deploying Registry %s\n"
addrRegistry=$(seid tx wasm instantiate $codeRegistry '{}' --chain-id $CHAIN_ID --from $ACCOUNT_NAME --gas=4000000 --fees=1000000usei --broadcast-mode=block --label "dotnames-registry" --admin $ACCOUNT_ADDRESS --node $ENDPOINT -y| grep -A 1 -m 1 "key: _contract_address" | sed -n 's/.*value: //p' | xargs)
printf "Deployed registry address is %s\n" $addrRegistry
printf  "Registry Address : $addrRegistry\n" >> contracts.txt


#Deploying Registrar
printf "Deploying Registrar %s\n"
addrRegistrar=$(seid tx wasm instantiate $codeRegistrar '{"base_name": "sei","registry_address": "'"$addrRegistry"'" ,"name": "dotlabs-dotsei","symbol": "DOTSEI","base_uri": "metadata.dotsei.me/api/"}' --chain-id $CHAIN_ID --from $ACCOUNT_NAME --gas=4000000 --fees=4000000usei --broadcast-mode=block --label "dotnames-registrar" --admin $ACCOUNT_ADDRESS --node $ENDPOINT -y| grep -A 1 -m 1 "key: _contract_address" | sed -n 's/.*value: //p' | xargs)
printf "Deployed registrar address is %s\n" $addrRegistrar
printf  "Registrar Address : $addrRegistrar \n" >> contracts.txt


#Deploying Reverse Registrar
printf "Deploying Reverse Registrar %s\n"
addrReverse=$(seid tx wasm instantiate $codeReverse '{
  "registry_address": "'"$addrRegistry"'",
  "resolver_address": "'"$ACCOUNT_ADDRESS"'"
}' --chain-id $CHAIN_ID --from $ACCOUNT_NAME --gas=4000000 --fees=2000000usei --broadcast-mode=block --label "dotnames-reverse-registrar" --admin $ACCOUNT_ADDRESS --node $ENDPOINT -y| grep -A 1 -m 1 "key: _contract_address" | sed -n 's/.*value: //p' | xargs)
printf "Deployed reverse registrar address is %s\n" $addrReverse
printf  "Reverse Registrar Address : $addrReverse \n" >> contracts.txt


#Deploying Controller
printf "Deploying Controller %s\n"
addrController=$(seid tx wasm instantiate $codeController '{
  "min_registration_duration": 31536000,
  "tier1_price": 1000000,
  "tier2_price": 300000,
  "tier3_price": 100000,
  "whitelist_price": 500000,
  "referal_percentage": [
    10,
    30
  ],
  "enable_registration": true,
  "registrar_address": "'"$addrRegistrar"'", "reverse_registrar_address": "'"$addrReverse"'","description" : "DotSei Domains- Making your complex blockchain addresses easy"
}' --chain-id $CHAIN_ID --from $ACCOUNT_NAME --gas=4000000 --fees=2000000usei --broadcast-mode=block --label "dotnames-controller" --admin $ACCOUNT_ADDRESS --node $ENDPOINT -y| grep -A 1 -m 1 "key: _contract_address" | sed -n 's/.*value: //p' | xargs)
printf "Deployed controller address is %s\n" $addrController
printf  "Controller Address : $addrController \n" >> contracts.txt



#Deploying Resolver
printf "Deploying Resolver %s\n"
addrResolver=$(seid tx wasm instantiate $codeResolver '{
  "interface_id": 1,
  "registry_address": "'"$addrRegistry"'",
  "trusted_reverse_registrar": "'"$addrReverse"'",
  "trusted_controller": "'"$addrController"'"
}' --chain-id $CHAIN_ID --from $ACCOUNT_NAME --gas=4000000 --fees=2000000usei --broadcast-mode=block --label "dotnames-resolver" --admin $ACCOUNT_ADDRESS --node $ENDPOINT -y| grep -A 1 -m 1 "key: _contract_address" | sed -n 's/.*value: //p' | xargs)
printf "Deployed resolver address is %s\n" $addrResolver
printf  "Resolver Address : $addrResolver \n" >> contracts.txt





