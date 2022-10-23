use crate::{
  error::ContractError,
  models::Token,
  state::{BALLOT, HAS_CLAIMED, VOTES},
  util::{respond_cw20, respond_native},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, Uint128};

/// A wallet can claim under 2 conditions: (1) the game was canceled or hung, or
/// (2) the winning choice has been decided. If the game was canceled, then the
/// wallet can reclaim their funds. However, if a wallet won, then the size of
/// the prize is proportional to the weight of their vote. For example, if they
/// voted with a weight of 5, then their portion of the winnings is 5x the
/// portion allocated to someone whose vote carried a weight of 1. The ballot
/// must be in the "decided" state to execute this method, and only winning
/// wallets are authorized.
pub fn claim(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
) -> Result<Response, ContractError> {
  let voter_addr = &info.sender;
  let ballot = BALLOT.load(deps.storage)?;

  // if the game was canceled or hung, then send the voter a complete refund.
  if ballot.is_canceled() || ballot.is_hung() {
    let mut voter_weight = 0u32;
    for choice_index in 0..ballot.choices.len() {
      if let Some(vote) = VOTES.may_load(deps.storage, (choice_index as u32, voter_addr.clone()))? {
        voter_weight += vote.weight;
      }
    }
    if voter_weight > 0 {
      // build a response with the necessary transfer msg/submsg for the refund
      let claim_amount = Uint128::from(voter_weight) * ballot.price;
      let response = match ballot.token.clone() {
        Token::Native { denom } => respond_native(&info, &env, &denom, claim_amount, "claim")?,
        Token::Cw20 { address } => {
          respond_cw20(&deps, &info, &env, &address, claim_amount, "claim")?
        },
      };
      Ok(response.add_attributes(vec![
        attr("type", "refund"),
        attr("amount", claim_amount.to_string()),
      ]))
    } else {
      // the tx sender didn't cast any votes
      Err(ContractError::NotAuthorized {})
    }
  }
  // otherwise, ssume the tx sender is trying to claim their prize
  else if let Some(choice_index) = ballot.winner {
    if !ballot.is_decided() {
      // even if the winner choice index is set, still check the ballot's status
      // and abort if it isn't "decided"
      return Err(ContractError::NotDecided {});
    }
    // get metadata for the choice that won...
    if let Some(choice) = ballot.choices.get(choice_index as usize) {
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
          let total_amount = ballot.price * Uint128::from(ballot.weight);
          let claim_amount = total_amount * Uint128::from(vote.weight / choice.weight);
          let response = match ballot.token.clone() {
            Token::Native { denom } => respond_native(&info, &env, &denom, claim_amount, "claim")?,
            Token::Cw20 { address } => {
              respond_cw20(&deps, &info, &env, &address, claim_amount, "claim")?
            },
          };
          Ok(response.add_attributes(vec![
            attr("type", "prize"),
            attr("amount", claim_amount.to_string()),
          ]))
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
