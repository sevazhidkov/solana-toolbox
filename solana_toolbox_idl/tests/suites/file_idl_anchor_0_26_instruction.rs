use std::collections::HashMap;
use std::fs::read_to_string;

use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdl;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl_string =
        read_to_string("./tests/fixtures/dummy_idl_anchor_0_26.json").unwrap();
    let idl = ToolboxIdl::try_from_str(&idl_string).unwrap();
    // Program
    let program_id = Pubkey::new_unique();
    // Prepare instruction accounts addresses
    let instruction_accounts_addresses = HashMap::from_iter([
        ("owner".to_string(), Pubkey::new_unique()),
        ("borrower".to_string(), Pubkey::new_unique()),
        ("globalMarketState".to_string(), Pubkey::new_unique()),
        ("systemProgram".to_string(), Pubkey::new_unique()),
    ]);
    // Prepare instruction accounts values
    let instruction_accounts_values = json!({
        "borrowerInfo": {
            "numOfDeals": 42,
        }
    });
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
    // Resolve missing instruction accounts
    let instruction_accounts_addresses = idl
        .fill_instruction_accounts_addresses(
            &program_id,
            "createDeal",
            &instruction_accounts_addresses,
            &instruction_accounts_values.as_object().unwrap(),
            instruction_args_value.as_object().unwrap(),
        )
        .unwrap();
    // Compile the instruction data
    let instruction_data = &idl
        .generate_instruction(
            &program_id,
            "createDeal",
            &instruction_accounts_addresses,
            instruction_args_value.as_object().unwrap(),
        )
        .unwrap();
    eprintln!("instruction_data: {:?}", instruction_data);
}
