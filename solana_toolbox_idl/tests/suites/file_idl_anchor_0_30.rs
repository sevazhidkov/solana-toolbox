use std::collections::HashMap;
use std::fs::read_to_string;

use serde_json::Map;
use serde_json::Number;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdl;

#[tokio::test]
pub async fn file_idl_anchor_0_30() {
    let idl = ToolboxIdl::try_from_str(
        &read_to_string(
            "./tests/fixtures/dummy_crowd_funding_anchor_0_30.json",
        )
        .unwrap(),
    )
    .unwrap();

    let program_id =
        Pubkey::from_str_const("UC2cQRtrbGmvuLKA3Jv719Cc6DS4r661ZRpyZduxu2j");

    let payer = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let campaign = Pubkey::new_unique();
    let collateral_mint = Pubkey::new_unique();
    let redeemable_mint = Pubkey::new_unique();

    let mut instruction_accounts = HashMap::new();
    instruction_accounts.insert("payer".into(), payer);
    instruction_accounts.insert("authority".into(), authority);
    instruction_accounts.insert("campaign".into(), campaign); // TODO - this should be auto-resolved from params
    instruction_accounts.insert("collateral_mint".into(), collateral_mint);
    instruction_accounts.insert("redeemable_mint".into(), redeemable_mint);

    let mut instruction_args_params_metadata_bytes = vec![];
    for _index in 0..512 {
        instruction_args_params_metadata_bytes
            .push(Value::Number(Number::from(7)));
    }

    let mut instruction_args_params_metadata = Map::new();
    instruction_args_params_metadata
        .insert("length".into(), Value::Number(Number::from(2)));
    instruction_args_params_metadata.insert(
        "bytes".into(),
        Value::Array(instruction_args_params_metadata_bytes),
    );

    let mut instruction_args_params = Map::new();
    instruction_args_params
        .insert("index".into(), Value::Number(Number::from(42u64)));
    instruction_args_params.insert(
        "funding_goal_collateral_amount".into(),
        Value::Number(Number::from(42)),
    );
    instruction_args_params.insert(
        "funding_phase_duration_seconds".into(),
        Value::Number(Number::from(42)),
    );
    instruction_args_params.insert(
        "funding_goal_collateral_amount".into(),
        Value::Number(Number::from(42)),
    );
    instruction_args_params.insert(
        "metadata".into(),
        Value::Object(instruction_args_params_metadata),
    );

    let mut instruction_args = Map::new();
    instruction_args
        .insert("params".into(), Value::Object(instruction_args_params));

    let dada = idl
        .generate_instruction(
            &program_id,
            "campaign_create",
            &instruction_accounts,
            &instruction_args,
        )
        .unwrap();
    eprintln!("dada:{:?}", dada);

    panic!("YESSSS :ok:");
}
