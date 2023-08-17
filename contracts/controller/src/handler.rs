use crate::error::ContractError;
use crate::msg::{
    IsValidNameResponse, MinRegistrationDurationResponse, NodeInfoResponse, NodehashResponse,
    OwnerResponse, PriceResponse, RegistrarResponse, RentPriceResponse, TokenIdResponse,
};
use crate::state::{CONFIG, REGISTER_FEE_DENOM, WHITELIST};
use cosmwasm_std::{
    to_binary, BalanceResponse, BankMsg, BankQuery, Coin, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, QueryRequest, Response, StdError, StdResult, Uint128, WasmMsg, WasmQuery, Addr,
};
use hex;
// use terraswap::asset::{Asset, AssetInfo};
use dotlabs::registrar::{
    ConfigResponse, ExecuteMsg as RegistrarExecuteMsg, Extension, GetBaseNodeResponse,
    GetRegistryResponse, IsAvailableResponse, QueryMsg as RegistrarQueryMsg,
};
use dotlabs::registry::QueryMsg as RegistryQueryMsg;
use dotlabs::registry::{ExecuteMsg as RegistryExecuteMsg, RecordResponse};
use dotlabs::resolver::ExecuteMsg as ResolverExecuteMsg;
use dotlabs::reverse_registar::ExecuteMsg as ReverseRegistrarExecuteMsg;
use dotlabs::utils::{get_label_from_name, get_token_id_from_label, keccak256};
use unicode_segmentation::UnicodeSegmentation;

fn only_owner(deps: Deps, info: &MessageInfo) -> Result<bool, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let sender = deps.api.addr_canonicalize(info.sender.as_str())?;
    if sender != config.owner {
        return Err(ContractError::NotOwner {
            sender: info.sender.to_string(),
            owner: deps.api.addr_humanize(&config.owner)?.to_string(),
        });
    }
    Ok(true)
}

pub fn withdraw(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    only_owner(deps.as_ref(), &info)?;

    let balance_response: BalanceResponse =
        deps.querier.query(&QueryRequest::Bank(BankQuery::Balance {
            address: env.contract.address.to_string(),
            denom: String::from(REGISTER_FEE_DENOM),
        }))?;

    let amount = balance_response.amount.amount;

    // let total_asset = Asset {
    //     info: AssetInfo::NativeToken {
    //         denom: balance_response.amount.denom,
    //     },
    //     amount,
    // };
    // let message = total_asset.into_msg(&deps.querier, info.sender);

    let message = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![balance_response.amount],
    };

    Ok(Response::new()
        .add_message(message)
        .add_attribute("method", "withdraw")
        .add_attribute("amount", amount))
}

pub fn set_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    min_registration_duration: u64,
    tier1_price: u64,
    tier2_price: u64,
    tier3_price: u64,
    registrar_address: String,
    reverse_registrar_address: String,
    owner: String,
    enable_registration: bool,
    description: String,
) -> Result<Response, ContractError> {
    only_owner(deps.as_ref(), &info)?;
    let mut config = CONFIG.load(deps.storage)?;

    let registrar_address = deps.api.addr_canonicalize(registrar_address.as_str())?;
    let reverse_registrar_address = deps
        .api
        .addr_canonicalize(reverse_registrar_address.as_str())?;
    let owner = deps.api.addr_canonicalize(owner.as_str())?;

    config.min_registration_duration = min_registration_duration;
    config.tier1_price = tier1_price;
    config.tier2_price = tier2_price;
    config.tier3_price = tier3_price;
    config.registrar_address = registrar_address.clone();
    config.reverse_registrar_address = reverse_registrar_address.clone();
    config.owner = owner.clone();
    config.enable_registration = enable_registration;
    config.description = description;

    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("method", "set_config")
        .add_attribute(
            "min_registration_duration",
            min_registration_duration.to_string(),
        )
        .add_attribute("tier1_price", tier1_price.to_string())
        .add_attribute("tier2_price", tier2_price.to_string())
        .add_attribute("tier3_price", tier3_price.to_string())
        .add_attribute("registrar_address", registrar_address.clone().to_string())
        .add_attribute(
            "enable_registration",
            enable_registration.clone().to_string(),
        )
        .add_attribute("owner", owner.clone().to_string()))
}

