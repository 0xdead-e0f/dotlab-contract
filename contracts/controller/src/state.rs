use cosmwasm_schema::cw_serde;
use cosmwasm_std::CanonicalAddr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub min_registration_duration: u64,
    pub tier1_price: u64,
    pub tier2_price: u64,
    pub tier3_price: u64,
    pub whitelist_price: u64,
    pub referal_percentage: (u32, u32),
    pub enable_registration: bool,
    pub registrar_address: CanonicalAddr,
    pub reverse_registrar_address: CanonicalAddr,
    pub owner: CanonicalAddr,
    pub description: String,
}

pub const REGISTER_FEE_DENOM: &str = "usei";
pub const CONFIG: Item<Config> = Item::new("CONFIG");
pub const WHITELIST: Map<String, (Vec<u8>, u32)> = Map::new("WHITELIST");
