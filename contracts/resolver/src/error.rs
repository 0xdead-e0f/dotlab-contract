use cosmwasm_std::{Binary, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("NotOwner: Sender is {sender}, but owner is {owner}.")]
    NotOwner { sender: String, owner: String },

    #[error("NotNodeOwner(Resolver): Sender {sender} is not node owner of {node}.")]
    NotNodeOwner { sender: String, node: String },

    #[error("Multicall InvalidCall")]
    InvalidCall {},

    #[error("Multicall Error")]
    MulticallExecuteError { errors: Vec<ContractError>},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("Querier system error: {0}")]
    System(String),

    #[error("Querier contract error: {0}")]
    Contract(String),
}

pub type QueryResult = core::result::Result<Binary, QueryError>;

impl QueryError {
    pub fn std_at_index(self, i: usize) -> StdError {
        StdError::generic_err(format!("Error at index {}, {}", i, self))
    }

    pub fn std(self) -> StdError {
        StdError::generic_err(self)
    }
}

impl From<QueryError> for String {
    fn from(q: QueryError) -> Self {
        q.to_string()
    }
}

impl From<QueryError> for StdError {
    fn from(source: QueryError) -> Self {
        source.std()
    }
}
