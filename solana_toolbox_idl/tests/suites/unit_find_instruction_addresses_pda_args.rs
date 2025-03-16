use std::collections::HashMap;

use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse_from_value(&json!({
        "instructions": {
            "my_ix": {
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
                                { "kind": "arg", "path": "inner.u8" },
                                { "kind": "arg", "path": "inner.u16" },
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
                    { "name": "inner", "fields": [
                        { "name": "u8", "type": "u8" },
                        { "name": "u16", "type": "u16" },
                    ] },
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
        &111u8.to_le_bytes(),
        &222u16.to_le_bytes(),
    ];
    let dummy_pda =
        Pubkey::find_program_address(dummy_seeds, &dummy_program_id).0;
    // Assert that the accounts can be properly resolved
    let instruction_addresses = idl_program
        .get_idl_instruction("my_ix")
        .unwrap()
        .find_addresses(
            &dummy_program_id,
            &HashMap::new(),
            &json!({
                "u8": 77,
                "u16": 78,
                "u32": 79,
                "u64": 80,
                "array_u8_2": [11, 12],
                "vec_u8_3": [21, 22, 23],
                "string": "hello",
                "inner": {
                    "u8": 111,
                    "u16": 222,
                },
            }),
        );
    // Assert that the accounts can be properly resolved
    assert_eq!(*instruction_addresses.get("pda").unwrap(), dummy_pda);
}
