#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, CosmosMsg, WasmMsg, QueryRequest, WasmQuery};
use cw2::set_contract_version;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, TransactionResponse};
use crate::state::{Config, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:interact";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {


    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config_data = Config {

        contract_addr: deps.api.addr_canonicalize(msg.contract_addr.as_str())?,
    };
    CONFIG.save(deps.storage, &config_data)?;
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
        ExecuteMsg::Interact {to_address} => execute_interact(deps, info, to_address),
        
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Send {to_address: String},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleQuery {
    GetTransaction {address: String, id: String},
}

pub fn execute_interact(deps: DepsMut, info: MessageInfo, to_address: String) ->  Result<Response, ContractError> {
    

    Ok(Response::new()
    .add_messages(try_interact(
        deps,
        info,
        to_address,
    )?)
    .add_attribute("action", "interact"))
}


pub fn try_interact(deps: DepsMut, info: MessageInfo, to_address: String) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;
    let coin_amount= info.funds;
    let contract_addr = config.contract_addr;

    Ok(vec![CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: deps.api.addr_humanize(&contract_addr).unwrap().to_string(),
        msg: to_binary(&HandleMsg::Send {to_address})?,
        funds: coin_amount,
    })])
    
}



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetInteraction {address, id} => to_binary(&get_interaction(deps, address, id)?),
    }
}

fn get_interaction(deps: Deps, address: String, id: String) ->  StdResult<TransactionResponse>  {

    let config = CONFIG.load(deps.storage)?;
    let contract_addr = config.contract_addr;

    let transaction: TransactionResponse  = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: deps.api.addr_humanize(&contract_addr).unwrap().to_string(),
        msg: to_binary(&HandleQuery::GetTransaction {
            address: address,
            id: id,
        })?,
    }))?;

    Ok(transaction)

}

