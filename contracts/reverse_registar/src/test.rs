mod tests {
    use crate::contract::{execute, instantiate};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, to_binary, Api, CosmosMsg, WasmMsg};
    use dotlabs::reverse_registar::{ExecuteMsg, InstantiateMsg};
    use dotlabs::utils::{get_label_from_name, namehash};

    // #[test]
    // fn test_xxxxxxxxx() {
    //     let mut deps = mock_dependencies();

    //     let address = "sei12klaltyqvg2j6v034jwdxrk5n4242ttsztzqqk".to_string();
    //     let xxx = deps.api.addr_canonicalize(address.as_str()).unwrap();
    //     let yyy = deps.api.addr_humanize(&xxx).unwrap().to_string();

    //     assert_eq!(xxx.to_string(), yyy);
    // }

    #[test]
    fn test_reverse_record() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            resolver_address: String::from("resolver_address"),
            registry_address: String::from("registry_address"),
        };
        let info = mock_info("creator", &coins(0, "uusd"));
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let address = String::from("alice_address");
        let msg = ExecuteMsg::SetNameForAddr {
            address: address.clone(),
            owner: "alice".to_string(),
            resolver: Some(String::from("resolver_address")),
            name: String::from("alice.eth"),
        };
        let result = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let resolver_set_name_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: "resolver_address".to_string(),
            msg: to_binary(&dotlabs::resolver::ExecuteMsg::SetName {
                address: address.clone(),
                name: String::from("alice.eth"),
            })
            .unwrap(),
            funds: vec![],
        });

        assert_eq!(result.messages[0].msg, resolver_set_name_msg);
    }
}
