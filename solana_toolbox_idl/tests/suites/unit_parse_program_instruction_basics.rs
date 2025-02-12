use std::vec;

use serde_json::json;
use solana_toolbox_idl::ToolboxIdl;
use solana_toolbox_idl::ToolboxIdlPrimitive;
use solana_toolbox_idl::ToolboxIdlProgramInstruction;
use solana_toolbox_idl::ToolboxIdlProgramInstructionAccount;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypeFlatFields;
use solana_toolbox_idl::ToolboxIdlTypeFull;
use solana_toolbox_idl::ToolboxIdlTypeFullFields;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl1 = ToolboxIdl::try_from_value(&json!({
        "instructions": [
            {
                "name": "my_instruction",
                "discriminator": [195, 241, 184, 14, 127, 155, 68, 53],
                "accounts": [
                    { "name": "account_ws", "signer": true, "writable": true },
                    { "name": "account_rs", "signer": true, "writable": false },
                    { "name": "account_w", "signer": false, "writable": true },
                    { "name": "account_r", "signer": false, "writable": false },
                ],
                "args": [
                    { "name": "arg", "type": {"vec": "u8"} },
                ],
            },
        ],
    }))
    .unwrap();
    let idl2 = ToolboxIdl::try_from_value(&json!({
        "instructions": [
            {
                "name": "my_instruction",
                "accounts": [
                    { "name": "account_ws", "signer": true, "writable": true },
                    { "name": "account_rs", "signer": true },
                    { "name": "account_w", "writable": true },
                    { "name": "account_r" },
                ],
                "args": [
                    { "name": "arg", "type": {"vec": "u8"} },
                ],
            },
        ],
    }))
    .unwrap();
    let idl3 = ToolboxIdl::try_from_value(&json!({
        "instructions": {
            "my_instruction": {
                "discriminator": [195, 241, 184, 14, 127, 155, 68, 53],
                "accounts": [
                    { "name": "account_ws", "isSigner": true, "isMut": true },
                    { "name": "account_rs", "isSigner": true },
                    { "name": "account_w", "isMut": true },
                    { "name": "account_r" },
                ],
                "args": [
                    { "name": "arg", "vec": "u8" },
                ],
            },
        },
    }))
    .unwrap();
    let idl4 = ToolboxIdl::try_from_value(&json!({
        "instructions": {
            "my_instruction": {
                "accounts": [
                    { "name": "account_ws", "isSigner": true, "isMut": true },
                    { "name": "account_rs", "isSigner": true },
                    { "name": "account_w", "isMut": true },
                    { "name": "account_r" },
                ],
                "args": [
                    { "name": "arg", "vec": "u8" },
                ],
            },
        },
    }))
    .unwrap();
    // Assert that all are equivalent
    assert_eq!(idl1, idl2);
    assert_eq!(idl1, idl3);
    assert_eq!(idl1, idl4);
    // Assert that the content is correct
    assert_eq!(
        idl1.program_instructions.get("my_instruction").unwrap(),
        &ToolboxIdlProgramInstruction {
            name: "my_instruction".to_string(),
            discriminator: vec![195, 241, 184, 14, 127, 155, 68, 53],
            accounts: vec![
                ToolboxIdlProgramInstructionAccount {
                    index: 1,
                    name: "account_ws".to_string(),
                    is_writable: true,
                    is_signer: true,
                    address: None,
                    pda: None
                },
                ToolboxIdlProgramInstructionAccount {
                    index: 2,
                    name: "account_rs".to_string(),
                    is_writable: false,
                    is_signer: true,
                    address: None,
                    pda: None
                },
                ToolboxIdlProgramInstructionAccount {
                    index: 3,
                    name: "account_w".to_string(),
                    is_writable: true,
                    is_signer: false,
                    address: None,
                    pda: None
                },
                ToolboxIdlProgramInstructionAccount {
                    index: 4,
                    name: "account_r".to_string(),
                    is_writable: false,
                    is_signer: false,
                    address: None,
                    pda: None
                },
            ],
            data_type_flat: ToolboxIdlTypeFlat::Struct {
                fields: ToolboxIdlTypeFlatFields::Named(vec![(
                    "arg".to_string(),
                    ToolboxIdlTypeFlat::Vec {
                        items: Box::new(ToolboxIdlTypeFlat::Primitive {
                            primitive: ToolboxIdlPrimitive::U8
                        }),
                    },
                )])
            },
            data_type_full: ToolboxIdlTypeFull::Struct {
                fields: ToolboxIdlTypeFullFields::Named(vec![(
                    "arg".to_string(),
                    ToolboxIdlTypeFull::Vec {
                        items: Box::new(ToolboxIdlTypeFull::Primitive {
                            primitive: ToolboxIdlPrimitive::U8
                        }),
                    },
                )])
            },
        }
    )
}
