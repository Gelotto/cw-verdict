use crate::{
  error::ContractError,
  models::Status,
  state::{CANCEL_REASON, TRIAL},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

pub fn cancel(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  reason: &String,
) -> Result<Response, ContractError> {
  let mut trial = TRIAL.load(deps.storage)?;

  // only the trial's owner can cancel;
  // only games that are active or deciding can be canceled
  if info.sender == trial.owner && trial.can_be_canceled() {
    trial.status = Status::Dismissed;
    TRIAL.save(deps.storage, &trial)?;
    CANCEL_REASON.save(deps.storage, reason)?;
    Ok(Response::new().add_attributes(vec![attr("action", "cancel")]))
  } else {
    Err(ContractError::NotAuthorized {})
  }
}
