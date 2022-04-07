use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

<<<<<<< HEAD
use send::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use send::state::State;
=======
use send::msg::{TransactionResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use send::state::{Config, History};
>>>>>>> ba91725770e3aaa1c8637d6b9c1093f7c09b4465

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
<<<<<<< HEAD
    export_schema(&schema_for!(State), &out_dir);
    export_schema(&schema_for!(CountResponse), &out_dir);
=======
    export_schema(&schema_for!(Config), &out_dir);
    export_schema(&schema_for!(History), &out_dir);
    export_schema(&schema_for!(TransactionResponse), &out_dir);
>>>>>>> ba91725770e3aaa1c8637d6b9c1093f7c09b4465
}