pub fn set_enable_registration(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    enable_registration: bool,
) -> Result<Response, ContractError> {
    only_owner(deps.as_ref(), &info)?;
    let mut config = CONFIG.load(deps.storage)?;
    config.enable_registration = enable_registration;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("method", "set_enable_registration")
        .add_attribute("enable_registration", enable_registration.to_string()))
}

pub fn set_whitelist_price(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    price: u64,
) -> Result<Response, ContractError> {
    only_owner(deps.as_ref(), &info)?;
    let mut config = CONFIG.load(deps.storage)?;
    config.whitelist_price = price;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("method", "set_whitelist_price")
        .add_attribute("whitelist_price", price.to_string()))
}

pub fn set_referal_percentage(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    normal_percentage: u32,
    whitelist_percentage: u32,
) -> Result<Response, ContractError> {
    if normal_percentage > 50u32 {
        return Err(ContractError::ReferalPercentageError {
            description: Some(String::from("Normal percentage must be in 0~50")),
        });
    }

    if whitelist_percentage > 50u32 || whitelist_percentage < 20u32 {
        return Err(ContractError::ReferalPercentageError {
            description: Some(String::from("Whitelist percentage must be in 20~50")),
        });
    }

    only_owner(deps.as_ref(), &info)?;
    let mut config = CONFIG.load(deps.storage)?;
    config.referal_percentage = (normal_percentage, whitelist_percentage);
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("method", "set_referal_percentage")
        .add_attribute("normal_percentage", normal_percentage.to_string())
        .add_attribute("whitelist_percentage", whitelist_percentage.to_string()))
}

// pub fn commit(
//     deps: DepsMut,
//     env: Env,
//     _info: MessageInfo,
//     commitment: String,
// ) -> Result<Response, ContractError> {
//     validate_enable_registration(deps.as_ref())?;

//     let config = CONFIG.load(deps.storage)?;

//     let last_commit_time = COMMITMENTS
//         .may_load(deps.storage, commitment.clone())?
//         .unwrap_or(0);
//     let current = env.block.time.seconds();

//     if last_commit_time + config.max_commitment_age > current {
//         return Err(ContractError::RecommitTooEarly {
//             commit_expired: last_commit_time + config.max_commitment_age,
//             current,
//         });
//     }

//     COMMITMENTS.save(deps.storage, commitment.clone(), &current)?;

//     Ok(Response::new()
//         .add_attribute("method", "commit")
//         .add_attribute("commitment", commitment))
// }

fn validate_name(deps: Deps, name: String) -> Result<(), ContractError> {
    if !get_is_valid_name(&name)?.is_valid_name {
        return Err(ContractError::InvalidName {});
    }

    if !is_available_name(deps, &name)? || !get_is_valid_name(&name)?.is_valid_name {
        return Err(ContractError::UnavailabledName {});
    }
    Ok(())
}

// pub fn consume_commitment(
//     deps: DepsMut,
//     env: Env,
//     commitment: String,
// ) -> Result<(), ContractError> {
//     let config = CONFIG.load(deps.storage)?;
//     let commit_time = COMMITMENTS.may_load(deps.storage, commitment.clone())?;
//     if commit_time.is_none() {
//         return Err(ContractError::ConsumeNonexistCommitment { commitment });
//     }

//     let commit_time = commit_time.unwrap();
//     let current = env.block.time.seconds();
//     if commit_time + config.min_commitment_age > current
//         || commit_time + config.max_commitment_age < current
//     {
//         return Err(ContractError::CommitmentIsTooEarlyOrExpired {
//             commit_expired: commit_time + config.max_commitment_age,
//             commit_matured: commit_time + config.min_commitment_age,
//             current,
//         });
//     }

//     COMMITMENTS.remove(deps.storage, commitment);
//     Ok(())
// }

pub fn get_cost(deps: Deps, name: String, duration: u64) -> Result<Uint128, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let min_duration = config.min_registration_duration;
    let name_length = name.graphemes(true).count();
    if name_length < 3 {
        return Err(ContractError::NameTooShort {});
    }
    if duration < min_duration {
        return Err(ContractError::DurationTooShort {
            input_duration: duration,
            min_duration: min_duration,
        });
    }

    let base_cost = match name_length {
        3 => config.tier1_price,
        4 => config.tier2_price,
        _ => config.tier3_price,
    };
    Ok(Uint128::from(base_cost).multiply_ratio(duration, 31_536_000u64))
}

