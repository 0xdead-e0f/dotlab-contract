use crate::error::ContractError;
use crate::handler::{
    add_whitelist, add_whitelist_by_owner, get_is_valid_name, get_min_registration_duration,
    get_node_info_from_name, get_nodehash_from_name, get_owner, get_price, get_registrar,
    get_rent_price, get_token_id_from_name, owner_register, owner_renew, referal_register,
    register, renew, set_config, set_enable_registration, set_referal_percentage,
    set_whitelist_price, withdraw,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{Config, CONFIG};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use sei_cosmwasm::SeiQueryWrapper;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let registrar_address = deps.api.addr_canonicalize(msg.registrar_address.as_str())?;
    let reverse_registrar_address = deps
        .api
        .addr_canonicalize(msg.reverse_registrar_address.as_str())?;
    let owner = deps.api.addr_canonicalize(info.sender.as_str())?;

    CONFIG.save(
        deps.storage,
        &Config {
            min_registration_duration: msg.min_registration_duration,
            tier1_price: msg.tier1_price,
            tier2_price: msg.tier2_price,
            tier3_price: msg.tier3_price,
            whitelist_price: msg.whitelist_price,
            referal_percentage: msg.referal_percentage,
            enable_registration: msg.enable_registration,
            registrar_address,
            reverse_registrar_address,
            owner,
            description: msg.description,
        },
    )?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<SeiQueryWrapper>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register {
            name,
            owner,
            duration,
            resolver,
            address,
            reverse_record,
        } => register(
            deps,
            env,
            info,
            name,
            owner,
            duration,
            resolver,
            address,
            reverse_record,
        ),
        ExecuteMsg::ReferalRegister {
            name,
            owner,
            duration,
            resolver,
            address,
            referer,
            reverse_record,
        } => referal_register(
            deps,
            env,
            info,
            name,
            owner,
            duration,
            resolver,
            address,
            referer,
            reverse_record,
        ),
        ExecuteMsg::Renew { name, duration } => renew(deps, env, info, name, duration),

        // Only owner
        ExecuteMsg::SetConfig {
            min_registration_duration,
            tier1_price,
            tier2_price,
            tier3_price,
            registrar_address,
            reverse_registrar_address,
            owner,
            enable_registration,
            description,
        } => set_config(
            deps,
            env,
            info,
            min_registration_duration,
            tier1_price,
            tier2_price,
            tier3_price,
            registrar_address,
            reverse_registrar_address,
            owner,
            enable_registration,
            description,
        ),
        ExecuteMsg::Withdraw {} => withdraw(deps, env, info),
        ExecuteMsg::OwnerRegister {
            name,
            owner,
            duration,
            resolver,
            address,
            reverse_record,
        } => owner_register(
            deps,
            env,
            info,
            name,
            owner,
            duration,
            resolver,
            address,
            reverse_record,
        ),
        ExecuteMsg::OwnerRenew { name, duration } => owner_renew(deps, env, info, name, duration),
        ExecuteMsg::SetEnableRegistration {
            enable_registration,
        } => set_enable_registration(deps, env, info, enable_registration),
        ExecuteMsg::AddWhiteList { ensname } => add_whitelist(deps, env, info, &ensname),
        ExecuteMsg::AddWhiteListByOwner {
            ensname,
            referal_percentage,
        } => add_whitelist_by_owner(deps, env, info, &ensname, referal_percentage),
        ExecuteMsg::SetReferalPercentage {
            normal_percentage,
            whitelist_percentage,
        } => set_referal_percentage(deps, env, info, normal_percentage, whitelist_percentage),
        ExecuteMsg::SetWhitelistPrice { price } => set_whitelist_price(deps, env, info, price),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<SeiQueryWrapper>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::RentPrice { name, duration } => to_binary(&get_rent_price(deps, name, duration)?),
        QueryMsg::MinRegistrationDuration {} => to_binary(&get_min_registration_duration(deps)?),
        QueryMsg::GetPrice {} => to_binary(&get_price(deps)?),
        QueryMsg::Registrar {} => to_binary(&get_registrar(deps)?),
        QueryMsg::Owner {} => to_binary(&get_owner(deps)?),

        QueryMsg::IsValidName { name } => to_binary(&get_is_valid_name(&name)?),
        QueryMsg::GetTokenId { name } => to_binary(&get_token_id_from_name(&name)?),
        QueryMsg::GetNodehash { name } => to_binary(&get_nodehash_from_name(deps, &name)?),
        QueryMsg::GetNodeInfo { name } => to_binary(&get_node_info_from_name(deps, &name)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
