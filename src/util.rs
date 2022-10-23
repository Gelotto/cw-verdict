use cosmwasm_std::{
  attr, to_binary, Addr, BankMsg, Coin, CosmosMsg, DepsMut, MessageInfo, Response, SubMsg, Uint128,
  WasmMsg,
};
use cw20::{Cw20ExecuteMsg, Cw20QueryMsg};

use crate::error::ContractError;

/// Return a Response that performs a CW20 token transfer to the contract.
/// Validates the payment amount sent in the tx, assuming the contract has been
/// granted the necessary allowance already.
pub fn respond_cw20(
  deps: &DepsMut,
  info: &MessageInfo,
  contract_addr: &Addr,
  cw20_token_address: &Addr,
  amount: Uint128,
  action: &str,
) -> Result<Response, ContractError> {
  validate_cw20_funds(&deps, &info.sender, amount, &cw20_token_address)?;
  let submsg = build_cw20_transfer_msg(
    deps,
    &info.sender,
    contract_addr,
    cw20_token_address,
    amount,
  )?;
  Ok(
    Response::new()
      .add_submessage(submsg)
      .add_attributes(vec![attr("action", action.to_owned())]),
  )
}

pub fn build_cw20_transfer_msg(
  deps: &DepsMut,
  from_address: &Addr,
  to_address: &Addr,
  cw20_token_address: &Addr,
  amount: Uint128,
) -> Result<SubMsg, ContractError> {
  // perform CW20 transfer from sender to contract.  note that the cw20
  // token allowance for this contract must be set.
  Ok(SubMsg::new(WasmMsg::Execute {
    contract_addr: cw20_token_address.clone().into(),
    msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
      owner: from_address.clone().into(),
      recipient: to_address.clone().into(),
      amount,
    })?,
    funds: vec![],
  }))
}

/// Return a Response that performs a bank transfer of native funds to the
/// contract. Validates the payment amount sent in the tx.
pub fn respond_native(
  info: &MessageInfo,
  contract_addr: &Addr,
  ibc_denom: &String,
  amount: Uint128,
  action: &str,
) -> Result<Response, ContractError> {
  // Perform transfer of IBC asset from sender to contract.
  validate_native_funds(&info.funds, amount, ibc_denom)?;
  let send_payment_message = build_native_send_msg(contract_addr, ibc_denom, amount)?;
  Ok(
    Response::new()
      .add_message(send_payment_message)
      .add_attributes(vec![attr("action", action.to_owned())]),
  )
}
/// Return a Response that performs a bank transfer of native funds to the
/// contract. Validates the payment amount sent in the tx.
pub fn build_native_send_msg(
  to_address: &Addr,
  ibc_denom: &String,
  amount: Uint128,
) -> Result<CosmosMsg, ContractError> {
  // Perform transfer of IBC asset from sender to contract.
  Ok(CosmosMsg::Bank(BankMsg::Send {
    to_address: to_address.clone().into_string(),
    amount: vec![Coin::new(amount.u128(), ibc_denom)],
  }))
}

// Check for the payment amount required by querying the CW20 token contract.
fn validate_cw20_funds(
  deps: &DepsMut,
  wallet: &Addr,
  payment_amount: Uint128,
  cw20_token_address: &Addr,
) -> Result<(), ContractError> {
  let resp: cw20::BalanceResponse = deps.querier.query_wasm_smart(
    cw20_token_address.clone(),
    &Cw20QueryMsg::Balance {
      address: wallet.clone().into(),
    },
  )?;
  if resp.balance < payment_amount {
    return Err(ContractError::InsufficientFunds {});
  }
  Ok(())
}

// Check for the exact payment amount required in the tx's funds.
fn validate_native_funds(
  funds: &Vec<Coin>,
  payment_amount: Uint128,
  denom: &String,
) -> Result<(), ContractError> {
  if let Some(coin) = funds.iter().find(|coin| -> bool { coin.denom == *denom }) {
    if coin.amount < payment_amount {
      return Err(ContractError::InsufficientFunds {});
    } else if coin.amount > payment_amount {
      return Err(ContractError::ExcessiveFunds {});
    }
  } else {
    return Err(ContractError::InsufficientFunds {});
  }
  Ok(())
}