pub fn get_price(deps: Deps) -> StdResult<PriceResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(PriceResponse {
        tier1_price: config.tier1_price,
        tier2_price: config.tier2_price,
        tier3_price: config.tier3_price,
        whitelist_price: config.whitelist_price,
    })
}

fn _register(
    deps: DepsMut,
    env: Env,
    name: String,
    owner: String,
    duration: u64,
    resolver: Option<String>,
    address: Option<String>,
    reverse_record: bool,
) -> Result<Vec<CosmosMsg>, ContractError> {
    let mut messages: Vec<CosmosMsg> = vec![];

    let config = CONFIG.load(deps.storage)?;
    let registrar_address = deps
        .api
        .addr_humanize(&config.registrar_address)?
        .to_string();

    let label: Vec<u8> = get_label_from_name(&name);
    let token_id = get_token_id_from_label(&label);

    // Register this contract to be temporary owner of the node at registrar
    let register_registrar_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: registrar_address.clone(),
        msg: to_binary(&RegistrarExecuteMsg::<Extension>::Register {
            id: token_id.clone(),
            owner: env.contract.address.to_string(),
            name: name.clone(),
            duration,
            extension: Extension {
                name: name.clone(),
                description: config.description,
            },
        })?,
        funds: vec![],
    });
    messages.push(register_registrar_msg);

    let get_registry_response: GetRegistryResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: registrar_address.clone(),
            msg: to_binary(&RegistrarQueryMsg::<WasmQuery>::GetRegistry {})?,
        }))?;
    let registry_address = String::from(get_registry_response.registry);

    // Set resolver of the node at registry
    let nodehash = get_nodehash(deps.as_ref(), label)?;
    let registry_set_resolver_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: registry_address.clone(),
        msg: to_binary(&RegistryExecuteMsg::SetResolver {
            node: nodehash.clone(),
            resolver: resolver.clone(),
        })?,
        funds: vec![],
    });
    messages.push(registry_set_resolver_msg);

    // Set address at resolver
    if let Some(address) = address.clone() {
        let set_address_resolver_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: resolver.clone().unwrap_or(registry_address),
            msg: to_binary(&ResolverExecuteMsg::SetSeiAddress {
                node: nodehash,
                address: address,
            })?,
            funds: vec![],
        });
        messages.push(set_address_resolver_msg);
    }

    // Transfer ownership of the node to user
    let reclaim_registrar_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: registrar_address.clone(),
        msg: to_binary(&RegistrarExecuteMsg::<Extension>::Reclaim {
            id: token_id.clone(),
            owner: owner.clone(),
        })?,
        funds: vec![],
    });
    messages.push(reclaim_registrar_msg);

    // Transfer ownership of NFT to user
    let transfer_nft_registrar_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: registrar_address.clone(),
        msg: to_binary(&RegistrarExecuteMsg::<Extension>::TransferNft {
            recipient: owner.clone(),
            token_id: token_id,
        })?,
        funds: vec![],
    });
    messages.push(transfer_nft_registrar_msg);

    // if reverse_record {
    //     if let Some(addr) = address {
    //         set_reverse_record(deps, name, addr, resolver, owner.to_string())?;
    //     }
    // }

    if reverse_record {
        if let Some(addr) = address {
            let get_registrar_config_response: ConfigResponse =
                deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
                    contract_addr: registrar_address.clone(),
                    msg: to_binary(&RegistrarQueryMsg::<WasmQuery>::GetConfig {})?,
                }))?;

            let base_name = get_registrar_config_response.base_name;

            let reverse_registrar_address = deps
                .api
                .addr_humanize(&config.reverse_registrar_address)?
                .to_string();

            let set_reverse_record_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: reverse_registrar_address,
                msg: to_binary(&ReverseRegistrarExecuteMsg::SetNameForAddr {
                    address: addr,
                    owner,
                    resolver,
                    name: name + &".".to_string() + base_name.as_str(),
                    // name: name + &".sei".to_string(),
                })?,
                funds: vec![],
            });
            messages.push(set_reverse_record_msg);
        }
    }

    Ok(messages)
}

fn validate_register_fund(
    deps: Deps,
    _env: Env,
    info: MessageInfo,
    name: String,
    duration: u64,
) -> Result<Coin, ContractError> {
    let cost: Uint128 = get_cost(deps, name.clone(), duration)?;
    let base_fund = &Coin {
        denom: String::from(REGISTER_FEE_DENOM),
        amount: Uint128::from(0u128),
    };
    let fund = info
        .funds
        .iter()
        .find(|fund| fund.denom == String::from(REGISTER_FEE_DENOM))
        .unwrap_or(base_fund);
    if fund.amount < cost {
        return Err(ContractError::InsufficientFund {
            amount: fund.amount,
            required: cost,
        });
    }

    Ok(fund.clone())
}

