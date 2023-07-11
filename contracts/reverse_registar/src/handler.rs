use crate::{error::ContractError, state::CONFIG};
use cosmwasm_std::{to_binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, WasmMsg};
use dotlabs::registry::ExecuteMsg as RegistryExecuteMsg;
use dotlabs::utils::get_label_from_name;

pub fn claim_for_addr(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    address: String,
    owner: String,
    resolver: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let registry_address = deps.api.addr_humanize(&config.registry_address)?;
    let contract_address = deps.api.addr_canonicalize(env.contract.address.as_str())?;

    let labelHash = get_label_from_name(&address);
    let set_subnode_owner_registry_msg: CosmosMsg<C> = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: registry_address.to_string(),
        msg: to_binary(&RegistryExecuteMsg::SetSubnodeOwner {
            node: config.addr_reverse_node,
            label: labelHash,
            owner: contract_address.clone(),
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
