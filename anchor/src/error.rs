use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Cannot set to own account")]
    CannotSetOwnAccount {},

    #[error("Invalid zero amount")]
    InvalidZeroAmount {},

    #[error("Pool: Zero amount not allowed")]
    NotAllowZeroAmount {},

    #[error("Pool: other denom except {denom:?} is not allowed")]
    NotAllowOtherDenoms { denom: String },

    #[error("Pool: other action except {action:?} is not allowed")]
    NotAllowOtherCw20ReceiveAction { action: String },

    #[error("Zero balance")]
    NoBalance {},

    #[error("Can not deposit")]
    InvalidDeposit {},

    #[error("Minting cannot exceed the cap")]
    CannotExceedCap {},
}