fn validate_enable_registration(deps: Deps) -> Result<(), ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if !config.enable_registration {
        return Err(ContractError::RegistrationDisabled {});
    }
    Ok(())
}

pub fn register(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
    owner: String,
    duration: u64,
    resolver: Option<String>,
    address: Option<String>,
    reverse_record: bool,
) -> Result<Response, ContractError> {
    validate_name(deps.as_ref(), name.clone())?;
    validate_enable_registration(deps.as_ref())?;

    validate_register_fund(
        deps.as_ref(),
        env.clone(),
        info,
        name.clone(),
        duration.clone(),
    )?;

    let messages = _register(
        deps.branch(),
        env.clone(),
        name.clone(),
        owner,
        duration,
        resolver,
        address,
        reverse_record,
    )?;

    let label: Vec<u8> = get_label_from_name(&name);
    let token_id = get_token_id_from_label(&label);
    let nodehash = get_nodehash(deps.as_ref(), label.clone())?;

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("method", "register")
        .add_attribute("name", name)
        .add_attribute("label", format!("{:?}", label.clone()))
        .add_attribute("token_id", token_id)
        .add_attribute("nodehash", format!("{:?}", nodehash)))
}

pub fn referal_register(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
    owner: String,
    duration: u64,
    resolver: Option<String>,
    address: Option<String>,
    referer_ensname: Option<String>,
    reverse_record: bool,
) -> Result<Response, ContractError> {
    validate_name(deps.as_ref(), name.clone())?;
    validate_enable_registration(deps.as_ref())?;

    let fund = validate_register_fund(
        deps.as_ref(),
        env.clone(),
        info.clone(),
        name.clone(),
        duration.clone(),
    )?;

    let mut messages = _register(
        deps.branch(),
        env.clone(),
        name.clone(),
        owner.clone(),
        duration,
        resolver,
        address,
        reverse_record,
    )?;

    let label: Vec<u8> = get_label_from_name(&name);
    let token_id = get_token_id_from_label(&label);
    let nodehash = get_nodehash(deps.as_ref(), label.clone());

    if let Some(referer) = referer_ensname {
        let result = send_referal_funds(deps.as_ref(), env, info, &fund, referer);

        if let Ok(response) = result {
            let referal_owner = response.1;
            let referal_fund_amount = response.2;
            let msg = response.0;

            messages.push(cosmwasm_std::CosmosMsg::Bank(msg));

            return Ok(Response::new()
                .add_messages(messages)
                .add_attribute("method", "register")
                .add_attribute("name", name)
                .add_attribute("label", format!("{:?}", label.clone()))
                .add_attribute("token_id", token_id)
                .add_attribute("nodehash", format!("{:?}", nodehash))
                .add_attribute("referal_owner", referal_owner)
                .add_attribute("referal_fund", referal_fund_amount));
        }
    }

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("method", "register")
        .add_attribute("name", name)
        .add_attribute("label", format!("{:?}", label.clone()))
        .add_attribute("token_id", token_id)
        .add_attribute("nodehash", format!("{:?}", nodehash)))
}

pub fn send_referal_funds(
    deps: Deps,
    _env: Env,
    _info: MessageInfo,
    fund: &Coin,
    referer_ensname: String,
) -> Result<(BankMsg, String, Uint128), ContractError>{
    let config = CONFIG.load(deps.storage)?;
    let registrar_address = deps
        .api
        .addr_humanize(&config.registrar_address)?
        .to_string();

    let label = get_label_from_name(&referer_ensname);
    let nodehash = get_nodehash(deps, label)?;
    let get_registry_response: GetRegistryResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: registrar_address.clone(),
            msg: to_binary(&RegistrarQueryMsg::<WasmQuery>::GetRegistry {})?,
        }))?;
    let registry_address = String::from(get_registry_response.registry);

    let get_record_by_node_response: RecordResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: registry_address.clone(),
            msg: to_binary(&RegistryQueryMsg::GetRecordByNode {
                node: nodehash.clone(),
            })?,
        }))?;

    let referal_owner = get_record_by_node_response.owner;

    let mut referal_fund = fund.clone();
    referal_fund.amount = referal_fund
        .amount
        .multiply_ratio(config.referal_percentage.0 as u128, 100u128);

    let result = is_whitelisted_account(deps, referer_ensname);

    if result.0 {
        referal_fund.amount = referal_fund
            .amount
            .multiply_ratio(result.2 as u128, 100u128);
    }

    let amount = referal_fund.amount;

    let bank_msg = BankMsg::Send {
        to_address: referal_owner.to_string(),
        amount: vec![referal_fund],
    };

    // Ok(Response::new()
    //     .add_attribute("owner", referal_owner)
    //     .add_attribute("fund", amount))
    Ok((bank_msg, referal_owner.to_string(), amount))
}

