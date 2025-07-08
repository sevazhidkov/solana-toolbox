use std::fs::read_to_string;

use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl_core::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl_program = ToolboxIdlProgram::try_parse_from_str(
        &read_to_string("./tests/fixtures/idl_anchor_30.json").unwrap(),
    )
    .unwrap();
    // Instruction used
    let idl_instruction =
        idl_program.instructions.get("campaign_create").unwrap();
    // Prepare instruction payload
    let mut instruction_payload_metadata_bytes = vec![];
    for index in 0..512 {
        instruction_payload_metadata_bytes.push(json!(index % 100));
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
    // Encode / decode the instruction payload and check that they match the original
    assert_eq!(
        &instruction_payload,
        &idl_instruction
            .decode_payload(
                &idl_instruction
                    .encode_payload(&instruction_payload)
                    .unwrap(),
            )
            .unwrap()
    );
    // IDL Account used
    let idl_account = idl_program.accounts.get("Campaign").unwrap();
    // Prepare account state
    let mut account_state_metadata_bytes = vec![];
    for index in 0..512 {
        account_state_metadata_bytes.push(json!(index % 100));
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
    // Encode/decode the account state and check that it matches the original
    let account_data = idl_account.encode(&account_state).unwrap();
    assert_eq!(account_data.len(), 675);
    assert_eq!(account_state, idl_account.decode(&account_data).unwrap());
    // IDL Account used
    let idl_account = idl_program.accounts.get("Pledge").unwrap();
    let account_state = json!({
        "bump": 44,
        "deposited_collateral_amount": 999,
        "claimed_redeemable_amount": 22,
    });
    // Encode/decode the account content and check that it matches the original
    let account_data = idl_account.encode(&account_state).unwrap();
    assert_eq!(account_data.len(), 25);
    assert_eq!(account_state, idl_account.decode(&account_data).unwrap());
}
