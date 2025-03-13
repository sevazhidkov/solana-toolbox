use std::collections::HashMap;

use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdlProgramRoot;
use solana_toolbox_idl::ToolboxIdlTransactionInstruction;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl = ToolboxIdlProgramRoot::try_parse_from_value(&json!({
        "instructions": {
            "my_instruction": {
                "discriminator": [77, 78],
                "accounts": [
                    {
                        "name": "pda",
                        "pda": {
                            "seeds": [
                                { "kind": "arg", "path": "u8" },
                                { "kind": "arg", "path": "u16" },
                                { "kind": "arg", "path": "u32" },
                                { "kind": "arg", "path": "u64" },
                                { "kind": "arg", "path": "array_u8_2" },
                                { "kind": "arg", "path": "vec_u8_3" },
                                { "kind": "arg", "path": "string" },
                            ]
                        }
                    },
                ],
                "args": [
                    { "name": "u8", "type": "u8" },
                    { "name": "u16", "type": "u16" },
                    { "name": "u32", "type": "u32" },
                    { "name": "u64", "type": "u64" },
                    { "name": "array_u8_2", "type": ["u8", 2] },
                    { "name": "vec_u8_3", "type": ["u8"] },
                    { "name": "string", "type": "string" },
                ]
            },
        },
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
    let instruction = ToolboxIdlTransactionInstruction {
        program_id: dummy_program_id,
        name: "my_instruction".to_string(),
        accounts_addresses: HashMap::new(),
        args: json!({
            "u8": 77,
            "u16": 78,
            "u32": 79,
            "u64": 80,
            "array_u8_2": [11, 12],
            "vec_u8_3": [21, 22, 23],
            "string": "hello",
        }),
    };
    // Assert that the accounts can be properly resolved
    assert_eq!(
        dummy_pda,
        idl.find_instruction_account_address(
            &instruction,
            &HashMap::new(),
            "pda",
        )
        .unwrap()
    );
}
