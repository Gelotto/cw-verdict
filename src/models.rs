use cosmwasm_std::{Addr, Timestamp, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Status {
  Active,
  Deliberating,
  HasVerdict,
  HungJury,
  Dismissed,
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
  pub minutes: u32,
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
  pub pct: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Trial {
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

impl Trial {
  pub fn is_active(&self) -> bool {
    self.status == Status::Active
  }

  pub fn is_in_deliberations(&self) -> bool {
    self.status == Status::Deliberating
  }

  pub fn has_verdict(&self) -> bool {
    self.status == Status::HasVerdict
  }

  pub fn has_been_canceled(&self) -> bool {
    self.status == Status::Dismissed
  }

  pub fn has_hung_jury(&self) -> bool {
    self.status == Status::HungJury
  }

  pub fn can_be_canceled(&self) -> bool {
    self.status == Status::Active || self.status == Status::Deliberating
  }
}
