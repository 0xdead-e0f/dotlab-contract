use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, CosmosMsg, WasmMsg, to_binary};
use dotlabs::utils::get_label_from_name;
use crate::{error::ContractError, state::CONFIG};
use dotlabs::registry::ExecuteMsg as RegistryExecuteMsg;

pub fn claim_for_addr(
    deps: DepsMut, 
    _env: Env, 
    info: MessageInfo, 
    address: String, 
    owner: String, 
    resolver: String
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let registry_address = deps.api.addr_humanize(&config.registry_address)?;

    let labelHash = get_label_from_name(&address);
    let set_subnode_owner_registry_msg: CosmosMsg<C> = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: registry_address.to_string(),
        msg: to_binary(&RegistryExecuteMsg::SetSubnodeOwner {
            node: config.addr_reverse_node,
            label: labelHash,
            owner: owner.clone(),
        })?,
        funds: vec![],
    });
}

pub fn set_name_for_addr(
    deps: DepsMut, 
    _env: Env, 
    info: MessageInfo, 
    address: String, 
    owner: String, 
    resolver: String,
    name: String,
) -> Result<Response, ContractError> {

}