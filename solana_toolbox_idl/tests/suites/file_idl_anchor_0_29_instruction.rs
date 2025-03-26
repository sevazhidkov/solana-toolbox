use std::collections::HashMap;
use std::fs::read_to_string;

use serde_json::json;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl_program = ToolboxIdlProgram::try_parse_from_str(
        &read_to_string("./tests/fixtures/idl_anchor_0_29.json").unwrap(),
    )
    .unwrap();
    // Important account addresses
    let program_id = Pubkey::new_unique();
    let payer = Pubkey::new_unique();
    let funding = Pubkey::new_unique();
    let placeholder = Pubkey::new_unique();
    // Actually generate the instruction
    let instruction = idl_program
        .instructions
        .get("initialize_realm")
        .unwrap()
        .encode(
            &program_id,
            &json!({
                "params": {
                    "liquid_insurance_fund_usdc_amount": 41,
                    "phase_one_duration_seconds": 42,
                    "phase_two_duration_seconds": 43,
                },
            }),
            &HashMap::from_iter([
                ("payer".to_string(), payer),
                ("funding".to_string(), funding),
                ("funding_usdc".to_string(), placeholder),
                ("realm".to_string(), placeholder),
                ("realm_usdc".to_string(), placeholder),
                ("uct_mint".to_string(), placeholder),
                ("uxp_mint".to_string(), placeholder),
                ("usdc_mint".to_string(), placeholder),
                ("authority".to_string(), placeholder),
                ("spill".to_string(), placeholder),
                ("system_program".to_string(), placeholder),
                ("token_program".to_string(), placeholder),
            ]),
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
    assert_account(payer, true, true, instruction.accounts.first());
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
