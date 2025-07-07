use std::collections::HashMap;

use serde_json::json;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl_core::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse(&json!({
        "instructions": {
            "my_ix": {
                "discriminator": [77, 78],
                "accounts": [
                    { "name": "signer", "signer": true },
                    { "name": "writable", "writable": true },
                ],
                "args": [
                    { "name": "arg1", "type": {"defined": "MyArg"} },
                    { "name": "arg2", "type": "i16" },
                ]
            }
        },
        "types": {
            "MyArg": {
                "fields": [
                    { "name": "id", "type": "u16" },
                    { "name": "data", "type": {"vec": "u8"} },
                ]
            }
        },
    }))
    .unwrap();
    // Choose the instruction
    let idl_instruction = idl_program.instructions.get("my_ix").unwrap();
    // Check that we can use the manual IDL to encode/decode our IX
    let instruction_program_id = Pubkey::new_unique();
    let instruction_payload = json!({
        "arg1": {
            "id": 42,
            "data": [1, 2, 3]
        },
        "arg2": -2,
    });
    let instruction_addresses = HashMap::from_iter([
        ("signer".to_string(), Pubkey::new_unique()),
        ("writable".to_string(), Pubkey::new_unique()),
    ]);
    let instruction = idl_instruction
        .encode(
            &instruction_program_id,
            &instruction_payload,
            &instruction_addresses,
        )
        .unwrap();
    assert_eq!(
        instruction,
        Instruction {
            program_id: instruction_program_id,
            accounts: vec![
                AccountMeta::new_readonly(
                    *instruction_addresses.get("signer").unwrap(),
                    true
                ),
                AccountMeta::new(
                    *instruction_addresses.get("writable").unwrap(),
                    false
                ),
            ],
            data: vec![77, 78, 42, 0, 3, 0, 0, 0, 1, 2, 3, 254, 255]
        }
    );
    assert_eq!(
        idl_instruction.decode(&instruction).unwrap(),
        (
            instruction_program_id,
            instruction_payload,
            instruction_addresses
        ),
    );
}
