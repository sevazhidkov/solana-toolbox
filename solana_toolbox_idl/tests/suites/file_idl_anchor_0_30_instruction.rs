use std::collections::HashMap;
use std::fs::read_to_string;

use serde_json::json;
use serde_json::Map;
use serde_json::Value;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_idl::ToolboxIdl;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl = ToolboxIdl::try_from_str(
        &read_to_string("./tests/fixtures/dummy_idl_anchor_0_30.json").unwrap(),
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
    // Prepare instruction accounts
    let mut instruction_accounts_addresses = HashMap::new();
    instruction_accounts_addresses.insert("payer".into(), payer);
    instruction_accounts_addresses.insert("authority".into(), authority);
    instruction_accounts_addresses
        .insert("collateral_mint".into(), collateral_mint);
    instruction_accounts_addresses
        .insert("redeemable_mint".into(), redeemable_mint);
    // Prepare instruction args
    let mut instruction_args_metadata_bytes = vec![];
    for index in 0..512 {
        instruction_args_metadata_bytes.push(Value::from(index % 100));
    }
    let instruction_args_value = json!({
        "params": {
            "index": 11,
            "funding_goal_collateral_amount": 41,
            "funding_phase_duration_seconds": 42,
            "metadata": {
                "length": 22,
                "bytes": instruction_args_metadata_bytes,
            }
        },
    });
    // Resolve missing instruction accounts
    let instruction_accounts_addresses = idl
        .fill_instruction_accounts_addresses(
            &program_id,
            "campaign_create",
            &instruction_accounts_addresses,
            &Map::new(),
            instruction_args_value.as_object().unwrap(),
        )
        .unwrap();
    // Actually generate the instruction
    let instruction = idl
        .generate_instruction(
            &program_id,
            "campaign_create",
            &instruction_accounts_addresses,
            instruction_args_value.as_object().unwrap(),
        )
        .unwrap();
    // Generate expected accounts
    let campaign_collateral =
        ToolboxEndpoint::find_spl_associated_token_account(
            &campaign,
            &collateral_mint,
        );
    let a_token_program =
        pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
    let token_program = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    let system_program = pubkey!("11111111111111111111111111111111");
    // Check instruction content
    assert_eq!(program_id, instruction.program_id);
    // Check instruction data
    assert_eq!(8 + 8 + 8 + 4 + 2 + 512, instruction.data.len());
    assert_eq!(bytemuck::bytes_of::<u64>(&11), &instruction.data[8..16]);
    assert_eq!(bytemuck::bytes_of::<u64>(&41), &instruction.data[16..24]);
    assert_eq!(bytemuck::bytes_of::<u32>(&42), &instruction.data[24..28]);
    assert_eq!(bytemuck::bytes_of::<u16>(&22), &instruction.data[28..30]);
    // Check instruction accounts
    assert_eq!(9, instruction.accounts.len());
    assert_account(payer, true, true, instruction.accounts.get(0));
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
