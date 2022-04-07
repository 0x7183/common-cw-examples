<<<<<<< HEAD
=======
use cosmwasm_std::{Addr, Uint128};
>>>>>>> ba91725770e3aaa1c8637d6b9c1093f7c09b4465
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
<<<<<<< HEAD
    pub count: i32,
=======
>>>>>>> ba91725770e3aaa1c8637d6b9c1093f7c09b4465
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
<<<<<<< HEAD
    Increment {},
    Reset { count: i32 },
=======
    Send {to_address: Addr},
>>>>>>> ba91725770e3aaa1c8637d6b9c1093f7c09b4465
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
<<<<<<< HEAD
    // GetCount returns the current count as a json-encoded number
    GetCount {},
=======
    // GetTransaction returns the transaction with id
    GetTransaction {address: String, id: String},
>>>>>>> ba91725770e3aaa1c8637d6b9c1093f7c09b4465
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
<<<<<<< HEAD
pub struct CountResponse {
    pub count: i32,
=======
pub struct TransactionResponse {
    pub sender: Addr,
    pub receiver: Addr,
    pub amount: Uint128,
>>>>>>> ba91725770e3aaa1c8637d6b9c1093f7c09b4465
}
