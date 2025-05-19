use std::collections::HashMap;
use std::fs::read_to_string;

use serde_json::json;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl_program = ToolboxIdlProgram::try_parse_from_str(
        &read_to_string("./tests/fixtures/idl_anchor_30.json").unwrap(),
    )
    .unwrap();
    // Important account addresses
    let program_id = Pubkey::new_unique();
    let payer = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let collateral_mint = Pubkey::new_unique();
    let redeemable_mint = Pubkey::new_unique();
    let campaign = Pubkey::find_program_address(
        &[b"Campaign", &11u64.to_le_bytes()],
        &program_id,
    )
    .0;
    // Prepare instruction payload
    let mut instruction_payload_metadata_bytes = vec![];
    for index in 0..512 {
        instruction_payload_metadata_bytes.push(json!(index % 100));
    }
    let instruction_payload = json!({
        "params": {
            "index": 11,
            "funding_goal_collateral_amount": 41,
            "funding_phase_duration_seconds": 42,
            "metadata": {
                "length": 22,
                "bytes": instruction_payload_metadata_bytes,
            }
        },
    });
    // Prepare instruction known accounts addresses
    let instruction_addresses = HashMap::from_iter([
        ("payer".to_string(), payer),
        ("authority".to_string(), authority),
        ("collateral_mint".to_string(), collateral_mint),
        ("redeemable_mint".to_string(), redeemable_mint),
    ]);
    // Useful instruction
    let idl_instruction =
        idl_program.instructions.get("campaign_create").unwrap();
    // Resolve missing instruction accounts
    let instruction_addresses = idl_instruction.find_addresses(
        &program_id,
        &instruction_payload,
        &instruction_addresses,
    );
    // Actually generate the instruction
    let instruction = idl_instruction
        .encode(&program_id, &instruction_payload, &instruction_addresses)
        .unwrap();
    // Generate expected accounts
    let campaign_collateral =
        ToolboxEndpoint::find_spl_associated_token_account(
            &campaign,
            &collateral_mint,
        );
    let a_token_program = ToolboxEndpoint::SPL_ASSOCIATED_TOKEN_PROGRAM_ID;
    let token_program = ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID;
    let system_program = ToolboxEndpoint::SYSTEM_PROGRAM_ID;
    // Check instruction content
    assert_eq!(program_id, instruction.program_id);
    // Check instruction data
    assert_eq!(8 + 8 + 8 + 4 + 2 + 512, instruction.data.len());
    assert_eq!(11u64.to_le_bytes(), &instruction.data[8..16]);
    assert_eq!(41u64.to_le_bytes(), &instruction.data[16..24]);
    assert_eq!(42u32.to_le_bytes(), &instruction.data[24..28]);
    assert_eq!(22u16.to_le_bytes(), &instruction.data[28..30]);
    // Check instruction accounts
    assert_eq!(9, instruction.accounts.len());
    assert_account(payer, true, true, instruction.accounts.first());
    assert_account(authority, false, true, instruction.accounts.get(1));
    assert_account(campaign, true, false, instruction.accounts.get(2));
    assert_account(
        campaign_collateral,
        true,
        false,
        instruction.accounts.get(3),
    );
    assert_account(collateral_mint, false, false, instruction.accounts.get(4));
    assert_account(redeemable_mint, true, true, instruction.accounts.get(5));
    assert_account(a_token_program, false, false, instruction.accounts.get(6));
    assert_account(token_program, false, false, instruction.accounts.get(7));
    assert_account(system_program, false, false, instruction.accounts.get(8));
}

fn assert_account(
    address: Pubkey,
    writable: bool,
    signer: bool,
    account: Option<&AccountMeta>,
) {
    let account = account.unwrap();
    assert_eq!(address, account.pubkey);
    assert_eq!(writable, account.is_writable);
    assert_eq!(signer, account.is_signer);
}
