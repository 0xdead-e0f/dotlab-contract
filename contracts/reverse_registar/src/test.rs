mod tests {
    use crate::contract::{execute, instantiate};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, to_binary, CosmosMsg, WasmMsg};
    use dotlabs::reverse_registar::{ExecuteMsg, InstantiateMsg};
    // use dotlabs::utils::{get_label_from_name, namehash};

    // #[test]
    // fn test_xxxxxxxxx() {
    //     let mut deps = mock_dependencies();

    //     let name = String::from("erwrew.sei");
    //     let namehash = namehash(name.as_str());

    //     let xxx = to_binary(QueryMsg::GetTextData{vec![ 13, 220, 91, 142, 243, 31, 92, 188, 110, 96, 4, 240, 121, 191, 85, 243, 214, 216, 26, 143, 90, 147, 151, 32, 115, 1, 40, 206, 184, 230, 48, 173], String::from("telegram")}).unwrap();
    //     // assert_eq!(namehash, vec![0]);
    //     assert_eq!(xxx, to_binary("xxx").unwrap());
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
