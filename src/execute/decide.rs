use crate::{
  error::ContractError,
  models::{Juror, Status, Token},
  state::{JURORS, TRIAL, VOTERS_TOTAL_CLAIM_AMOUNT},
  util::{build_cw20_transfer_msg, build_native_send_msg},
};
use cosmwasm_std::{attr, CosmosMsg, DepsMut, Env, MessageInfo, Order, Response, SubMsg, Uint128};

/// A jury uploads the result of running the decision script. Once all jurors
/// have invoked this message and agree on the outcome (i.e. all "choice" values
/// must be equal), the contract transfers rewards to the jury, and the game
/// goes into the `HasVerdict` state, where winning voters can claim rewards.
pub fn decide(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  choice_index: usize,
  script_logs: &String,
) -> Result<Response, ContractError> {
  if let Some(mut juror) = JURORS.may_load(deps.storage, info.sender.clone())? {
    let mut trial = TRIAL.load(deps.storage)?;

    // abort if we're not in the deciding state
    if !trial.is_in_deliberations() {
      return Err(ContractError::NotDeciding {});
    }
    // abort if the jury is casting a vote for an invalid choice
    if choice_index > trial.choices.len() - 1 {
      return Err(ContractError::InvalidChoice {});
    }
    // abort if the deliberation period has ended
    let end_time = trial
      .verdict
      .start
      .plus_seconds(60u64 * (trial.verdict.minutes as u64));
    if env.block.time > end_time {
      return Err(ContractError::DeliberationsExpired {});
    }

    // persist juror's choice and script execution logs
    juror.choice = Some(choice_index as u32);
    juror.logs = Some(script_logs.clone());

    JURORS.save(deps.storage, info.sender.clone(), &juror)?;

    // determine if the jury is now hung. The jury is "hung" when the current
    // juror's "choice" does not agree with all extant jury choices.
    let mut can_be_decided = true;
    let mut jurors: Vec<Juror> = vec![];
    for juror_result in JURORS.range(deps.storage, None, None, Order::Ascending) {
      if let Some((_juror_addr, juror)) = juror_result.ok() {
        if let Some(existing_choice_index) = juror.choice {
          if existing_choice_index != choice_index as u32 {
            trial.status = Status::HungJury;
            can_be_decided = false;
            break;
          } else {
            jurors.push(juror);
          }
        } else {
          // the game can't be decided unless 100% of jurors are present,
          // and this juror hasn't made their choice yet
          can_be_decided = false;
          break;
        }
      }
    }
    let mut cw20_jury_transfer_submsgs: Vec<SubMsg> = Vec::with_capacity(jurors.len());
    let mut native_jury_send_msgs: Vec<CosmosMsg> = Vec::with_capacity(jurors.len());
    // if not hung, transition the state to Decided because all jurors are in
    // agreement winning choice.
    if can_be_decided {
      trial.status = Status::HasVerdict;
      trial.winner = Some(choice_index as u32);

      let total = trial.price * Uint128::from(trial.weight);
      let mut jurors_total_claim_amount = Uint128::zero();

      // build transfer msgs for auto-sending rewards to jury members
      if total > Uint128::zero() {
        match trial.token.clone() {
          Token::Cw20 { address } => {
            for juror in jurors.iter() {
              let amount = Uint128::from(juror.pct) * total / Uint128::from(100u128);
              jurors_total_claim_amount += amount;
              cw20_jury_transfer_submsgs.push(build_cw20_transfer_msg(
                &env.contract.address,
                &juror.address,
                &address,
                amount,
              )?)
            }
          },
          Token::Native { denom } => {
            for juror in jurors.iter() {
              let amount = Uint128::from(juror.pct) * total / Uint128::from(100u128);
              jurors_total_claim_amount += amount;
              native_jury_send_msgs.push(build_native_send_msg(&juror.address, &denom, amount)?);
            }
          },
        }
        // save the remainder of the rewards eligible for claims by winning voters
        VOTERS_TOTAL_CLAIM_AMOUNT.save(deps.storage, &(total - jurors_total_claim_amount))?;
      }
    }
    // persist all accumulated updates to Trial and return response with
    // msgs for performing transfers to jury members
    TRIAL.save(deps.storage, &trial)?;

    let mut response = Response::new().add_attributes(vec![attr("action", "decide")]);

    if cw20_jury_transfer_submsgs.len() > 0 {
      response = response.add_submessages(cw20_jury_transfer_submsgs)
    } else if native_jury_send_msgs.len() > 0 {
      response = response.add_messages(native_jury_send_msgs);
    }
    Ok(response)
  } else {
    // the tx sender isn't a registered juror
    return Err(ContractError::NotAuthorized {});
  }
}
