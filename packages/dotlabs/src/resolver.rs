use cosmwasm_std::{Addr, Binary};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub interface_id: u64,
    pub registry_address: String,
    pub trusted_reverse_registrar: String,
    pub trusted_controller: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetAddress {
        node: Vec<u8>,
        address: String,
    },
    SetSeiAddress {
        node: Vec<u8>,
        address: String,
    },
    SetTextData {
        node: Vec<u8>,
        key: String,
        value: String,
    },
    SetContentHash {
        node: Vec<u8>,
        hash: Vec<u8>,
    },
    SetConfig {
        interface_id: u64,
        registry_address: String,
        trusted_reverse_registrar: String,
        trusted_controller: String,
        owner: String,
    },
    SetName {
        address: String,
        name: String,
    },
    SetAvatar {
        node: Vec<u8>,
        avatar_uri: String,
    },
    Multicall {
        functions: Vec<FunctionCall>,
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FunctionCall {
    SetAddress {
        node: Vec<u8>,
        address: String,
    },
    SetSeiAddress {
        node: Vec<u8>,
        address: String,
    },
    SetTextData {
        node: Vec<u8>,
        key: String,
        value: String,
    },
    SetContentHash {
        node: Vec<u8>,
        hash: Vec<u8>,
    },
    SetName {
        address: String,
        name: String,
    },
    SetAvatar {
        node: Vec<u8>,
        avatar_uri: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetAddress { node: Vec<u8> },
    GetAvatar { node: Vec<u8> },
    GetName { node: Vec<u8> },
    GetTextData { node: Vec<u8>, key: String },
    GetSeiAddress { node: Vec<u8> },
    GetContentHash { node: Vec<u8> },
    GetConfig {},
    Multicall { queries: Vec<Binary> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AddressResponse {
    pub address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AvatarResponse {
    pub avatar_uri: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NameResponse {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TextDataResponse {
    pub data: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContentHashResponse {
    pub hash: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MulticallResponse {
    pub data: Vec<Binary>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub interface_id: u64,
    pub registry_address: Addr,
    pub trusted_reverse_registrar: Addr,
    pub trusted_controller: Addr,
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

