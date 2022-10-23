use cosmwasm_std::{Addr, Timestamp, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Status {
  Active,
  Deciding,
  Decided,
  Hung,
  Canceled,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vote {
  pub choice: u32,
  pub weight: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Token {
  Native { denom: String },
  Cw20 { address: Addr },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Choice {
  pub text: String,
  pub tally: u32,
  pub weight: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Background {
  Value(String),
  Video { provider: String, id: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Font {
  pub family: String,
  pub color: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProgrammingLanguage {
  Python,
  TypeScript,
  Rust,
  Bash,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Verdict {
  pub script: String,
  pub language: ProgrammingLanguage,
  pub start: Timestamp,
  pub duration: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Style {
  pub background: Background,
  pub font: Font,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Juror {
  pub address: Addr,
  pub name: String,
  pub url: Option<String>,
  pub choice: Option<u32>,
  pub logs: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Ballot {
  pub owner: Addr,
  pub prompt: String,
  pub choices: Vec<Choice>,
  pub verdict: Verdict,
  pub status: Status,
  pub token: Token,
  pub price: Uint128,
  pub style: Style,
  pub weight: u32,
  pub winner: Option<u32>,
}

impl Ballot {
  pub fn is_active(&self) -> bool {
    self.status == Status::Active
  }

  pub fn is_deciding(&self) -> bool {
    self.status == Status::Deciding
  }

  pub fn is_decided(&self) -> bool {
    self.status == Status::Decided
  }

  pub fn is_canceled(&self) -> bool {
    self.status == Status::Canceled
  }

  pub fn is_hung(&self) -> bool {
    self.status == Status::Hung
  }

  pub fn can_be_canceled(&self) -> bool {
    self.status == Status::Active || self.status == Status::Deciding
  }
}
