mod tests {
    use crate::contract::{execute, instantiate, query};
    use crate::error::ContractError;
    use crate::mock_querier::mock_dependencies;
    use crate::msg::{
        ExecuteMsg, InstantiateMsg, MinRegistrationDurationResponse, NodehashResponse,
        OwnerResponse, PriceResponse, QueryMsg, RegistrarResponse, RentPriceResponse,
        TokenIdResponse,
    };
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{
        coins, from_binary, to_binary, Addr, BankMsg, Coin, CosmosMsg, Timestamp, Uint128, WasmMsg,
    };
    use dotlabs::registrar::{ExecuteMsg as RegistrarExecuteMsg, Extension};
    use dotlabs::registry::ExecuteMsg as RegistryExecuteMsg;
    use dotlabs::resolver::ExecuteMsg as ResolverExecuteMsg;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {
            registrar_address: String::from("registrar_address"),
            reverse_registrar_address: String::from("reverse_registrar_address"),
            min_registration_duration: 0,
            tier1_price: 640_000_000u64,
            tier2_price: 160_000_000u64,
            tier3_price: 5_000_000u64,
            whitelist_price: 640_000_000u64,
            referal_percentage: (20, 40),
            enable_registration: true,
            description: "".to_string(),
        };
        let info = mock_info("creator", &coins(0, "usei"));
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn test_get_token_id() {
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {
            registrar_address: String::from("registrar_address"),
            reverse_registrar_address: String::from("reverse_registrar_address"),
            min_registration_duration: 0,
            tier1_price: 640_000_000u64,
            tier2_price: 160_000_000u64,
            tier3_price: 5_000_000u64,
            whitelist_price: 640_000_000u64,
            referal_percentage: (20, 40),
            enable_registration: true,
            description: "".to_string(),
        };
        let info = mock_info("creator", &coins(0, "usei"));
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let name = String::from("alice");

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetTokenId { name }).unwrap();
        let token_id_response: TokenIdResponse = from_binary(&res).unwrap();
        assert_eq!(
            token_id_response.token_id,
            "9c0257114eb9399a2985f8e75dad7600c5d89fe3824ffa99ec1c3eb8bf3b0501"
        );
    }

    #[test]
    fn test_get_nodehash_from_name() {
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {
            registrar_address: String::from("registrar_address"),
            reverse_registrar_address: String::from("reverse_registrar_address"),
            min_registration_duration: 0,
            tier1_price: 640_000_000u64,
            tier2_price: 160_000_000u64,
            tier3_price: 5_000_000u64,
            whitelist_price: 640_000_000u64,
            referal_percentage: (20, 40),
            enable_registration: true,
            description: "".to_string(),
        };
        let info = mock_info("creator", &coins(0, "usei"));
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let name = String::from("alice");
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetNodehash { name }).unwrap();
        let nodehash_response: NodehashResponse = from_binary(&res).unwrap();
        assert_eq!(
            nodehash_response.node,
            [
                78, 137, 50, 222, 163, 237, 87, 141, 30, 30, 144, 123, 133, 152, 167, 161, 204, 44,
                197, 227, 125, 124, 105, 133, 160, 177, 82, 121, 97, 207, 166, 156
            ]
        )
    }

    #[test] // Should return correct messages
    fn test_register() {
        let mut deps = mock_dependencies(&[]);
        let registrar_address = String::from("registrar_address");
        let reverse_registrar_address = String::from("reverse_registrar_address");
        let msg = InstantiateMsg {
            registrar_address: registrar_address.clone(),
            reverse_registrar_address: reverse_registrar_address.clone(),
            min_registration_duration: 24 * 3600 * 365,

            tier1_price: 640_000_000u64,
            tier2_price: 160_000_000u64,
            tier3_price: 5_000_000u64,
            whitelist_price: 640_000_000u64,
            referal_percentage: (20, 40),
            enable_registration: true,
            description: "".to_string(),
        };
        let info = mock_info("creator", &coins(0, "usei"));
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let name = String::from("alice");
        let owner = String::from("alice");
        let secret = String::from("tns_secret");
        let resolver = String::from("registry_address");
        let address = String::from("alice_addr");
        let info = mock_info("alice", &coins(0, "usei"));

        let duration: u64 = 24 * 3600 * 365;
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::RentPrice {
                name: name.clone(),
                duration: duration.clone(),
            },
        )
        .unwrap();
        let rent_price_response: RentPriceResponse = from_binary(&res).unwrap();
        let info = mock_info("alice", &coins(rent_price_response.price.u128(), "usei"));
        let msg = ExecuteMsg::Register {
            name: name.clone(),
            owner: owner.clone(),
            duration: duration.clone(),
            secret: secret.clone(),
            resolver: Some(resolver.clone()),
            address: Some(address.clone()),
            reverse_record: false,
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let register_registrar_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: registrar_address.clone(),
            msg: to_binary(&RegistrarExecuteMsg::<Extension>::Register {
                id: String::from(
                    "9c0257114eb9399a2985f8e75dad7600c5d89fe3824ffa99ec1c3eb8bf3b0501",
                ), // token_id of alice
                owner: mock_env().contract.address.to_string(),
                duration: duration.clone(),
                name: name.clone(),
                extension: Extension {
                    name: name,
                    description: String::from(""),
                },
            })
            .unwrap(),
            funds: vec![],
        });

        let registry_set_resolver_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: "registry_address".to_string(),
            msg: to_binary(&RegistryExecuteMsg::SetResolver {
                node: vec![
                    78, 137, 50, 222, 163, 237, 87, 141, 30, 30, 144, 123, 133, 152, 167, 161, 204,
                    44, 197, 227, 125, 124, 105, 133, 160, 177, 82, 121, 97, 207, 166, 156,
                ], // nodehash of alice.ust
                resolver: Some("registry_address".to_string()),
            })
            .unwrap(),
            funds: vec![],
        });

        let set_address_resolver_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: "registry_address".to_string(),
            msg: to_binary(&ResolverExecuteMsg::SetSeiAddress {
                node: vec![
                    78, 137, 50, 222, 163, 237, 87, 141, 30, 30, 144, 123, 133, 152, 167, 161, 204,
                    44, 197, 227, 125, 124, 105, 133, 160, 177, 82, 121, 97, 207, 166, 156,
                ], // nodehash of alice.ust
                address: address.clone(),
            })
            .unwrap(),
            funds: vec![],
        });

        let reclaim_registrar_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: "registrar_address".to_string(),
            msg: to_binary(&RegistrarExecuteMsg::<Extension>::Reclaim {
                id: String::from(
                    "9c0257114eb9399a2985f8e75dad7600c5d89fe3824ffa99ec1c3eb8bf3b0501",
                ), // token_id of alice,
                owner: owner.clone(),
            })
            .unwrap(),
            funds: vec![],
        });

        let transfer_nft_registrar_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: "registrar_address".to_string(),
            msg: to_binary(&RegistrarExecuteMsg::<Extension>::TransferNft {
                recipient: owner.clone(),
                token_id: String::from(
                    "9c0257114eb9399a2985f8e75dad7600c5d89fe3824ffa99ec1c3eb8bf3b0501",
                ), // token_id of alice,
            })
            .unwrap(),
            funds: vec![],
        });

        assert_eq!(res.messages.len(), 5); // Register, Set resolver, Set name, Reclaim, Transfer NFT
        assert_eq!(res.messages[0].msg, register_registrar_msg);
        assert_eq!(res.messages[1].msg, registry_set_resolver_msg);
        assert_eq!(res.messages[2].msg, set_address_resolver_msg);
        assert_eq!(res.messages[3].msg, reclaim_registrar_msg);
        assert_eq!(res.messages[4].msg, transfer_nft_registrar_msg);

        let name = String::from("Alice");
        let owner = String::from("alice");
        let secret = String::from("tns_secret");
        let resolver = String::from("registry_address");
        let address = String::from("alice_addr");
        let info = mock_info("alice", &coins(0, "usei"));

        let duration: u64 = 24 * 3600 * 365;
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::RentPrice {
                name: name.clone(),
                duration: duration.clone(),
            },
        )
        .unwrap();
        let rent_price_response: RentPriceResponse = from_binary(&res).unwrap();
        let info = mock_info("alice", &coins(rent_price_response.price.u128(), "usei"));
        let msg = ExecuteMsg::Register {
            name: name.clone(),
            owner: owner.clone(),
            duration: duration.clone(),
            secret: secret.clone(),
            resolver: Some(resolver.clone()),
            address: Some(address.clone()),
            reverse_record: false,
        };
        assert_eq!(execute(deps.as_mut(), mock_env(), info, msg).is_err(), true);
    }

    #[test] // Should return correct messages
    fn test_owner_register() {
        let mut deps = mock_dependencies(&[]);
        let registrar_address = String::from("registrar_address");
        let reverse_registrar_address = String::from("reverse_registrar_address");
        let msg = InstantiateMsg {
            registrar_address: registrar_address.clone(),
            reverse_registrar_address: reverse_registrar_address.clone(),
            min_registration_duration: 24 * 3600 * 365,

            tier1_price: 640_000_000u64,
            tier2_price: 160_000_000u64,
            tier3_price: 5_000_000u64,
            whitelist_price: 640_000_000u64,
            referal_percentage: (20, 40),
            enable_registration: true,
            description: "".to_string(),
        };
        let info = mock_info("creator", &coins(0, "usei"));
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let name = String::from("alice");
        let owner = String::from("alice");
        let resolver = String::from("registry_address");
        let address = String::from("alice_addr");
        let duration: u64 = 24 * 3600 * 365;
        let msg = ExecuteMsg::OwnerRegister {
            name: name.clone(),
            owner: owner.clone(),
            duration: duration.clone(),
            resolver: Some(resolver.clone()),
            address: Some(address.clone()),
            reverse_record: false,
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        let register_registrar_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: registrar_address.clone(),
            msg: to_binary(&RegistrarExecuteMsg::<Extension>::Register {
                id: String::from(
                    "9c0257114eb9399a2985f8e75dad7600c5d89fe3824ffa99ec1c3eb8bf3b0501",
                ), // token_id of alice
                owner: mock_env().contract.address.to_string(),
                duration: duration.clone(),
                name: name.clone(),
                extension: Extension {
                    name: name,
                    description: String::from(""),
                },
            })
            .unwrap(),
            funds: vec![],
        });

        let registry_set_resolver_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: "registry_address".to_string(),
            msg: to_binary(&RegistryExecuteMsg::SetResolver {
                node: vec![
                    78, 137, 50, 222, 163, 237, 87, 141, 30, 30, 144, 123, 133, 152, 167, 161, 204,
                    44, 197, 227, 125, 124, 105, 133, 160, 177, 82, 121, 97, 207, 166, 156,
                ], // nodehash of alice.ust
                resolver: Some("registry_address".to_string()),
            })
            .unwrap(),
            funds: vec![],
        });

        let set_address_resolver_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: "registry_address".to_string(),
            msg: to_binary(&ResolverExecuteMsg::SetSeiAddress {
                node: vec![
                    78, 137, 50, 222, 163, 237, 87, 141, 30, 30, 144, 123, 133, 152, 167, 161, 204,
                    44, 197, 227, 125, 124, 105, 133, 160, 177, 82, 121, 97, 207, 166, 156,
                ], // nodehash of alice.ust
                address: address.clone(),
            })
            .unwrap(),
            funds: vec![],
        });

        let reclaim_registrar_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: "registrar_address".to_string(),
            msg: to_binary(&RegistrarExecuteMsg::<Extension>::Reclaim {
                id: String::from(
                    "9c0257114eb9399a2985f8e75dad7600c5d89fe3824ffa99ec1c3eb8bf3b0501",
                ), // token_id of alice,
                owner: owner.clone(),
            })
            .unwrap(),
            funds: vec![],
        });

        let transfer_nft_registrar_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: "registrar_address".to_string(),
            msg: to_binary(&RegistrarExecuteMsg::<Extension>::TransferNft {
                recipient: owner.clone(),
                token_id: String::from(
                    "9c0257114eb9399a2985f8e75dad7600c5d89fe3824ffa99ec1c3eb8bf3b0501",
                ), // token_id of alice,
            })
            .unwrap(),
            funds: vec![],
        });

        assert_eq!(res.messages.len(), 5); // Register, Set resolver, Set name, Reclaim, Transfer NFT
        assert_eq!(res.messages[0].msg, register_registrar_msg);
        assert_eq!(res.messages[1].msg, registry_set_resolver_msg);
        assert_eq!(res.messages[2].msg, set_address_resolver_msg);
        assert_eq!(res.messages[3].msg, reclaim_registrar_msg);
        assert_eq!(res.messages[4].msg, transfer_nft_registrar_msg);

        let name = String::from("Alice");
        let owner = String::from("alice");
        let resolver = String::from("registry_address");
        let address = String::from("alice_addr");
        let duration: u64 = 24 * 3600 * 365;
        let msg = ExecuteMsg::OwnerRegister {
            name: name.clone(),
            owner: owner.clone(),
            duration: duration.clone(),
            resolver: Some(resolver.clone()),
            address: Some(address.clone()),
            reverse_record: false,
        };
        assert_eq!(execute(deps.as_mut(), mock_env(), info, msg).is_ok(), true);
    }

    #[test]
    fn test_disable_register() {
        let mut deps = mock_dependencies(&[]);
        let registrar_address = String::from("registrar_address");
        let reverse_registrar_address = String::from("reverse_registrar_address");
        let msg = InstantiateMsg {
            registrar_address: registrar_address.clone(),
            reverse_registrar_address: reverse_registrar_address.clone(),
            min_registration_duration: 24 * 3600 * 365,

            tier1_price: 640_000_000u64,
            tier2_price: 160_000_000u64,
            tier3_price: 5_000_000u64,
            whitelist_price: 640_000_000u64,
            referal_percentage: (20, 40),
            enable_registration: false,
            description: "".to_string(),
        };
        let info = mock_info("creator", &coins(0, "usei"));
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let name = String::from("alice");
        let owner = String::from("alice");
        let secret = String::from("tns_secret");
        let resolver = String::from("registry_address");
        let address = String::from("alice_addr");
        let info = mock_info("alice", &coins(0, "usei"));

        let duration: u64 = 24 * 3600 * 365;
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::RentPrice {
                name: name.clone(),
                duration: duration.clone(),
            },
        )
        .unwrap();
        let rent_price_response: RentPriceResponse = from_binary(&res).unwrap();
        let info = mock_info("alice", &coins(rent_price_response.price.u128(), "usei"));
        let msg = ExecuteMsg::Register {
            name: name.clone(),
            owner: owner.clone(),
            duration: duration.clone(),
            secret: secret.clone(),
            resolver: Some(resolver.clone()),
            address: Some(address.clone()),

            reverse_record: false,
        };
        assert_eq!(execute(deps.as_mut(), mock_env(), info, msg).is_err(), true);

        let msg = ExecuteMsg::SetEnableRegistration {
            enable_registration: true,
        };
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("creator", &coins(0, "usei")),
            msg.clone(),
        )
        .unwrap();

        let info = mock_info("alice", &coins(rent_price_response.price.u128(), "usei"));
        let msg = ExecuteMsg::Register {
            name: name.clone(),
            owner: owner.clone(),
            duration: duration.clone(),
            secret: secret.clone(),
            resolver: Some(resolver.clone()),
            address: Some(address.clone()),

            reverse_record: false,
        };
        assert_eq!(execute(deps.as_mut(), mock_env(), info, msg).is_ok(), true);
    }

    #[test] // Should not be able to register with insufficient fund
    fn test_register_with_insufficient_fund() {
        let mut deps = mock_dependencies(&[]);
        let registrar_address = String::from("registrar_address");
        let reverse_registrar_address = String::from("reverse_registrar_address");
        let msg = InstantiateMsg {
            registrar_address: registrar_address.clone(),
            reverse_registrar_address: reverse_registrar_address.clone(),
            min_registration_duration: 24 * 3600 * 365,

            tier1_price: 640_000_000u64,
            tier2_price: 160_000_000u64,
            tier3_price: 5_000_000u64,
            whitelist_price: 640_000_000u64,
            referal_percentage: (20, 40),
            enable_registration: true,
            description: "".to_string(),
        };
        let info = mock_info("creator", &coins(0, "usei"));
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let name = String::from("alice");
        let owner = String::from("alice");
        let secret = String::from("tns_secret");
        let resolver = String::from("registry_address");
        let address = String::from("alice_addr");
        let info = mock_info("alice", &coins(0, "usei"));

        let duration: u64 = 24 * 3600 * 365;
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::RentPrice {
                name: name.clone(),
                duration: duration.clone(),
            },
        )
        .unwrap();

        let rent_price_response: RentPriceResponse = from_binary(&res).unwrap();

        let info = mock_info(
            "alice",
            &coins(rent_price_response.price.u128() / 2, "usei"),
        );
        let msg = ExecuteMsg::Register {
            name: name.clone(),
            owner: owner.clone(),
            duration: duration.clone(),
            secret: secret.clone(),
            resolver: Some(resolver.clone()),
            address: Some(address.clone()),

            reverse_record: false,
        };

        let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        assert_eq!(
            err,
            ContractError::InsufficientFund {
                amount: Uint128::from(2_500_000u128),
                required: Uint128::from(5_000_000u128),
            }
        )
    }

    #[test]
    fn test_renew() {
        let mut deps = mock_dependencies(&[]);
        let registrar_address = String::from("registrar_address");
        let reverse_registrar_address = String::from("reverse_registrar_address");
        let msg = InstantiateMsg {
            registrar_address: registrar_address.clone(),
            reverse_registrar_address: reverse_registrar_address.clone(),
            min_registration_duration: 24 * 3600 * 365,

            tier1_price: 640_000_000u64,
            tier2_price: 160_000_000u64,
            tier3_price: 5_000_000u64,
            whitelist_price: 640_000_000u64,
            referal_percentage: (20, 40),
            enable_registration: true,
            description: "".to_string(),
        };
        let info = mock_info("creator", &coins(0, "usei"));
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let name = String::from("alice");
        let owner = String::from("alice");
        let secret = String::from("tns_secret");
        let resolver = String::from("registry_address");
        let address = String::from("alice_addr");
        let info = mock_info("alice", &coins(0, "usei"));

        let duration: u64 = 24 * 3600 * 365;
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::RentPrice {
                name: name.clone(),
                duration: duration.clone(),
            },
        )
        .unwrap();
        let rent_price_response: RentPriceResponse = from_binary(&res).unwrap();

        let info = mock_info("alice", &coins(rent_price_response.price.u128(), "usei"));
        let msg = ExecuteMsg::Register {
            name: name.clone(),
            owner: owner.clone(),
            duration: duration.clone(),
            secret: secret.clone(),
            resolver: Some(resolver.clone()),
            address: Some(address.clone()),

            reverse_record: false,
        };

        execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("alice", &coins(rent_price_response.price.u128(), "usei"));
        let msg = ExecuteMsg::Renew {
            name: name.clone(),
            duration: duration.clone(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let renew_registrar_message: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: "registrar_address".to_string(),
            msg: to_binary(&RegistrarExecuteMsg::<Extension>::Renew {
                id: String::from(
                    "9c0257114eb9399a2985f8e75dad7600c5d89fe3824ffa99ec1c3eb8bf3b0501",
                ), // token_id of alice,,
                duration: duration.clone(),
            })
            .unwrap(),
            funds: vec![],
        });

        assert_eq!(res.messages.len(), 1); // Renew
        assert_eq!(res.messages[0].msg, renew_registrar_message);
    }

    #[test]
    fn test_renew_insufficient_fund() {
        let mut deps = mock_dependencies(&[]);
        let registrar_address = String::from("registrar_address");
        let reverse_registrar_address = String::from("reverse_registrar_address");
        let msg = InstantiateMsg {
            registrar_address: registrar_address.clone(),
            reverse_registrar_address: reverse_registrar_address.clone(),
            min_registration_duration: 24 * 3600 * 365,

            tier1_price: 640_000_000u64,
            tier2_price: 160_000_000u64,
            tier3_price: 5_000_000u64,
            whitelist_price: 640_000_000u64,
            referal_percentage: (20, 40),
            enable_registration: true,
            description: "".to_string(),
        };
        let info = mock_info("creator", &coins(0, "usei"));
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let name = String::from("alice");
        let duration: u64 = 24 * 3600 * 365;
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::RentPrice {
                name: name.clone(),
                duration: duration.clone(),
            },
        )
        .unwrap();
        let rent_price_response: RentPriceResponse = from_binary(&res).unwrap();

        // Sent half of required rent
        let half = rent_price_response.price.u128() / 2;
        let info = mock_info("alice", &coins(half, "usei"));
        let msg = ExecuteMsg::Renew {
            name: name.clone(),
            duration: duration.clone(),
        };

        let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        assert_eq!(
            err,
            ContractError::InsufficientFund {
                amount: Uint128::from(half),
                required: Uint128::from(rent_price_response.price.u128())
            }
        );
    }

    #[test]
    fn test_withdraw() {
        let mut deps = mock_dependencies(&[]);
        let registrar_address = String::from("registrar_address");
        let reverse_registrar_address = String::from("reverse_registrar_address");
        let msg = InstantiateMsg {
            registrar_address: registrar_address.clone(),
            reverse_registrar_address: reverse_registrar_address.clone(),
            min_registration_duration: 24 * 3600 * 365,

            tier1_price: 640_000_000u64,
            tier2_price: 160_000_000u64,
            tier3_price: 5_000_000u64,
            whitelist_price: 640_000_000u64,
            referal_percentage: (20, 40),
            enable_registration: true,
            description: "".to_string(),
        };
        let info = mock_info("creator", &coins(0, "usei"));
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("creator", &coins(0, "usei"));
        let msg = ExecuteMsg::Withdraw {};

        // Zero balance
        let bank_send_message: CosmosMsg = CosmosMsg::Bank(BankMsg::Send {
            to_address: "creator".to_string(),
            amount: vec![Coin {
                denom: "usei".to_string(),
                amount: Uint128::from(0 as u32),
            }],
        });

        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        assert_eq!(res.messages.len(), 1); // Renew
        assert_eq!(res.messages[0].msg, bank_send_message);
    }

    #[test] // Should return error if withdraw with non-owner
    fn test_withdraw_not_owner() {
        let mut deps = mock_dependencies(&[]);
        let registrar_address = String::from("registrar_address");
        let reverse_registrar_address = String::from("reverse_registrar_address");
        let msg = InstantiateMsg {
            registrar_address: registrar_address.clone(),
            reverse_registrar_address: reverse_registrar_address.clone(),
            min_registration_duration: 24 * 3600 * 365,

            tier1_price: 640_000_000u64,
            tier2_price: 160_000_000u64,
            tier3_price: 5_000_000u64,
            whitelist_price: 640_000_000u64,
            referal_percentage: (20, 40),
            enable_registration: true,
            description: "".to_string(),
        };
        let info = mock_info("creator", &coins(0, "usei"));
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("alice", &coins(0, "usei"));
        let msg = ExecuteMsg::Withdraw {};

        let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        assert_eq!(
            err,
            ContractError::NotOwner {
                sender: String::from("alice"),
                owner: String::from("creator")
            }
        );
    }

    #[test]
    fn test_set_config() {
        let mut deps = mock_dependencies(&[]);
        let registrar_address = String::from("registrar_address");
        let reverse_registrar_address = String::from("reverse_registrar_address");
        let msg = InstantiateMsg {
            registrar_address: registrar_address.clone(),
            reverse_registrar_address: reverse_registrar_address.clone(),
            min_registration_duration: 24 * 3600 * 365,

            tier1_price: 640_000_000u64,
            tier2_price: 160_000_000u64,
            tier3_price: 5_000_000u64,
            whitelist_price: 640_000_000u64,
            referal_percentage: (20, 40),
            enable_registration: true,
            description: "".to_string(),
        };
        let info = mock_info("creator", &coins(0, "usei"));
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::SetConfig {
            min_registration_duration: 24 * 3600 * 365 * 2,
            tier1_price: 6_000_000u64,
            tier2_price: 5_000_000u64,
            tier3_price: 4_000_000u64,
            enable_registration: true,
            description: "".to_string(),
            registrar_address: String::from("new_registrar_address"),
            reverse_registrar_address: String::from("reverse_registrar_address"),
            owner: String::from("new_owner"),
        };
        execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = QueryMsg::Owner {};
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let res: OwnerResponse = from_binary(&res).unwrap();
        assert_eq!(
            res,
            OwnerResponse {
                owner: Addr::unchecked(String::from("new_owner"))
            }
        );

        let msg = QueryMsg::Registrar {};
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let res: RegistrarResponse = from_binary(&res).unwrap();
        assert_eq!(
            res,
            RegistrarResponse {
                registrar_address: Addr::unchecked(String::from("new_registrar_address")),
            }
        );

        let msg = QueryMsg::MinRegistrationDuration {};
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let res: MinRegistrationDurationResponse = from_binary(&res).unwrap();
        assert_eq!(
            res,
            MinRegistrationDurationResponse {
                duration: 24 * 3600 * 365 * 2,
            }
        );

        let msg = QueryMsg::GetPrice {};
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let res: PriceResponse = from_binary(&res).unwrap();
        assert_eq!(
            res,
            PriceResponse {
                tier1_price: 6_000_000u64,
                tier2_price: 5_000_000u64,
                tier3_price: 4_000_000u64,
                whitelist_price: 640_000_000u64,
            }
        );
    }

    #[test] // Should return error if set config with non-owner
    fn test_cannot_set_config_if_not_owner() {
        let mut deps = mock_dependencies(&[]);
        let registrar_address = String::from("registrar_address");
        let reverse_registrar_address = String::from("reverse_registrar_address");
        let reverse_registrar_address = String::from("reverse_registrar_address");
        let msg = InstantiateMsg {
            registrar_address: registrar_address.clone(),
            reverse_registrar_address: reverse_registrar_address.clone(),
            min_registration_duration: 24 * 3600 * 365,

            tier1_price: 640_000_000u64,
            tier2_price: 160_000_000u64,
            tier3_price: 5_000_000u64,
            whitelist_price: 640_000_000u64,
            referal_percentage: (20, 40),
            enable_registration: true,
            description: "".to_string(),
        };
        let info = mock_info("creator", &coins(0, "usei"));
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::SetConfig {
            min_registration_duration: 24 * 3600 * 365 * 2,
            tier1_price: 6_000_000u64,
            tier2_price: 5_000_000u64,
            tier3_price: 4_000_000u64,
            enable_registration: true,
            description: "".to_string(),
            registrar_address: String::from("new_registrar_address"),
            reverse_registrar_address: String::from("reverse_registrar_address"),
            owner: String::from("new_owner"),
        };
        let info = mock_info("alice", &coins(0, "usei"));
        let err = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();

        assert_eq!(
            err,
            ContractError::NotOwner {
                sender: String::from("alice"),
                owner: String::from("creator")
            }
        );
    }

    #[test]
    fn test_set_config_transfer_owner() {
        let mut deps = mock_dependencies(&[]);
        let registrar_address = String::from("registrar_address");
        let reverse_registrar_address = String::from("reverse_registrar_address");
        let msg = InstantiateMsg {
            registrar_address: registrar_address.clone(),
            reverse_registrar_address: reverse_registrar_address.clone(),
            min_registration_duration: 24 * 3600 * 365,

            tier1_price: 640_000_000u64,
            tier2_price: 160_000_000u64,
            tier3_price: 5_000_000u64,
            whitelist_price: 640_000_000u64,
            referal_percentage: (20, 40),
            enable_registration: true,
            description: "".to_string(),
        };
        let info = mock_info("creator", &coins(0, "usei"));
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        let msg = ExecuteMsg::SetConfig {
            min_registration_duration: 24 * 3600 * 365 * 2,
            tier1_price: 6_000_000u64,
            tier2_price: 5_000_000u64,
            tier3_price: 4_000_000u64,
            enable_registration: true,
            description: "".to_string(),
            registrar_address: String::from("new_registrar_address"),
            reverse_registrar_address: String::from("reverse_registrar_address"),
            owner: String::from("new_owner"),
        };
        execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = QueryMsg::Owner {};
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let res: OwnerResponse = from_binary(&res).unwrap();
        assert_eq!(
            res,
            OwnerResponse {
                owner: Addr::unchecked(String::from("new_owner"))
            }
        );

        let msg = ExecuteMsg::SetConfig {
            min_registration_duration: 24 * 3600 * 365 * 2,
            tier1_price: 6_000_000u64,
            tier2_price: 5_000_000u64,
            tier3_price: 4_000_000u64,
            enable_registration: true,
            description: "".to_string(),
            registrar_address: String::from("new_registrar_address"),
            reverse_registrar_address: String::from("reverse_registrar_address"),
            owner: String::from("creator"),
        };
        let info = mock_info("new_owner", &coins(0, "usei"));
        execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = QueryMsg::Owner {};
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let res: OwnerResponse = from_binary(&res).unwrap();
        assert_eq!(
            res,
            OwnerResponse {
                owner: Addr::unchecked(String::from("creator"))
            }
        );
    }
}
