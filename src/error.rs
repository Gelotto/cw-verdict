use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContractError {
  #[error("{0}")]
  Std(#[from] StdError),

  #[error("ValidationError")]
  ValidationError {},

  #[error("NotAuthorized")]
  NotAuthorized {},

  #[error("InsufficientFunds")]
  InsufficientFunds {},

  #[error("ExcessiveFunds")]
  ExcessiveFunds {},

  #[error("NotActive")]
  NotActive {},

  #[error("NotDeciding")]
  NotDeciding {},

  #[error("InvalidChoice")]
  InvalidChoice {},

  #[error("DeliberationsExpired")]
  DeliberationsExpired {},

  #[error("NotDecided")]
  NotDecided {},

  #[error("HasClaimed")]
  HasClaimed {},

  #[error("InvalidWeight")]
  InvalidWeight {},
}