pub fn is_whitelisted_account(deps: Deps, ensname: String) -> (bool, Vec<u8>, u32) {
    let account = WHITELIST.load(deps.storage, ensname.clone());

    match account {
        Ok(val) => {
            return (true, val.0, val.1);
        }
        Err(_) => return (false, vec![], 0),
    }
}

pub fn owner_register(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
    owner: String,
    duration: u64,
    resolver: Option<String>,
    address: Option<String>,
    reverse_record: bool,
) -> Result<Response, ContractError> {
    only_owner(deps.as_ref(), &info)?;

    if !is_available_name(deps.as_ref(), &name)? {
        return Err(ContractError::UnavailabledName {});
    }

    let messages = _register(
        deps.branch(),
        env.clone(),
        name.clone(),
        owner,
        duration,
        resolver,
        address,
        reverse_record,
    )?;

    let label: Vec<u8> = get_label_from_name(&name);
    let token_id = get_token_id_from_label(&label);
    let nodehash = get_nodehash(deps.as_ref(), label.clone())?;

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("method", "owner_register")
        .add_attribute("name", name)
        .add_attribute("label", format!("{:?}", label.clone()))
        .add_attribute("token_id", token_id)
        .add_attribute("nodehash", format!("{:?}", nodehash)))
}

fn _renew(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    token_id: String,
    duration: u64,
) -> Result<Vec<CosmosMsg>, ContractError> {
    let mut messages: Vec<CosmosMsg> = vec![];
    let config = CONFIG.load(deps.storage)?;
    let registrar_address = deps
        .api
        .addr_humanize(&config.registrar_address)?
        .to_string();

    let renew_registrar_message: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: registrar_address.clone(),
        msg: to_binary(&RegistrarExecuteMsg::<Extension>::Renew {
            id: token_id.clone(),
            duration,
        })?,
        funds: vec![],
    });
    messages.push(renew_registrar_message);

    Ok(messages)
}

pub fn owner_renew(
    mut deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
    duration: u64,
) -> Result<Response, ContractError> {
    only_owner(deps.as_ref(), &info)?;
    let label = get_label_from_name(&name);
    let token_id = get_token_id_from_label(&label);
    let nodehash = get_nodehash(deps.as_ref(), label.clone())?;
    let messages = _renew(deps.branch(), _env, info, token_id.clone(), duration)?;
    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("method", "owner_renew")
        .add_attribute("name", name)
        .add_attribute("duration", duration.to_string())
        .add_attribute("label", format!("{:?}", label))
        .add_attribute("token_id", token_id)
        .add_attribute("nodehash", format!("{:?}", nodehash)))
}

pub fn renew(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
    duration: u64,
) -> Result<Response, ContractError> {
    validate_register_fund(
        deps.as_ref(),
        env.clone(),
        info.clone(),
        name.clone(),
        duration,
    )?;
    let label = get_label_from_name(&name);
    let token_id = get_token_id_from_label(&label);
    let nodehash = get_nodehash(deps.as_ref(), label.clone())?;
    let messages = _renew(deps.branch(), env, info, token_id.clone(), duration)?;
    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("method", "renew")
        .add_attribute("name", name)
        .add_attribute("duration", duration.to_string())
        .add_attribute("label", format!("{:?}", label.clone()))
        .add_attribute("token_id", token_id)
        .add_attribute("nodehash", format!("{:?}", nodehash)))
}

// pub fn get_commitment(
//     name: &String,
//     owner: &String,
//     secret: &String,
//     resolver: &Option<String>,
//     address: &Option<String>,
// ) -> StdResult<GetCommitmentResponse> {
//     let label = get_label_from_name(name);

