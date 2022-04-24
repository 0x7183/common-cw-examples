
use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response, StdResult,Uint128};


use crate::error::ContractError;

use crate::state::{PoolInfo, CONFIG, DEPOSITORS};

use cosmwasm_bignumber::{Uint256};


use cosmwasm_std::*;
use std::ops::{Div, Mul};
use std::str::FromStr;

use crate::querier::anchor::deduct_tax;
use crate::querier::anchor::{self, epoch_state};


// Execute deposit
pub fn execute_deposit(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {

    // Load config data

    let config = CONFIG.load(deps.storage)?;

    let stable_denom = "uusd";

    // Extract coin from transaction
    let coin_amount: Uint128 = info
        .funds
        .iter()
        .find(|c| c.denom == stable_denom)
        .map(|c| Uint128::from(c.amount))
        .unwrap_or_else(Uint128::zero);

    // Check it
    if coin_amount.is_zero() {
        return Err(ContractError::NotAllowZeroAmount {});
    }
    if info.funds.len() > 1 {
        return Err(ContractError::NotAllowOtherDenoms {
            denom: config.stable_denom,});
    }
    // Extract depositor addr
    let deposit_addr = info.sender.to_string();
    // Update deposit count
    let new_deposit_count = config.deposit_count + 1;
    // Update item CONFIG
    CONFIG.update(
        deps.storage,
        |x| -> StdResult<_> {
            let mut config = x;
            config.deposit_count = new_deposit_count;
            Ok(config)
        },
    )?;
    // Create our item
    let pool = PoolInfo {
        id: new_deposit_count.to_string(),
        amount: Uint256::from(coin_amount),
        denom: stable_denom.to_string(),
        aust_amount: Uint256::zero(),
        exchange_rate_prev: Uint256::zero().to_string(),
        depositor_addr: deposit_addr.clone(),
    };

    // Store it into our Map
    DEPOSITORS.save(
        deps.storage,
        (&deposit_addr, &new_deposit_count.to_string()),
        &pool.clone()
    )?;
    
    // let the handler deposit the amount into anchor
    deposit(deps, _env, info, coin_amount, &deposit_addr)
}

// Execute withdrawal, only depositor can execute it 
pub fn execute_withdrawal(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: String,
) -> Result<Response, ContractError> {

    // Load data from maps
    let pool = DEPOSITORS.load(deps.storage, (&info.sender.to_string(), &id))?;

    // Extract aust_amount
    let aust_amount: Uint256 = pool.aust_amount;
    // Extract depositor addr
    let rcpt_addr = deps.api.addr_canonicalize(&pool.depositor_addr)?;

    // Removing deposit from map
    DEPOSITORS.remove(deps.storage, (&info.sender.to_string(), &id));

    // redeem the aust
    redeem(
        deps.as_ref(),
        _env, info,
        aust_amount, 
        rcpt_addr, 
    )
}


// Handler for execute_deposit
pub fn deposit(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    received: Uint128,
    deposit_addr: &str,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let epoch_state = epoch_state(deps.as_ref(), _env.clone(), &config.moneymarket)?;

    let ust_amount = deduct_tax(
        deps.as_ref(),
        Coin {
            denom: config.stable_denom.clone(),
            amount: received,
        },
    )?
    .amount;

    let aust_amount = Uint256::from(ust_amount).div(epoch_state.exchange_rate);
    
    DEPOSITORS.update(
        deps.storage,
        (&deposit_addr, &config.deposit_count.to_string()),
        |x| -> StdResult<_> {
            let mut info = x.unwrap();
            info.aust_amount = Uint256::from_str(&aust_amount.to_string())?;
            info.exchange_rate_prev = epoch_state.exchange_rate.to_string();
            Ok(info)
        },
    )?;

    Ok(Response::new()
        .add_messages(anchor::deposit_stable_msg(
            deps.as_ref(),
            &config.moneymarket,
            &config.stable_denom,
            received.into(),
        )?)
        .add_attribute("action", "deposit")
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("amount", aust_amount.clone().to_string())
        .add_attribute("id", &config.deposit_count.to_string())
        .add_attribute("exchange_rate_prev", epoch_state.exchange_rate.to_string()))

}
// Handler for  execute_withdrawall

pub fn redeem(
    deps: Deps,
    _env: Env,
    _info: MessageInfo,
    aust_amount: Uint256,
    rcpt_address: CanonicalAddr,
) -> Result<Response, ContractError> {

    let config = CONFIG.load(deps.storage)?;

    let epoch_state = anchor::epoch_state(deps, _env.clone(), &config.moneymarket)?;
    let rcpt_ust_amount = aust_amount.mul(epoch_state.exchange_rate);

    let rcpt_redeem_amount = deduct_tax(
        deps,
        Coin {
            denom: config.stable_denom.clone(),
            amount: deduct_tax(
                deps,
                Coin {
                    denom: config.stable_denom.clone(),
                    amount: rcpt_ust_amount.into(),
                },
            )?
            .amount,
        },
    )?;


    Ok(Response::new()
        .add_messages(anchor::redeem_stable_msg(
            deps,
            &config.moneymarket,
            &config.aterra_address,
            aust_amount.into(),
        )?)
        .add_message(CosmosMsg::Bank(BankMsg::Send {
            to_address: deps.api.addr_humanize(&rcpt_address).unwrap().into_string(),
            amount: vec![coin(
                u128::from(rcpt_redeem_amount.amount),
                rcpt_redeem_amount.denom.clone(),
            )],
        }))
        .add_attribute("action", "redeem")
        .add_attribute("sender", _env.contract.address)
        .add_attribute("rcpt_amount", rcpt_redeem_amount.to_string())
        .add_attribute("recipient", rcpt_address.clone().to_string()))
}