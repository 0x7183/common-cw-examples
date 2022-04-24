#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, BankMsg, CosmosMsg, Addr, Order, coin};
use cw2::set_contract_version;


use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, Deposit, DEPOSITS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:deposit";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    // Instantiate the deposit count
    let config = Config {
        deposit_count: 0,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit {} => execute_deposit(deps, info),
        ExecuteMsg::Withdrawal { id } => execute_withdrawal(deps, info, id),
    }
}

pub fn execute_deposit(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    // Load config
    let config = CONFIG.load(deps.storage)?;

    // Extract the depositor address
    let deposit_addr = info.sender;

    // Extract usd amount
    let coin_amount: Uint128 = info
        .funds
        .iter()
        .find(|c| c.denom == "uusd")
        .map(|c| Uint128::from(c.amount))
        .unwrap_or_else(Uint128::zero);


    // Update the count
    let new_deposit_count = config.deposit_count + 1;

    // Store it
    CONFIG.update(
        deps.storage,
        |x| -> StdResult<_> {
            let mut config = x;
            config.deposit_count = new_deposit_count;
            Ok(config)
        },
    )?;

    // Create a Deposit item
    let deposit = Deposit{
        id: new_deposit_count.to_string(),
        owner: deposit_addr.clone(),
        amount: Uint128::from(coin_amount),
    };

    DEPOSITS.save(
        deps.storage,
        (&deposit_addr, &new_deposit_count.to_string()),
        &deposit
    )?;

    Ok(Response::new().add_attribute("method", "deposit"))
}
pub fn execute_withdrawal(deps: DepsMut, info: MessageInfo, id: String) -> Result<Response, ContractError> {
    // Load data from storage
    let deposits = DEPOSITS.load(deps.storage, (&info.sender, &id))?;

    // Extract amount
    let amount= deposits.amount;

    // Check amount
    if amount == Uint128::zero() {
        return Err(ContractError::NoBalance {});
    }
    // Extract deposit owner
    let rcpt_addr = deposits.owner;

    // Remove deposit from map
    DEPOSITS.remove(deps.storage, (&info.sender, &id));

    Ok(Response::new().add_message(CosmosMsg::Bank(BankMsg::Send {
        to_address: rcpt_addr.to_string(),
        amount: vec![coin(
            u128::from(amount),
            "uusd".to_string(),
        )],
    })))
}



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetDeposits {address} => to_binary(&query_deposits(deps, address)?),
    }
}

pub fn query_deposits(deps: Deps, address: Addr) -> Result<std::vec::Vec<(std::vec::Vec<u8>, Deposit)>, cosmwasm_std::StdError>{

    let all: Vec<_> = DEPOSITS
    .prefix(&address)
    .range(deps.storage, None, None, Order::Ascending)
    .collect::<StdResult<_>>()?;
 
    Ok(all)

}

