use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub registrar_address: String,
    pub reverse_registrar_address: String,
    pub min_registration_duration: u64,
    pub tier1_price: u64,
    pub tier2_price: u64,
    pub tier3_price: u64,
    pub whitelist_price: u64,
    pub referal_percentage: (u32, u32),
    pub enable_registration: bool,
    pub description: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Register {
        name: String,
        owner: String,
        duration: u64,
        secret: String,
        resolver: Option<String>,
        address: Option<String>,
        reverse_record: bool,
    },
    ReferalRegister {
        name: String,
        owner: String,
        duration: u64,
        secret: String,
        resolver: Option<String>,
        address: Option<String>,
        referer: Option<String>,
        reverse_record: bool,
    },
    OwnerRegister {
        name: String,
        owner: String,
        duration: u64,
        resolver: Option<String>,
        address: Option<String>,
        reverse_record: bool,
    },
    SetConfig {
        min_registration_duration: u64,
        tier1_price: u64,
        tier2_price: u64,
        tier3_price: u64,
        registrar_address: String,
        reverse_registrar_address: String,
        owner: String,
        enable_registration: bool,
        description: String,
    },
    Withdraw {},
    Renew {
        name: String,
        duration: u64,
    },
    OwnerRenew {
        name: String,
        duration: u64,
    },
    SetEnableRegistration {
        enable_registration: bool,
    },
    AddWhiteList {
        ensname: String,
    },
    AddWhiteListByOwner {
        ensname: String,
        referal_percentage: Option<u32>,
    },
    SetReferalPercentage {
        normal_percentage: u32,
        whitelist_percentage: u32,
    },
    SetWhitelistPrice {
        price: u64,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Binary)]
    Owner {},
    #[returns(Binary)]
    Registrar {},
    #[returns(Binary)]
    RentPrice { name: String, duration: u64 },
    #[returns(Binary)]
    MinRegistrationDuration {},
    #[returns(Binary)]
    IsValidName { name: String },
    #[returns(Binary)]
    GetTokenId { name: String },
    #[returns(Binary)]
    GetNodehash { name: String },
    #[returns(Binary)]
    GetNodeInfo { name: String },
    #[returns(Binary)]
    GetPrice {},
}

// We define a custom struct for each query response

#[cw_serde]
pub struct RentPriceResponse {
    pub price: Uint128,
}

#[cw_serde]
pub struct MinRegistrationDurationResponse {
    pub duration: u64,
}

#[cw_serde]
pub struct IsValidNameResponse {
    pub is_valid_name: bool,
}

#[cw_serde]
pub struct TokenIdResponse {
    pub token_id: String,
}

#[cw_serde]
pub struct NodehashResponse {
    pub node: Vec<u8>,
}

#[cw_serde]
pub struct NodeInfoResponse {
    pub label: Vec<u8>,
    pub token_id: String,
    pub node: Vec<u8>,
}

#[cw_serde]
pub struct OwnerResponse {
    pub owner: Addr,
}

#[cw_serde]
pub struct RegistrarResponse {
    pub registrar_address: Addr,
}

#[cw_serde]
pub struct PriceResponse {
    pub tier1_price: u64,
    pub tier2_price: u64,
    pub tier3_price: u64,
    pub whitelist_price: u64,
}

#[cw_serde]
pub struct MigrateMsg {}
