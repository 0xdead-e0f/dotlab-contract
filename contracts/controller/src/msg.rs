use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub registrar_address: String,
    pub max_commitment_age: u64,
    pub min_commitment_age: u64,
    pub min_registration_duration: u64,
    pub tier1_price: u64,
    pub tier2_price: u64,
    pub tier3_price: u64,
    pub whitelist_price: u64,
    pub enable_registration: bool,
}

#[cw_serde]
pub enum ExecuteMsg {
    Commit {
        commitment: String,
    },
    Register {
        name: String,
        owner: String,
        duration: u64,
        secret: String,
        resolver: Option<String>,
        address: Option<String>,
    },
    ReferalRegister {
        name: String,
        owner: String,
        duration: u64,
        secret: String,
        resolver: Option<String>,
        address: Option<String>,
        referer: Option<String>,
    },
    OwnerRegister {
        name: String,
        owner: String,
        duration: u64,
        resolver: Option<String>,
        address: Option<String>,
    },
    SetConfig {
        max_commitment_age: u64,
        min_commitment_age: u64,
        min_registration_duration: u64,
        tier1_price: u64,
        tier2_price: u64,
        tier3_price: u64,
        registrar_address: String,
        owner: String,
        enable_registration: bool,
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
        address: String,
        name: String,
    },
    AddWhiteListByOwner {
        address: String, 
        name: String,
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Binary)]
    Owner {},
    #[returns(Binary)]
    Registrar {},
    #[returns(Binary)]
    CommitmentTimestamp { commitment: String },
    #[returns(Binary)]
    GetCommitment {
        name: String,
        owner: String,
        secret: String,
        resolver: Option<String>,
        address: Option<String>,
    },
    #[returns(Binary)]
    RentPrice { name: String, duration: u64 },
    #[returns(Binary)]
    MaxCommitmentAge {},
    #[returns(Binary)]
    MinCommitmentAge {},
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
pub struct GetCommitmentResponse {
    pub commitment: String,
}

#[cw_serde]
pub struct CommitmentTimestampResponse {
    pub timestamp: u64,
}

#[cw_serde]
pub struct RentPriceResponse {
    pub price: Uint128,
}

#[cw_serde]
pub struct MaxCommitmentAgeResponse {
    pub age: u64,
}

#[cw_serde]
pub struct MinCommitmentAgeResponse {
    pub age: u64,
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
