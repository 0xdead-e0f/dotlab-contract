use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

//#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[cw_serde]
pub struct InstantiateMsg {
    pub interface_id: u64,
    pub addr_reverse_node: String,
    pub registry_address: String,
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
#[cw_serde]
pub enum ExecuteMsg {
    SetNameForAddr {
        address: String,
        owner: String,
        resolver: Option<String>,
        name: String,
    },
    SetNameForAddrWithSignature {
        address: String,
        owner: String,
        resolver: Option<String>,
        relayer: String,
        signature_expiry: u128,
        signature: Vec<u8>,
        name: String,
    },
    Claim {
        owner: String,
    },
    ClaimForAddrWithSignature {
        address: String,
        owner: String,
        resolver: String,
        relayer: String,
        signature_expiry: u128,
        signature: Vec<u8>,
    },
    ClaimForAddr {
        address: String,
        owner: String,
        resolver: String,
    },
    ClaimWithResolver {
        owner: String,
        resolver: String,
    },
    SetName {
        name: String,
    },
    SetConfig {
        interface_id: u64,
        registry_address: String,
        owner: String,
    },
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
#[cw_serde]
pub enum QueryMsg {
    GetNode { address: String },
    GetConfig {},
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[cw_serde]
pub struct NameResponse {
    pub name: String,
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[cw_serde]
pub struct ConfigResponse {
    pub interface_id: u64,
    pub registry_address: Addr,
    pub owner: Addr,
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[cw_serde]
pub struct MigrateMsg {}
