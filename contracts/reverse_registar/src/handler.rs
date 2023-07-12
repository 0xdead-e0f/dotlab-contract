use crate::{error::ContractError, state::CONFIG};
use cosmwasm_std::{to_binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, WasmMsg, Deps, StdResult, WasmQuery, QueryRequest};
use dotlabs::registry::ExecuteMsg as RegistryExecuteMsg;
use dotlabs::resolver::ExecuteMsg as ResolverMsg;
use dotlabs::reverse_registar::ConfigResponse;
use dotlabs::registry::QueryMsg as RegistryQueryMsg;
use dotlabs::registry::RecordResponse as RegistryRecordResponse;
use dotlabs::utils::{get_label_from_name, namehash};
use dotlabs::OUR_COIN_TYPE;

pub fn set_config(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    resolver_address: String,
    registry_address: String,
    owner: String,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    config.owner = deps.api.addr_canonicalize(owner.as_str())?;
    config.resolver_address = deps.api.addr_canonicalize(resolver_address.as_str())?;
    config.registry_address = deps.api.addr_canonicalize(&registry_address.as_str())?;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::default())
}

pub fn get_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    let owner = deps.api.addr_humanize(&config.owner)?;
    let resolver = deps.api.addr_humanize(&config.resolver_address)?;
    let registry = deps.api.addr_humanize(&config.registry_address)?;
    Ok(ConfigResponse {
        registry_address: registry,
        resolver_address: resolver,
        owner,
    })
}

pub fn get_reverse_record(deps: Deps, node: Vec<u8>) -> StdResult<RegistryRecordResponse> {
    let config = CONFIG.load(deps.storage)?;
    let registry_address = config.registry_address;
    let get_registry_response: RegistryRecordResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: registry_address.to_string(),
            msg: to_binary(&RegistryQueryMsg::GetRecordByNode { node })?,
        }))?;
    Ok(get_registry_response)
    // let registry_address = String::from(get_registry_response.registry);
}

pub fn claim_for_addr(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    address: String,
    owner: String,
    resolver: Option<String>,
) -> Result<Response, ContractError> {
    let mut messages: Vec<CosmosMsg> = vec![];

    let config = CONFIG.load(deps.storage)?;
    let registry_address = config.registry_address.clone().to_string();

    let labelhash = get_label_from_name(&address);
    // let reverse_node = namehash((address + &".addr.reverse".to_string()).as_str());
    let set_subnode_owner_registry_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: registry_address.to_string(),
        msg: to_binary(&RegistryExecuteMsg::SetSubnodeOwner {
            node: namehash(&"addr.reverse".to_string()),
            owner: owner.clone(),
            label: labelhash,
        })?,
        funds: vec![],
    });
    messages.push(set_subnode_owner_registry_msg);
    let set_resolver_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: registry_address.to_string(),
        msg: to_binary(&RegistryExecuteMsg::SetResolver {
            node: get_label_from_name(&".addr.reverse".to_string()),
            resolver: resolver,
        })?,
        funds: vec![],
    });
    messages.push(set_resolver_msg);
    Ok(Response::new()
        .add_messages(messages)
    )

}

pub fn claim(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    owner: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let resolver = config.resolver_address;
    claim_for_addr(
        deps,
        env,
        info.clone(),
        info.sender.to_string(),
        owner,
        Some(resolver.to_string()),
    )
}

pub fn set_name_for_addr(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    address: String,
    owner: String,
    resolver: Option<String>,
    name: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let node = claim_for_addr(deps, env, info, address.clone(), owner, resolver.clone())?;
    let resolver_address ;

    if let Some(resolver) = resolver {
        resolver_address = resolver.clone()
    } else {
        resolver_address = config.resolver_address.clone().to_string();
    }

    let set_name_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: resolver_address.to_string(),
        msg: to_binary(&ResolverMsg::SetName {
            address,
            name,
            coin_type: OUR_COIN_TYPE,
        })?,
        funds: vec![],
    });
    Ok(Response::new()
        .add_messages(vec![set_name_msg])
        .add_attributes(node.attributes)
    )
}

pub fn set_name(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let resolver = config.resolver_address;
    set_name_for_addr(
        deps,
        env,
        info.clone(),
        info.sender.to_string(),
        info.sender.to_string(),
        Some(resolver.to_string()),
        name,
    )
}
