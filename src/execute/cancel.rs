use crate::{error::ContractError, models::Status, state::BALLOT};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

pub fn cancel(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
) -> Result<Response, ContractError> {
  let mut ballot = BALLOT.load(deps.storage)?;

  // only the ballot's owner can cancel;
  // only games that are active or deciding can be canceled
  if info.sender == ballot.owner && ballot.can_be_canceled() {
    ballot.status = Status::Canceled;
    BALLOT.save(deps.storage, &ballot)?;
    Ok(Response::new().add_attributes(vec![attr("action", "cancel")]))
  } else {
    Err(ContractError::NotAuthorized {})
  }
}
