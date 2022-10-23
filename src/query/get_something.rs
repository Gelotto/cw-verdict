use crate::{msg::GetBallotResponse, state::BALLOT};
use cosmwasm_std::{Deps, StdResult};

pub fn get_ballot(deps: Deps) -> StdResult<GetBallotResponse> {
  let ballot = BALLOT.load(deps.storage)?;
  Ok(GetBallotResponse { ballot })
}
