use std::collections::HashMap;
use std::fs::read_to_string;

use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdlAccount;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlTransactionInstruction;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl_string =
        read_to_string("./tests/fixtures/idl_anchor_0_26.json").unwrap();
    let idl_program = ToolboxIdlProgram::try_parse_from_str(&idl_string).unwrap();
    // Program
    let program_id = Pubkey::new_unique();
    // Prepare instruction accounts addresses
    let instruction_accounts_addresses = HashMap::from_iter([
        ("owner".to_string(), Pubkey::new_unique()),
        ("borrower".to_string(), Pubkey::new_unique()),
        ("globalMarketState".to_string(), Pubkey::new_unique()),
        ("systemProgram".to_string(), Pubkey::new_unique()),
    ]);
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
        .find_instruction_accounts_addresses(
            &ToolboxIdlTransactionInstruction {
                program_id,
                name: "createDeal".to_string(),
                accounts_addresses: instruction_accounts_addresses.clone(),
                args: instruction_args_value.clone(),
            },
            &HashMap::from_iter([(
                "borrowerInfo".to_string(),
                ToolboxIdlAccount {
                    name: "BorrowerInfo".to_string(),
                    state: json!({
                        "numOfDeals": 42,
                    }),
                },
            )]),
        )
        .unwrap();
    // Make an instruction
    let instruction = ToolboxIdlTransactionInstruction {
        program_id,
        name: "createDeal".to_string(),
        accounts_addresses: instruction_accounts_addresses.clone(),
        args: instruction_args_value.clone(),
    };
    // Check that we can compile it and then decompile it
    assert_eq!(
        &instruction,
        &idl.decompile_instruction(
            &idl.compile_instruction(&instruction).unwrap()
        )
        .unwrap()
    );
}
