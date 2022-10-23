use crate::{
  models::{Style, Token, Verdict},
  state::Something,
};
use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct JurorIntiatiationParams {
  pub address: Addr,
  pub name: String,
  pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
  pub prompt: String,
  pub choices: Vec<String>,
  pub verdict: Verdict,
  pub token: Token,
  pub price: Uint128,
  pub style: Style,
  pub jury: Vec<JurorIntiatiationParams>,
}

/// Executable contract endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
  Vote { choice: u32, weight: u32 },
  Decide { choice: u32, logs: String },
  Cancel {},
  Claim {},
}

/// Custom contract query endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  GetSomething {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetSomethingResponse {
  pub something: Something,
}
