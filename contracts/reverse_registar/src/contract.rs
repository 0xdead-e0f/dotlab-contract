use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use dotlabs::reverse_registar::{InstantiateMsg, ExecuteMsg};
use cosmwasm_std::entry_point;
use crate::error::ContractError;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    
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
        ExecuteMsg::ClaimForAddr { address, owner, resolver } => todo!(),
        ExecuteMsg::Claim { owner } => todo!(),
        ExecuteMsg::ClaimForAddrWithSignature { address, owner, resolver, relayer, signature_expiry, signature } => todo!(),
        ExecuteMsg::ClaimWithResolver { owner, resolver } => todo!(),
        ExecuteMsg::SetName { name } => todo!(),
        ExecuteMsg::SetNameForAddr { address, owner, resolver, name } => todo!(),
        ExecuteMsg::SetNameForAddrWithSignature { address, owner, resolver, relayer, signature_expiry, signature, name } => todo!(),
        ExecuteMsg::SetConfig { interface_id, registry_address, owner } => todo!(),
    }
}