use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, BlockInfo};
use cw0::Expiration;
use cw_storage_plus::{Index, IndexList, MultiIndex};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[cw_serde]
pub struct InstantiateMsg {
    pub base_node: String,
    pub base_name: String,
    pub registry_address: String,
    pub grace_period: Option<u64>,

    /// Name of the NFT contract
    pub name: String,
    /// Symbol of the NFT contract
    pub symbol: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg<Q: JsonSchema> {
    #[returns(IsAvailableResponse)]
    IsAvailable { id: String },
    #[returns(GetExpiresResponse)]
    GetExpires { id: String },
    #[returns(GetBaseNodeResponse)]
    GetBaseNode {},
    #[returns(GetRegistryResponse)]
    GetRegistry {},
    #[returns(GetGracePeriodResponse)]
    GetGracePeriod {},
    #[returns(ConfigResponse)]
    GetConfig {},
    /// Return the owner of the given token, error if token does not exist
    #[returns(cw721::OwnerOfResponse)]
    OwnerOf {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },
    /// Return operator that can access all of the owner's tokens.
    #[returns(cw721::ApprovalResponse)]
    Approval {
        token_id: String,
        spender: String,
        include_expired: Option<bool>,
    },
    /// Return approvals that a token has
    #[returns(cw721::ApprovalsResponse)]
    Approvals {
        token_id: String,
        include_expired: Option<bool>,
    },
    /// Return approval of a given operator for all tokens of an owner, error if not set
    #[returns(cw721::OperatorResponse)]
    Operator {
        owner: String,
        operator: String,
        include_expired: Option<bool>,
    },
    /// List all operators that can access all of the owner's tokens
    #[returns(cw721::OperatorsResponse)]
    AllOperators {
        owner: String,
        /// unset or false will filter out expired items, you must set to true to see them
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Total number of tokens issued
    #[returns(cw721::NumTokensResponse)]
    NumTokens {},

    /// With MetaData Extension.
    /// Returns top-level metadata about the contract
    #[returns(cw721::ContractInfoResponse)]
    ContractInfo {},
    /// With MetaData Extension.
    /// Returns metadata about one particular token, based on *ERC721 Metadata JSON Schema*
    /// but directly from the contract
    #[returns(cw721::NftInfoResponse<Q>)]
    NftInfo { token_id: String },
    /// With MetaData Extension.
    /// Returns the result of both `NftInfo` and `OwnerOf` as one query as an optimization
    /// for clients
    #[returns(cw721::AllNftInfoResponse<Q>)]
    AllNftInfo {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },

    /// With Enumerable extension.
    /// Returns all tokens owned by the given address, [] if unset.
    #[returns(cw721::TokensResponse)]
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// With Enumerable extension.
    /// Requires pagination. Lists all token_ids controlled by the contract.
    #[returns(cw721::TokensResponse)]
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },

    /// Return the minter
    #[returns(MinterResponse)]
    Minter {},

    /// Extension query
    #[returns(())]
    Extension { msg: Q },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg<T> {
    Register {
        id: String,
        owner: String,
        duration: u64,
        name: String,
    },
    AddController {
        address: String,
    },
    RemoveController {
        address: String,
    },
    SetConfig {
        grace_period: u64,
        registry_address: String,
        owner: String,
    },
    Renew {
        id: String,
        duration: u64,
    },
    Reclaim {
        id: String,
        owner: String,
    },

    /// Transfer is a base message to move a token to another account without triggering actions
    TransferNft {
        recipient: String,
        token_id: String,
    },
    /// Send is a base message to transfer a token to a contract and trigger an action
    /// on the receiving contract.
    SendNft {
        contract: String,
        token_id: String,
        msg: Binary,
    },
    /// Allows operator to transfer / send the token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    Approve {
        spender: String,
        token_id: String,
        expires: Option<cw721::Expiration>,
    },
    /// Remove previously granted Approval
    Revoke {
        spender: String,
        token_id: String,
    },
    /// Allows operator to transfer / send any token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    ApproveAll {
        operator: String,
        expires: Option<cw721::Expiration>,
    },
    /// Remove previously granted ApproveAll permission
    RevokeAll {
        operator: String,
    },
    /// Mint a new NFT, can only be called by the contract minter
    Mint(MintMsg<T>),
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[cw_serde]
pub struct IsAvailableResponse {
    pub available: bool,
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[cw_serde]
pub struct GetExpiresResponse {
    pub expires: u64,
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[cw_serde]
pub struct GetBaseNodeResponse {
    pub base_node: String,
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[cw_serde]
pub struct GetRegistryResponse {
    pub registry: Addr,
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[cw_serde]
pub struct GetGracePeriodResponse {
    pub grace_period: u64,
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[cw_serde]
pub struct ConfigResponse {
    pub grace_period: u64,
    pub registry_address: Addr,
    pub owner: Addr,
    pub base_node: Vec<u8>,
    pub base_name: String,
}

// #[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[cw_serde]
pub struct OwnerOfResponse {
    /// Owner of the token
    pub owner: String,
    /// If set this address is approved to transfer/send the token as well
    pub approvals: Vec<Approval>,
}

// #[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[cw_serde]
pub struct Approval {
    /// Account that can transfer/send the token
    pub spender: String,
    /// When the Approval expires (maybe Expiration::never)
    pub expires: Expiration,
}

// #[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[cw_serde]
pub struct ApprovedForAllResponse {
    pub operators: Vec<Approval>,
}

// #[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[cw_serde]
pub struct NumTokensResponse {
    pub count: u64,
}

// #[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[cw_serde]
pub struct ContractInfoResponse {
    pub name: String,
    pub symbol: String,
}

// #[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[cw_serde]
pub struct NftInfoResponse<T> {
    /// Universal resource identifier for this NFT
    /// Should point to a JSON file that conforms to the ERC721
    /// Metadata JSON Schema
    pub token_uri: Option<String>,
    /// You can add any custom metadata here when you extend cw721-base
    pub extension: T,
}

// #[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[cw_serde]
pub struct AllNftInfoResponse<T> {
    /// Who can transfer the token
    pub access: OwnerOfResponse,
    /// Data on the token itself,
    pub info: NftInfoResponse<T>,
}

// #[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[cw_serde]
pub struct TokensResponse {
    /// Contains all token_ids in lexicographical ordering
    /// If there are more than `limit`, use `start_from` in future queries
    /// to achieve pagination.
    pub tokens: Vec<String>,
}

/// Shows who can mint these tokens
// #[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[cw_serde]
pub struct MinterResponse {
    pub minter: String,
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[cw_serde]
pub struct TokenInfo<T> {
    /// The owner of the newly minted NFT
    pub owner: Addr,
    /// Approvals are stored here, as we clear them all upon transfer and cannot accumulate much
    pub approvals: Vec<Approval>,

