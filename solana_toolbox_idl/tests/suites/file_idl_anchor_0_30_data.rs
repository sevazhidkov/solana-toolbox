use std::fs::read_to_string;

use serde_json::json;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdlBreadcrumbs;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl_program = ToolboxIdlProgram::try_parse_from_str(
        &read_to_string("./tests/fixtures/idl_anchor_0_30.json").unwrap(),
    )
    .unwrap();
    // Instruction used
    let idl_instruction =
        idl_program.get_idl_instruction("campaign_create").unwrap();
    // Prepare instruction payload
    let mut instruction_payload_metadata_bytes = vec![];
    for index in 0..512 {
        instruction_payload_metadata_bytes.push(Value::from(index % 100));
    }
    let instruction_payload = json!({
        "params": {
            "index": 42,
            "funding_goal_collateral_amount": 41,
            "funding_phase_duration_seconds": 99,
            "metadata": {
                "length": 22,
                "bytes": instruction_payload_metadata_bytes,
            },
        },
    });
    // Compile / decompile the instruction payload and check that they match the original
    assert_eq!(
        &instruction_payload,
        &idl_instruction
            .decompile_payload(
                &idl_instruction
                    .compile_payload(
                        &instruction_payload,
                        &ToolboxIdlBreadcrumbs::default()
                    )
                    .unwrap(),
                &ToolboxIdlBreadcrumbs::default()
            )
            .unwrap()
    );
    // IDL Account used
    let idl_account = idl_program.get_idl_account("Campaign").unwrap();
    // Prepare account state
    let mut account_state_metadata_bytes = vec![];
    for index in 0..512 {
        account_state_metadata_bytes.push(Value::from(index % 100));
    }
    let account_state = json!({
        "index": 77,
        "bump": 99,
        "authority": Pubkey::new_unique().to_string(),
        "collateral_mint": Pubkey::new_unique().to_string(),
        "redeemable_mint": Pubkey::new_unique().to_string(),
        "funding_goal_collateral_amount": 11,
        "total_deposited_collateral_amount": 22,
        "total_claimed_redeemable_amount": 33,
        "funding_phase_start_unix_timestamp": -44,
        "funding_phase_end_unix_timestamp": -55,
        "extracted_collateral_amount": 66,
        "metadata": {
            "length": 99,
            "bytes": account_state_metadata_bytes,
        }
    });
    // Decompile the account state and check that it matches the original state
    let account_data = idl_account.compile(&account_state).unwrap();
    assert_eq!(account_data.len(), 555);
    assert_eq!(account_state, idl_account.decompile(&account_data).unwrap());
    // IDL Account used
    let idl_account = idl_program.get_idl_account("Pledge").unwrap();
    let account_state = json!({
        "bump": 44,
        "deposited_collateral_amount": 999,
        "claimed_redeemable_amount": 22,
    });
    // Decompile the account content and check that it matches the original
    let account_data = idl_account.compile(&account_state).unwrap();
    assert_eq!(account_data.len(), 17);
    assert_eq!(account_state, idl_account.decompile(&account_data).unwrap());
}
