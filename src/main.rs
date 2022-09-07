use clap::Parser;
use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use std::env::current_dir;
use std::fs::create_dir_all;

#[derive(clap::Parser)] // requires `derive` feature
#[clap(author, version, about, long_about = None)]
enum Cli {
    Schema(Schema),
}

#[derive(clap::Args)]
#[clap(long_about = "Generates JSON schema of every contracts' interfaces")]
struct Schema {}

fn main() {
    match Cli::parse() {
        Cli::Schema(_) => {
            create_core_schemas();
            create_market_schemas();
            create_oracle_schemas();
            create_treasury_schemas();
        }
    }
}

fn create_core_schemas() {
    use noi_interface::core::{ExecuteMsg, InstantiateMsg, QueryMsg};

    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema/core");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
}

fn create_market_schemas() {
    use noi_interface::market::{ExecuteMsg, InstantiateMsg, QueryMsg};

    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema/market");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
}

fn create_oracle_schemas() {
    use noi_interface::oracle::{ExecuteMsg, InstantiateMsg, QueryMsg};

    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema/oracle");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
}

fn create_treasury_schemas() {
    use noi_interface::treasury::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg};

    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema/treasury");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(ConfigResponse), &out_dir);
}
