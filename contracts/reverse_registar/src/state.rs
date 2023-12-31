use cosmwasm_std::CanonicalAddr;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: CanonicalAddr,
    pub resolver_address: CanonicalAddr,
    pub registry_address: CanonicalAddr,
}

pub const CONFIG: Item<Config> = Item::new("CONFIG");
