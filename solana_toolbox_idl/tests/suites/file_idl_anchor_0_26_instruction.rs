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
    let idl_instruction = idl_program.instructions.get("create_deal").unwrap();
    // Program
    let program_id = Pubkey::new_unique();
    // Prepare instruction accounts addresses
    let instruction_addresses = HashMap::from_iter([
        ("owner".to_string(), Pubkey::new_unique()),
        ("borrower".to_string(), Pubkey::new_unique()),
        ("global_market_state".to_string(), Pubkey::new_unique()),
        ("system_program".to_string(), Pubkey::new_unique()),
    ]);
    // Prepare instruction args
    let instruction_payload = json!({
        "max_funding_duration": 42,
        "deal_name": "deal hello world",
        "arrangement_fees": 41,
        "arrangement_fee_percentage": {
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
            "borrower_info".to_string(),
            (
                idl_program
                    .accounts
                    .get("BorrowerInfo")
                    .unwrap()
                    .content_type_full
                    .clone(),
                json!({
                    "num_of_deals": 42,
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
