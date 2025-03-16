use std::collections::HashMap;
use std::fs::read_to_string;

use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl_program = ToolboxIdlProgram::try_parse_from_str(
        &read_to_string("./tests/fixtures/idl_anchor_0_26.json").unwrap(),
    )
    .unwrap();
    // IDL instruction
    let idl_instruction =
        idl_program.get_idl_instruction("createDeal").unwrap();
    // Program
    let program_id = Pubkey::new_unique();
    // Prepare instruction accounts addresses
    let instruction_addresses = HashMap::from_iter([
        ("owner".to_string(), Pubkey::new_unique()),
        ("borrower".to_string(), Pubkey::new_unique()),
        ("globalMarketState".to_string(), Pubkey::new_unique()),
        ("systemProgram".to_string(), Pubkey::new_unique()),
    ]);
    // Prepare instruction args
    let instruction_payload = json!({
        "maxFundingDuration": 42,
        "dealName": "deal hello world",
        "arrangementFees": 41,
        "arrangementFeePercentage": {
            "numerator": 100,
            "denominator": 1,
        },
        "migrated": true,
    });
    // Find missing instruction accounts
    let instruction_addresses = idl_instruction.find_addresses_with_snapshots(
        &program_id,
        &instruction_addresses,
        &instruction_payload,
        &HashMap::from_iter([(
            "borrowerInfo".to_string(),
            (
                idl_program
                    .get_idl_account("BorrowerInfo")
                    .unwrap()
                    .content_type_full
                    .clone(),
                json!({
                    "numOfDeals": 42,
                }),
            ),
        )]),
    );
    // Check that we can compile it and then decompile it
    assert_eq!(
        (
            program_id,
            instruction_addresses.clone(),
            instruction_payload.clone()
        ),
        idl_instruction
            .decompile(
                &idl_instruction
                    .compile(
                        &program_id,
                        &instruction_addresses,
                        &instruction_payload,
                    )
                    .unwrap()
            )
            .unwrap()
    );
}
