use crate::error::ContractError;
use crate::handler::{
    claim, claim_for_addr, get_config, get_reverse_record, set_config, set_name, set_name_for_addr,
};
use crate::state::{Config, CONFIG};
use cosmwasm_std::{entry_point, to_binary, Binary, Deps, StdResult};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use dotlabs::reverse_registar::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let sender = deps.api.addr_canonicalize(info.sender.as_str())?;
    let temp_resolver = deps.api.addr_canonicalize(env.contract.address.as_str())?;
    CONFIG.save(
        deps.storage,
        &Config {
            resolver_address: temp_resolver, // This will be set as resolver address once deployed
            owner: sender.clone(),
            registry_address: deps.api.addr_canonicalize(msg.registry_address.as_str())?,
        },
    )?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ClaimForAddr {
            address,
            owner,
            resolver,
        } => claim_for_addr(deps, env, info, address, owner, Some(resolver)),
        ExecuteMsg::Claim { owner } => claim(deps, env, info, owner),
        ExecuteMsg::SetName { name } => set_name(deps, env, info, name),
        ExecuteMsg::SetNameForAddr {
            address,
            owner,
            resolver,
            name,
        } => set_name_for_addr(deps, env, info, address, owner, resolver, name),
        ExecuteMsg::SetConfig {
            resolver_address,
            registry_address,
            owner,
        } => set_config(deps, env, info, resolver_address, registry_address, owner),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&get_config(deps)?),
        QueryMsg::GetReverseRecord { node } => to_binary(&get_reverse_record(deps, node)?),
    }
}
