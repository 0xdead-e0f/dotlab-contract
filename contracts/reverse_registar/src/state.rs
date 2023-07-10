use cosmwasm_std::{CanonicalAddr, Addr};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub grace_period: u64,
    pub owner: CanonicalAddr,
    pub addr_reverse_node: Vec<u8>,
    pub registry_address: CanonicalAddr,
}

pub const CONFIG: Item<Config> = Item::new("CONFIG");
pub const EXPIRIES: Map<String, u64> = Map::new("EXPIRIES");
pub const CONTROLLERS: Map<Addr, bool> = Map::new("CONTROLLERS");