//     let arr = [
//         &label[..],
//         owner.as_bytes(),
//         resolver.as_deref().unwrap_or(&String::from("")).as_bytes(),
//         address.as_deref().unwrap_or(&String::from("")).as_bytes(),
//         secret.as_bytes(),
//     ]
//     .concat();

//     let commitment_vec = keccak256(&arr);
//     Ok(GetCommitmentResponse {
//         commitment: hex::encode(commitment_vec),
//     })
// }

pub fn get_nodehash(deps: Deps, label: Vec<u8>) -> StdResult<Vec<u8>> {
    let config = CONFIG.load(deps.storage)?;
    let registrar_address = deps
        .api
        .addr_humanize(&config.registrar_address)?
        .to_string();

    let get_base_node_response: GetBaseNodeResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: registrar_address.clone(),
            msg: to_binary(&RegistrarQueryMsg::<WasmQuery>::GetBaseNode {})?,
        }))?;
    let base_node = get_base_node_response.base_node;

    let arr = [&hex::decode(base_node).unwrap(), &label[..]].concat();

    let nodehash = keccak256(&arr);
    Ok(nodehash)
}

pub fn is_available_name(deps: Deps, name: &String) -> StdResult<bool> {
    let label = get_label_from_name(name);
    let id = get_token_id_from_label(&label);
    let config = CONFIG.load(deps.storage)?;
    let registrar_address = deps
        .api
        .addr_humanize(&config.registrar_address)?
        .to_string();
    let is_available_response: IsAvailableResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: registrar_address,
            msg: to_binary(&RegistrarQueryMsg::<WasmQuery>::IsAvailable { id })?,
        }))?;
    return Ok(is_available_response.available);
}

pub fn get_owner(deps: Deps) -> StdResult<OwnerResponse> {
    let config = CONFIG.load(deps.storage)?;
    let owner = deps.api.addr_humanize(&config.owner)?;
    Ok(OwnerResponse { owner })
}

pub fn get_registrar(deps: Deps) -> StdResult<RegistrarResponse> {
    let config = CONFIG.load(deps.storage)?;
    let registrar_address = deps.api.addr_humanize(&config.registrar_address)?;
    Ok(RegistrarResponse { registrar_address })
}

pub fn get_rent_price(deps: Deps, name: String, duration: u64) -> StdResult<RentPriceResponse> {
    let cost = get_cost(deps, name, duration);
    if let Err(_err) = cost {
        return Err(StdError::generic_err("error"));
    }
    Ok(RentPriceResponse {
        price: cost.unwrap(),
    })
}

// pub fn get_commitment_timestamp(
//     deps: Deps,
//     commitment: String,
// ) -> StdResult<CommitmentTimestampResponse> {
//     let timestamp = COMMITMENTS.load(deps.storage, commitment)?;
//     Ok(CommitmentTimestampResponse { timestamp })
// }

pub fn get_min_registration_duration(deps: Deps) -> StdResult<MinRegistrationDurationResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(MinRegistrationDurationResponse {
        duration: config.min_registration_duration,
    })
}

pub fn get_is_valid_name(name: &String) -> StdResult<IsValidNameResponse> {
    let graphemes = name.graphemes(true).collect::<Vec<&str>>();
    let name_length = graphemes.len();
    if graphemes[0usize] == "-" {
        return Ok(IsValidNameResponse {
            is_valid_name: false,
        });
    }
    let is_valid_name = name_length >= 3
        && name.chars().all(|c| -> bool {
            match c {
                '0'..='9' => true,
                'a'..='z' => true,
                '-' => true,
                _c => false,
            }
        });
    Ok(IsValidNameResponse { is_valid_name })
}

pub fn get_node_info_from_name(deps: Deps, name: &String) -> StdResult<NodeInfoResponse> {
    let label: Vec<u8> = get_label_from_name(&name);
    let token_id = get_token_id_from_label(&label);
    let node = get_nodehash(deps, label.clone())?;
    Ok(NodeInfoResponse {
        label,
        token_id,
        node,
    })
}

pub fn get_token_id_from_name(name: &String) -> StdResult<TokenIdResponse> {
    let label: Vec<u8> = get_label_from_name(&name);
    let token_id = get_token_id_from_label(&label);
    Ok(TokenIdResponse { token_id })
}

pub fn get_nodehash_from_name(deps: Deps, name: &String) -> StdResult<NodehashResponse> {
    let label: Vec<u8> = get_label_from_name(&name);
    let node = get_nodehash(deps, label)?;
    Ok(NodehashResponse { node })
}

