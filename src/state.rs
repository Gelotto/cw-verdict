use crate::error::ContractError;
use crate::models::{Ballot, Choice, Juror, Status, Vote};
use crate::msg::InstantiateMsg;
use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Something {
  pub value: Option<String>,
}

pub const SOMETHING: Item<Something> = Item::new("something");
pub const BALLOT: Item<Ballot> = Item::new("ballot");
pub const VOTES: Map<(u32, Addr), Vote> = Map::new("votes");
pub const JURORS: Map<Addr, Juror> = Map::new("decisions");
pub const CLAIM_AMOUNT: Item<Uint128> = Item::new("claim_amount");
pub const HAS_CLAIMED: Map<Addr, bool> = Map::new("has_claimed");

/// Initialize contract state data.
pub fn initialize(
  deps: DepsMut,
  _env: &Env,
  info: &MessageInfo,
  msg: &InstantiateMsg,
) -> Result<(), ContractError> {
  // initialize ballot
  let ballot = Ballot {
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

  BALLOT.save(deps.storage, &ballot)?;

  // initialize decision records
  for params in msg.jury.iter() {
    JURORS.save(
      deps.storage,
      params.address.clone(),
      &Juror {
        address: params.address.clone(),
        name: params.name.clone(),
        url: params.url.clone(),
        choice: None,
        logs: None,
      },
    )?;
  }

  Ok(())
}
