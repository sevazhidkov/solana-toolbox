use std::{collections::HashMap, fs::read_to_string};

use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdl;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl = ToolboxIdl::try_from_str(
        &read_to_string("./tests/fixtures/dummy_idl_anchor_0_26.json").unwrap(),
    )
    .unwrap();
    // Program
    let program_id = Pubkey::new_unique();
    // Prepare instruction accounts
    let mut instruction_accounts = HashMap::new();
    instruction_accounts.insert("owner".into(), Pubkey::new_unique());
    instruction_accounts.insert("borrower".into(), Pubkey::new_unique());
    instruction_accounts.insert("systemProgram".into(), Pubkey::new_unique());
    // TODO - this should not be necessary
    instruction_accounts
        .insert("globalMarketState".into(), Pubkey::new_unique());
    instruction_accounts.insert("deal".into(), Pubkey::new_unique());
    // Prepare instruction args
    let instruction_args_value = json!({
        "maxFundingDuration": 42,
        "dealName": "deal hello world",
        "arrangementFees": 41,
        "arrangementFeePercentage": {
            "numerator": 100,
            "denominator": 1,
        },
        "migrated": true,
    });
    // Compile the instruction data
    let instruction_data = &idl
        .generate_instruction(
            &program_id,
            "createDeal",
            &instruction_accounts,
            instruction_args_value.as_object().unwrap(),
        )
        .unwrap();
    eprintln!("instruction_data: {:?}", instruction_data);
}
