use std::collections::HashMap;
use std::fs::read_to_string;

use serde_json::json;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdl;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl = ToolboxIdl::try_from_str(
        &read_to_string("./tests/fixtures/dummy_idl_anchor_0_29.json").unwrap(),
    )
    .unwrap();
    // Important account addresses
    let program_id = Pubkey::new_unique();
    let payer = Pubkey::new_unique();
    let funding = Pubkey::new_unique();
    let placeholder = Pubkey::new_unique();
    // Prepare instruction accounts
    let mut instruction_accounts = HashMap::new();
    instruction_accounts.insert("payer".into(), payer);
    instruction_accounts.insert("funding".into(), funding);
    instruction_accounts.insert("fundingUsdc".into(), placeholder);
    instruction_accounts.insert("realm".into(), placeholder);
    instruction_accounts.insert("realmUsdc".into(), placeholder);
    instruction_accounts.insert("uctMint".into(), placeholder);
    instruction_accounts.insert("uxpMint".into(), placeholder);
    instruction_accounts.insert("usdcMint".into(), placeholder);
    instruction_accounts.insert("authority".into(), placeholder);
    instruction_accounts.insert("spill".into(), placeholder);
    instruction_accounts.insert("systemProgram".into(), placeholder);
    instruction_accounts.insert("tokenProgram".into(), placeholder);
    // Prepare instruction args
    let instruction_args_value = json!({
        "params": {
            "liquidInsuranceFundUsdcAmount": 41,
            "phaseOneDurationSeconds": 42,
            "phaseTwoDurationSeconds": 43,
        },
    });
    // Actually generate the instruction
    let instruction = idl
        .generate_instruction(
            &program_id,
            "initializeRealm",
            &instruction_accounts,
            instruction_args_value.as_object().unwrap(),
        )
        .unwrap();
    // Check instruction content
    assert_eq!(program_id, instruction.program_id);
    // Check instruction data
    assert_eq!(8 + 8 + 8 + 8, instruction.data.len());
    assert_eq!(bytemuck::bytes_of::<u64>(&41), &instruction.data[8..16]);
    assert_eq!(bytemuck::bytes_of::<u64>(&42), &instruction.data[16..24]);
    assert_eq!(bytemuck::bytes_of::<u64>(&43), &instruction.data[24..32]);
    // Check instruction accounts
    assert_eq!(12, instruction.accounts.len());
    assert_account(payer, true, true, instruction.accounts.get(0));
    assert_account(funding, false, true, instruction.accounts.get(1));
    assert_account(placeholder, true, false, instruction.accounts.get(2));
    assert_account(placeholder, true, false, instruction.accounts.get(3));
    assert_account(placeholder, true, false, instruction.accounts.get(4));
    assert_account(placeholder, true, false, instruction.accounts.get(5));
    assert_account(placeholder, false, false, instruction.accounts.get(6));
    assert_account(placeholder, false, false, instruction.accounts.get(7));
    assert_account(placeholder, false, false, instruction.accounts.get(8));
    assert_account(placeholder, false, false, instruction.accounts.get(9));
    assert_account(placeholder, false, false, instruction.accounts.get(10));
    assert_account(placeholder, false, false, instruction.accounts.get(11));
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
