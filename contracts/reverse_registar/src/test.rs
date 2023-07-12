mod tests {
    use crate::contract::{execute, instantiate, query};
    use crate::error::ContractError;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, Addr, Response, WasmMsg, CosmosMsg, to_binary};
    use dotlabs::registry::RecordResponse;
    use dotlabs::{resolver, OUR_COIN_TYPE};
    use dotlabs::reverse_registar::{
        ExecuteMsg, InstantiateMsg, QueryMsg,
    };
    use dotlabs::utils::{
        convert_namehash_to_hex_string, get_label_from_name, keccak256, namehash,
    };

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
             name: String::from("alice.eth") 
        }; 
        let result = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        
        // let reverse_node = namehash((address + &".addr.reverse".to_string()).as_str());

        // let query_msg = QueryMsg::GetReverseRecord { 
        //     node: reverse_node
        // };

        // let res = query(deps.as_ref(), mock_env(), info, query_msg.clone()).unwrap();

        // let res: RecordResponse = from_binary(&res).unwrap();

        let resolver_set_name_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: "resolver_address".to_string(),
            msg: to_binary(&dotlabs::resolver::ExecuteMsg::SetName { 
                address: address.clone(),
                coin_type: OUR_COIN_TYPE,
                name: String::from("alice.eth")
            })
            .unwrap(),
            funds: vec![],
        });

        assert_eq!(
            result.messages[0].msg,
            resolver_set_name_msg
        );
    }

}