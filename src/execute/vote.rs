use crate::{
  error::ContractError,
  models::{Token, Vote},
  state::{TRIAL, VOTES},
  util::{respond_cw20, respond_native},
};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};

/// Cast a vote on an active trial. The funding required is proportional to the
/// weight of the vote.
pub fn vote(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  choice_index: usize,
  weight: u32,
) -> Result<Response, ContractError> {
  let mut trial = TRIAL.load(deps.storage)?;
  let payment = Uint128::from(weight) * trial.price;

  // abort the vote if the trial is closed
  if !trial.is_active() {
    return Err(ContractError::NotActive {});
  }
  // abort if the weight is 0
  if weight < 1 {
    return Err(ContractError::InvalidWeight {});
  }

  // cast the vote
  if let Some(choice) = trial.choices.get_mut(choice_index) {
    // increment global vote tally
    choice.tally += 1;
    choice.weight += weight;
    // upsert the wallet's voting record for this choice,
    // incrementing the net weight.
    VOTES.update(
      deps.storage,
      (choice_index as u32, info.sender.clone()),
      |some_vote| -> Result<Vote, ContractError> {
        if let Some(mut vote) = some_vote {
          vote.weight += weight;
          Ok(vote)
        } else {
          Ok(Vote {
            choice: choice_index as u32,
            weight,
          })
        }
      },
    )?;
  } else {
    return Err(ContractError::InvalidChoice {});
  }

  // note: trial.weight must equal the sum of the choices' weights
  trial.weight += weight;

  // save Ballot with updated Choice record
  TRIAL.save(deps.storage, &trial)?;

  // return a respnse with the necessary transfer msg/submsg
  Ok(match trial.token.clone() {
    Token::Native { denom } => {
      respond_native(&info, &env.contract.address, &denom, payment, "vote")?
    },
    Token::Cw20 { address } => respond_cw20(
      &deps,
      &info,
      &env.contract.address,
      &address,
      payment,
      "vote",
    )?,
  })
}
