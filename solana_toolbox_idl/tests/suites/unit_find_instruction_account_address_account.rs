use std::collections::HashMap;

use serde_json::json;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdl;
use solana_toolbox_idl::ToolboxIdlAccount;
use solana_toolbox_idl::ToolboxIdlInstruction;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl1 = ToolboxIdl::try_from_value(&json!({
        "instructions": {
            "my_instruction": {
                "discriminator": [77, 78],
                "accounts": [
                    {
                        "name": "first",
                        "type": "my_account",
                    },
                    {
                        "name": "pda",
                        "pda": {
                            "seeds": [
                                { "kind": "account", "path": "first.u8" },
                                { "kind": "account", "path": "first.u16" },
                                { "kind": "account", "path": "first.u32" },
                                { "kind": "account", "path": "first.u64" },
                                { "kind": "account", "path": "first.array_u8_2" },
                                { "kind": "account", "path": "first.vec_u8_3" },
                                { "kind": "account", "path": "first.string" },
                            ]
                        }
                    },
                ],
            },
        },
        "accounts": {
            "my_account": {
                "fields": [
                    { "name": "u8", "type": "u8" },
                    { "name": "u16", "type": "u16" },
                    { "name": "u32", "type": "u32" },
                    { "name": "u64", "type": "u64" },
                    { "name": "array_u8_2", "type": ["u8", 2] },
                    { "name": "vec_u8_3", "type": ["u8"] },
                    { "name": "string", "type": "string" },
                ]
            }
        }
    }))
    .unwrap();
    // Keys used during the test
    let dummy_program_id = Pubkey::new_unique();
    let dummy_seeds: &[&[u8]] = &[
        &77u8.to_le_bytes(),
        &78u16.to_le_bytes(),
        &79u32.to_le_bytes(),
        &80u64.to_le_bytes(),
        &[11u8, 12u8],
        &[21u8, 22u8, 23u8],
        b"hello",
    ];
    let dummy_pda =
        Pubkey::find_program_address(dummy_seeds, &dummy_program_id).0;
    // The instruction we'll use
    let instruction = ToolboxIdlInstruction {
        program_id: dummy_program_id,
        name: "my_instruction".to_string(),
        accounts_addresses: HashMap::new(),
        args: Value::Null,
    };
    // Assert that the accounts can be properly resolved
    assert_eq!(
        dummy_pda,
        idl1.find_instruction_account_address(
            &instruction,
            &HashMap::from_iter(vec![(
                "first".to_string(),
                ToolboxIdlAccount {
                    name: "my_account".to_string(),
                    state: json!({
                        "u8": 77,
                        "u16": 78,
                        "u32": 79,
                        "u64": 80,
                        "array_u8_2": [11, 12],
                        "vec_u8_3": [21, 22, 23],
                        "string": "hello",
                    })
                }
            )]),
            "pda",
        )
        .unwrap()
    );
}
