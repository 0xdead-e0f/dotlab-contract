use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::registry::RecordResponse;

//#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[cw_serde]
pub struct InstantiateMsg {
    pub resolver_address: String,
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
    Claim {
        owner: String,
    },
    ClaimForAddr {
        address: String,
        owner: String,
        resolver: String,
    },
    SetName {
        name: String,
    },
    SetConfig {
        resolver_address: String,
        registry_address: String,
        owner: String,
    },
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    GetConfig {},
    #[returns(RecordResponse)]
    GetReverseRecord { node: Vec<u8> },
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[cw_serde]
pub struct NameResponse {
    pub name: String,
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[cw_serde]
pub struct ConfigResponse {
    pub registry_address: Addr,
    pub resolver_address: Addr,
    pub owner: Addr,
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[cw_serde]
pub struct MigrateMsg {}
