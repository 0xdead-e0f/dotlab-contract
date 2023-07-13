mod tests {
    use crate::contract::{execute, instantiate};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, to_binary, CosmosMsg, WasmMsg};
    use dotlabs::reverse_registar::{ExecuteMsg, InstantiateMsg};
    use dotlabs::utils::{get_label_from_name, namehash};

    // #[test]
    // fn test_xxxxxxxxx() {
    //     let address = "sei12klaltyqvg2j6v034jwdxrk5n4242ttsztzqqk".to_string();
    //     let labelhash = get_label_from_name(&address);
    //     println!("{:?}", namehash("addr.reverse"));
    //     let reverse_node1 = namehash((address.clone() + &".addr.reverse".to_string()).as_str());
    //     let reverse_node = namehash([address, ".addr.reverse".to_string()].concat().as_str());

    //     assert_eq!(reverse_node1, reverse_node);
    //     let kk = [namehash("addr.reverse"), labelhash].concat();
    //     let kk = dotlabs::utils::keccak256(&kk);
    //     assert_eq!(reverse_node, kk);
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