fn validate_whitelist_fund(deps: Deps, _env: Env, info: MessageInfo) -> Result<(), ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let cost = Uint128::from(config.whitelist_price);
    let base_fund = &Coin {
        denom: String::from(REGISTER_FEE_DENOM),
        amount: Uint128::from(0u128),
    };
    let fund = info
        .funds
        .iter()
        .find(|fund| fund.denom == String::from(REGISTER_FEE_DENOM))
        .unwrap_or(base_fund);
    if fund.amount < cost {
        return Err(ContractError::InsufficientFund {
            amount: fund.amount,
            required: cost,
        });
    }

    Ok(())
}

pub fn add_whitelist(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    ensname: &String,
) -> Result<Response, ContractError> {
    validate_whitelist_fund(deps.as_ref(), env, info)?;

    let config = CONFIG.load(deps.storage)?;
    let registrar_address = deps
        .api
        .addr_humanize(&config.registrar_address)?
        .to_string();

    let label = get_label_from_name(ensname);

    let get_registry_response: GetRegistryResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: registrar_address.clone(),
            msg: to_binary(&RegistrarQueryMsg::<WasmQuery>::GetRegistry {})?,
        }))?;
    let registry_address = String::from(get_registry_response.registry);

    // Set resolver of the node at registry
    let nodehash = get_nodehash(deps.as_ref(), label)?;
    let get_record_by_node_response: RecordResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: registry_address.clone(),
            msg: to_binary(&RegistryQueryMsg::GetRecordByNode {
                node: nodehash.clone(),
            })?,
        }))?;

    WHITELIST.save(
        deps.storage,
        ensname.clone(),
        &(nodehash, config.referal_percentage.1),
    )?;
    Ok(Response::new()
        .add_attribute("method", "add_white_list")
        .add_attribute("ensname", ensname)
        .add_attribute("owner", get_record_by_node_response.owner))
}

pub fn add_whitelist_by_owner(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    ensname: &String,
    refereal_percentage: Option<u32>,
) -> Result<Response, ContractError> {
    only_owner(deps.as_ref(), &info)?;

    let config = CONFIG.load(deps.storage)?;
    let registrar_address = deps
        .api
        .addr_humanize(&config.registrar_address)?
        .to_string();

    let label = get_label_from_name(ensname);

    let get_registry_response: GetRegistryResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: registrar_address.clone(),
            msg: to_binary(&RegistrarQueryMsg::<WasmQuery>::GetRegistry {})?,
        }))?;
    let registry_address = String::from(get_registry_response.registry);

    // Set resolver of the node at registry
    let nodehash = get_nodehash(deps.as_ref(), label)?;
    let get_record_by_node_response: RecordResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: registry_address.clone(),
            msg: to_binary(&RegistryQueryMsg::GetRecordByNode {
                node: nodehash.clone(),
            })?,
        }))?;

    match refereal_percentage {
        Some(value) => {
            WHITELIST.save(deps.storage, ensname.clone(), &(nodehash, value))?;
        }
        None => {
            WHITELIST.save(
                deps.storage,
                ensname.clone(),
                &(nodehash, config.referal_percentage.1),
            )?;
        }
    }

    Ok(Response::new()
        .add_attribute("method", "add_white_list")
        .add_attribute("ensname", ensname)
        .add_attribute("owner", get_record_by_node_response.owner))
}

fn set_reverse_record(
    deps: DepsMut,
    name: String,
    address: String,
    resolver: Option<String>,
    owner: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let registrar_address = deps
        .api
        .addr_humanize(&config.registrar_address)?
        .to_string();

    let get_registrar_config_response: ConfigResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: registrar_address.clone(),
            msg: to_binary(&RegistrarQueryMsg::<WasmQuery>::GetConfig {})?,
        }))?;

    let base_name = get_registrar_config_response.base_name;

    let reverse_registrar_address = deps
        .api
        .addr_humanize(&config.reverse_registrar_address)?
        .to_string();

    let _set_reverse_record_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: reverse_registrar_address,
        msg: to_binary(&ReverseRegistrarExecuteMsg::SetNameForAddr {
            address,
            owner,
            resolver,
            name: name + &".".to_string() + base_name.as_str(),
            // name: name + &".sei".to_string(),
        })?,
        funds: vec![],
    });
    Ok(Response::default())
}
