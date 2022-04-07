#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
<<<<<<< HEAD
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};
=======
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, CosmosMsg, BankMsg, coin, Uint128};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, TransactionResponse};
use crate::state::{Config, CONFIG, HISTORY, History};
>>>>>>> ba91725770e3aaa1c8637d6b9c1093f7c09b4465

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:send";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
<<<<<<< HEAD
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

=======
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
>>>>>>> ba91725770e3aaa1c8637d6b9c1093f7c09b4465
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
<<<<<<< HEAD
        ExecuteMsg::Increment {} => try_increment(deps),
        ExecuteMsg::Reset { count } => try_reset(deps, info, count),
    }
}

pub fn try_increment(deps: DepsMut) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count += 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_increment"))
}
pub fn try_reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.count = count;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "reset"))
=======
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
>>>>>>> ba91725770e3aaa1c8637d6b9c1093f7c09b4465
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
<<<<<<< HEAD
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
    }
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(CountResponse { count: state.count })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }
}
=======
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

>>>>>>> ba91725770e3aaa1c8637d6b9c1093f7c09b4465
