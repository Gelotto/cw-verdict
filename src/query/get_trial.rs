use crate::{msg::GetTrialResponse, state::TRIAL};
use cosmwasm_std::{Deps, StdResult};

pub fn get_trial(deps: Deps) -> StdResult<GetTrialResponse> {
  let trial = TRIAL.load(deps.storage)?;
  Ok(GetTrialResponse { trial: trial })
}
