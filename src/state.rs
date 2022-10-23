use crate::error::ContractError;
use crate::models::{Choice, Juror, Status, Trial, Vote};
use crate::msg::InstantiateMsg;
use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Uint128};
use cw_storage_plus::{Item, Map};

pub const TRIAL: Item<Trial> = Item::new("trial");
pub const VOTES: Map<(u32, Addr), Vote> = Map::new("votes");
pub const JURORS: Map<Addr, Juror> = Map::new("decisions");
pub const HAS_CLAIMED: Map<Addr, bool> = Map::new("has_claimed");
pub const VOTERS_TOTAL_CLAIM_AMOUNT: Item<Uint128> = Item::new("voters_total_claim_amount");
pub const CANCEL_REASON: Item<String> = Item::new("cancel_reason");

/// Initialize contract state data.
pub fn initialize(
  deps: DepsMut,
  _env: &Env,
  info: &MessageInfo,
  msg: &InstantiateMsg,
) -> Result<(), ContractError> {
  // initialize trial
  let trial = Trial {
    status: Status::Active,
    owner: info.sender.clone(),
    prompt: msg.prompt.clone(),
    token: msg.token.clone(),
    style: msg.style.clone(),
    verdict: msg.verdict.clone(),
    winner: None,
    weight: 0,
    price: msg.price,
    choices: msg
      .choices
      .iter()
      .map(|text| Choice {
        text: text.clone(),
        weight: 0,
        tally: 0,
      })
      .collect(),
  };

  // initialize decision records
  for params in msg.jury.iter() {
    JURORS.save(
      deps.storage,
      params.address.clone(),
      // TODO: validate juror
      &Juror {
        address: params.address.clone(),
        name: params.name.clone(),
        url: params.url.clone(),
        pct: params.pct,
        choice: None,
        logs: None,
      },
    )?;
  }

  // TODO: validate trial
  TRIAL.save(deps.storage, &trial)?;
  VOTERS_TOTAL_CLAIM_AMOUNT.save(deps.storage, &Uint128::zero())?;

  Ok(())
}
