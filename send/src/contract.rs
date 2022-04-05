#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, CosmosMsg, BankMsg, coin, Uint128};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, TransactionResponse};
use crate::state::{Config, CONFIG, HISTORY, History};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:send";
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
        id: 0,
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
        ExecuteMsg::Send{to_address} => execute_send(deps, info, to_address),
    }
}

pub fn execute_send(deps: DepsMut, info: MessageInfo, to_address:Addr) -> Result<Response, ContractError> {
    // Load storage
    let config = CONFIG.load(deps.storage)?;

    // Update the count
    let new_id = config.id + 1;
    let sender = info.sender;

    // Store it
    CONFIG.update(
        deps.storage,
        |x| -> StdResult<_> {
            let mut config = x;
            config.id = new_id;
            Ok(config)
        },
    )?;

    // Extract coin amount
    let coin_amount: Uint128 = info
    .funds
    .iter()
    .find(|c| c.denom == "uusd")
    .map(|c| Uint128::from(c.amount))
    .unwrap_or_else(Uint128::zero);

    // Create History item
    let transaction = History{
        id: new_id.to_string(),
        sender: sender.clone(),
        receiver: to_address.clone(),
        amount: coin_amount,
    };

    // Store the transaction
    HISTORY.save(
        deps.storage,
        (&sender, &new_id.to_string()),
        &transaction
    )?;


    // Send transaction
    Ok(Response::new().add_message(CosmosMsg::Bank(BankMsg::Send {
        to_address: to_address.to_string(),
        amount: vec![coin(
            u128::from(coin_amount),
            "uusd".to_string(),
        )],
    })))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetTransaction{address, id} => to_binary(&query_transaction(deps, address, id)?),
    }
}

fn query_transaction(deps: Deps, address: String, id: String) -> StdResult<TransactionResponse> {

    // Check if the address exist and convert the type
    let checked: Addr = deps.api.addr_validate(&address)?;

    // Load the data
    let info = HISTORY
    .may_load(deps.storage, (&checked, &id))?
    .unwrap();

    // Create response
    Ok(TransactionResponse {
        sender: info.sender,
        receiver: info.receiver,
        amount: info.amount,
    })
}

