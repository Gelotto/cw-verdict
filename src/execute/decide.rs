use crate::{
  error::ContractError,
  models::Status,
  state::{BALLOT, CLAIM_AMOUNT, JURORS},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Order, Response, Uint128};

/// A decider casts their vote here.
pub fn decide(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  choice_index: usize,
  script_logs: &String,
) -> Result<Response, ContractError> {
  if let Some(mut juror) = JURORS.may_load(deps.storage, info.sender.clone())? {
    let mut ballot = BALLOT.load(deps.storage)?;

    // abort if we're not in the deciding state
    if !ballot.is_deciding() {
      return Err(ContractError::NotDeciding {});
    }
    // abort if the jury is casting a vote for an invalid choice
    if choice_index > ballot.choices.len() - 1 {
      return Err(ContractError::InvalidChoice {});
    }
    // abort if the deliberation period has ended
    let end_time = ballot
      .verdict
      .start
      .plus_seconds(60u64 * (ballot.verdict.duration as u64));
    if env.block.time > end_time {
      return Err(ContractError::DeliberationsExpired {});
    }

    juror.choice = Some(choice_index as u32);
    juror.logs = Some(script_logs.clone());

    JURORS.save(deps.storage, info.sender.clone(), &juror)?;

    // determine if the jury is now hung. The jury is "hung" when the current
    // juror's "choice" does not agree with all extant jury choices.
    let mut can_be_decided = true;
    for juror_result in JURORS.range(deps.storage, None, None, Order::Ascending) {
      if let Some((_juror_addr, juror)) = juror_result.ok() {
        if let Some(existing_choice_index) = juror.choice {
          if existing_choice_index != choice_index as u32 {
            ballot.status = Status::Hung;
            can_be_decided = false;
            break;
          }
        } else {
          // the game can't be decided unless 100% of jurors are present,
          // and this juror hasn't made their choice yet
          can_be_decided = false;
          break;
        }
      }
    }
    // if not hung, transition the state to Decided because all jurors are in
    // agreement winning choice.
    if can_be_decided {
      ballot.status = Status::Decided;
      ballot.winner = Some(choice_index as u32);

      // set claim amount for winners
      CLAIM_AMOUNT.save(deps.storage, &(ballot.price * Uint128::from(ballot.weight)))?;
    }

    BALLOT.save(deps.storage, &ballot)?;
  } else {
    // the tx sender isn't a registered juror
    return Err(ContractError::NotAuthorized {});
  }

  Ok(Response::new().add_attributes(vec![attr("action", "decide")]))
}
