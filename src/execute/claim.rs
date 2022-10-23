use crate::{
  error::ContractError,
  models::Token,
  state::{HAS_CLAIMED, TRIAL, VOTERS_TOTAL_CLAIM_AMOUNT, VOTES},
  util::{build_cw20_transfer_msg, build_native_send_msg, respond_cw20, respond_native},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, Uint128};

/// A wallet can claim under 2 conditions: (1) the game was canceled or hung, or
/// (2) the winning choice has been decided. If the game was canceled, then the
/// wallet can reclaim their funds. However, if a wallet won, then the size of
/// the prize is proportional to the weight of their vote. For example, if they
/// voted with a weight of 5, then their portion of the winnings is 5x the
/// portion allocated to someone whose vote carried a weight of 1. The trial
/// must be in the "decided" state to execute this method, and only winning
/// wallets are authorized.
pub fn claim(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
) -> Result<Response, ContractError> {
  let voter_addr = &info.sender;
  let trial = TRIAL.load(deps.storage)?;

  // if the game was canceled or hung, then send the voter a complete refund.
  if trial.has_been_canceled() || trial.has_hung_jury() {
    let mut voter_weight = 0u32;
    for choice_index in 0..trial.choices.len() {
      if let Some(vote) = VOTES.may_load(deps.storage, (choice_index as u32, voter_addr.clone()))? {
        voter_weight += vote.weight;
      }
    }
    if voter_weight > 0 {
      // build a response with the necessary transfer msg/submsg for the refund
      let claim_amount = Uint128::from(voter_weight) * trial.price;
      let response = Response::new().add_attributes(vec![
        attr("action", "claim"),
        attr("type", "refund"),
        attr("amount", claim_amount.to_string()),
      ]);
      Ok(match trial.token.clone() {
        Token::Native { denom } => {
          response.add_message(build_native_send_msg(&info.sender, &denom, claim_amount)?)
        },
        Token::Cw20 {
          address: cw20_token_address,
        } => response.add_submessage(build_cw20_transfer_msg(
          &deps,
          &env.contract.address,
          &info.sender,
          &cw20_token_address,
          claim_amount,
        )?),
      })
    } else {
      // the tx sender didn't cast any votes
      Err(ContractError::NotAuthorized {})
    }
  }
  // otherwise, ssume the tx sender is trying to claim their prize
  else if let Some(choice_index) = trial.winner {
    if !trial.has_verdict() {
      // even if the winner choice index is set, still check the trial's status
      // and abort if it isn't "decided"
      return Err(ContractError::NotDecided {});
    }
    // get metadata for the choice that won...
    if let Some(choice) = trial.choices.get(choice_index as usize) {
      // get the claimant's voting record...
      if let Some(vote) = VOTES.may_load(deps.storage, (choice_index, voter_addr.clone()))? {
        // abort if the voter has already claimed
        if HAS_CLAIMED
          .may_load(deps.storage, voter_addr.clone())?
          .unwrap_or(false)
        {
          Err(ContractError::HasClaimed {})
        } else {
          // persist the fact that the wallet has now claimed
          HAS_CLAIMED.save(deps.storage, voter_addr.clone(), &true)?;
          // build a response with the necessary transfer msg/submsg
          let voters_total_amount = VOTERS_TOTAL_CLAIM_AMOUNT.load(deps.storage)?;
          let claim_amount = voters_total_amount * Uint128::from(vote.weight / choice.weight);

          let response = Response::new().add_attributes(vec![
            attr("action", "claim"),
            attr("type", "reward"),
            attr("amount", claim_amount.to_string()),
          ]);

          Ok(match trial.token.clone() {
            Token::Native { denom } => {
              response.add_message(build_native_send_msg(&info.sender, &denom, claim_amount)?)
            },
            Token::Cw20 {
              address: cw20_token_address,
            } => response.add_submessage(build_cw20_transfer_msg(
              &deps,
              &env.contract.address,
              &info.sender,
              &cw20_token_address,
              claim_amount,
            )?),
          })
        }
      } else {
        // the sender didn't vote for the winner
        Err(ContractError::NotAuthorized {})
      }
    } else {
      // shouldn't be possible. this means that somehow an invalid choice index
      // was saved as the winner in the `decide` function.
      Err(ContractError::InvalidChoice {})
    }
  } else {
    // the jury is still out, so nothing can be claimed.
    Err(ContractError::NotDecided {})
  }
}