    /// Identifies the asset to which this NFT represents
    pub name: String,
    /// Describes the asset to which this NFT represents
    pub description: String,
    /// A URI pointing to an image representing the asset
    pub image: Option<String>,

    /// You can add any custom metadata here when you extend cw721-base
    pub extension: T,
}

impl Approval {
    pub fn is_expired(&self, block: &BlockInfo) -> bool {
        self.expires.is_expired(block)
    }
}

pub struct TokenIndexes<'a, T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    // pk goes to second tuple element
    pub owner: MultiIndex<'a, (Addr, Vec<u8>), TokenInfo<T>, String>,
}

impl<'a, T> IndexList<TokenInfo<T>> for TokenIndexes<'a, T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<TokenInfo<T>>> + '_> {
        let v: Vec<&dyn Index<TokenInfo<T>>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}

pub fn token_owner_idx<T>(d: &TokenInfo<T>, k: Vec<u8>) -> (Addr, Vec<u8>) {
    (d.owner.clone(), k)
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Extension {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MintMsg<T> {
    /// Unique ID of the NFT
    pub token_id: String,
    /// The owner of the newly minter NFT
    pub owner: String,
    /// Identifies the asset to which this NFT represents
    pub name: String,
    /// Describes the asset to which this NFT represents (may be empty)
    pub description: Option<String>,
    /// A URI pointing to an image representing the asset
    pub image: Option<String>,
    /// Any custom extension used by this contract
    pub extension: T,

    // /// Unique ID of the gift Card NFT
    // pub token_id: String,
    // /// The recipient of the newly minted NFT
    // pub owner: String,
    // /// The time to which funds and tokens will be locked in contract
    // pub lockup_time: u128,
    // /// Universal resource identifier for this NFT
    // /// Should point to a JSON file that conforms to the ERC721
    // /// Metadata JSON Schema
    // pub token_uri: Option<String>,
    // pub message: String,
    // /// Any custom extension used by this contract
    // pub extension: T,
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[cw_serde]
pub struct MigrateMsg {}
