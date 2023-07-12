use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::CanonicalAddr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub interface_id: u64,
    pub registry_address: CanonicalAddr,
    pub trusted_reverse_registrar: CanonicalAddr,
    pub trusted_controller: CanonicalAddr,
    pub owner: CanonicalAddr,
}

pub const CONFIG: Item<Config> = Item::new("CONFIG");

pub const ADDRESSES: Map<Vec<u8>, String> = Map::new("ADDRESSES");

pub const AVATARS: Map<Vec<u8>, String> = Map::new("AVATARS");

pub const NAMES: Map<Vec<u8>, String> = Map::new("NAMES");

pub const TEXT_DATA: Map<(Vec<u8>, String), String> = Map::new("TEXT");

pub const CONTENT_HASH: Map<Vec<u8>, Vec<u8>> = Map::new("CONTENT_HASH");
