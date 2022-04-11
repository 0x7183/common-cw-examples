
use crate::state::{DEPOSITORS, PoolInfo};


use cosmwasm_std::{StdResult, Order};
use cosmwasm_std::Deps;



pub fn query_depositor(deps: Deps, address: String) -> Result<std::vec::Vec<(std::vec::Vec<u8>, PoolInfo)>, cosmwasm_std::StdError>{

    let all: Vec<_> = DEPOSITORS
    .prefix(&address)
    .range(deps.storage, None, None, Order::Ascending)
    .collect::<StdResult<_>>()?;
 
    Ok(all)

}


