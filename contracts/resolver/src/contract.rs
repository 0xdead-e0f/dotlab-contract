use crate::error::ContractError;
use crate::handler::{
    get_config, multicall, query_address, query_avatar, query_content_hash, query_name,
    query_sei_address, query_text_data, set_address, set_avatar, set_config, set_content_hash,
    set_name, set_sei_address, set_text_data,
};
use crate::state::{Config, CONFIG};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use dotlabs::resolver::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let registry_address = deps.api.addr_canonicalize(msg.registry_address.as_str())?;
    let trusted_reverse_registrar = deps
        .api
        .addr_canonicalize(&msg.trusted_reverse_registrar.as_str())?;
    let trusted_controller = deps
        .api
        .addr_canonicalize(&msg.trusted_controller.as_str())?;
    let sender = deps.api.addr_canonicalize(info.sender.as_str())?;
    CONFIG.save(
        deps.storage,
        &Config {
            interface_id: msg.interface_id,
            registry_address,
            trusted_reverse_registrar,
            trusted_controller,
            owner: sender.clone(),
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
        ExecuteMsg::SetAddress { node, address } => set_address(deps, env, info, node, address),
        ExecuteMsg::SetAvatar { node, avatar_uri } => set_avatar(deps, env, info, node, avatar_uri),
        ExecuteMsg::SetName { address, name } => set_name(deps, env, info, address, name),
        ExecuteMsg::SetSeiAddress { node, address } => {
            set_sei_address(deps, env, info, node, address)
        }
        ExecuteMsg::SetTextData { node, key, value } => {
            set_text_data(deps, env, info, node, key, value)
        }
        ExecuteMsg::SetContentHash { node, hash } => set_content_hash(deps, env, info, node, hash),
        ExecuteMsg::SetConfig {
            interface_id,
            registry_address,
            trusted_reverse_registrar,
            trusted_controller,
            owner,
        } => set_config(
            deps,
            env,
            info,
            interface_id,
            registry_address,
            trusted_reverse_registrar,
            trusted_controller,
            owner,
        ),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAddress { node } => to_binary(&query_address(deps, env, node)?),
        QueryMsg::GetAvatar { node } => to_binary(&query_avatar(deps, env, node)?),
        QueryMsg::GetName { node } => to_binary(&query_name(deps, env, node)?),
        QueryMsg::GetSeiAddress { node } => to_binary(&query_sei_address(deps, env, node)?),
        QueryMsg::GetTextData { node, key } => to_binary(&query_text_data(deps, env, node, key)?),
        QueryMsg::GetContentHash { node } => to_binary(&query_content_hash(deps, env, node)?),
        QueryMsg::GetConfig {} => to_binary(&get_config(deps)?),
        QueryMsg::Multicall { queries } => to_binary(&multicall(deps, env, queries)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
