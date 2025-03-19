use std::vec;

use serde_json::json;
use solana_toolbox_idl::ToolboxIdlInstruction;
use solana_toolbox_idl::ToolboxIdlInstructionAccount;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypeFlatFields;
use solana_toolbox_idl::ToolboxIdlTypeFull;
use solana_toolbox_idl::ToolboxIdlTypeFullFields;
use solana_toolbox_idl::ToolboxIdlTypePrimitive;

#[tokio::test]
pub async fn run() {
    // TODO - test is_optional parsing and compile/decompile
    // Create IDLs using different shortened formats
    let idl_program1 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "instructions": [
            {
                "name": "my_ix",
                "discriminator": [38, 19, 70, 194, 0, 59, 80, 114],
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
    let idl_program2 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "instructions": [
            {
                "name": "my_ix",
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
    let idl_program3 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "instructions": {
            "my_ix": {
                "discriminator": [38, 19, 70, 194, 0, 59, 80, 114],
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
    let idl_program4 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "instructions": {
            "my_ix": {
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
    assert_eq!(idl_program1, idl_program2);
    assert_eq!(idl_program1, idl_program3);
    assert_eq!(idl_program1, idl_program4);
    // Assert that the content is correct
    assert_eq!(
        *idl_program1.instructions.get("my_ix").unwrap(),
        ToolboxIdlInstruction {
            name: "my_ix".to_string(),
            docs: None,
            discriminator: vec![38, 19, 70, 194, 0, 59, 80, 114],
            accounts: vec![
                ToolboxIdlInstructionAccount {
                    name: "account_ws".to_string(),
                    docs: None,
                    writable: true,
                    signer: true,
                    optional: false,
                    address: None,
                    pda: None
                },
                ToolboxIdlInstructionAccount {
                    name: "account_rs".to_string(),
                    docs: None,
                    writable: false,
                    signer: true,
                    optional: false,
                    address: None,
                    pda: None
                },
                ToolboxIdlInstructionAccount {
                    name: "account_w".to_string(),
                    docs: None,
                    writable: true,
                    signer: false,
                    optional: false,
                    address: None,
                    pda: None
                },
                ToolboxIdlInstructionAccount {
                    name: "account_r".to_string(),
                    docs: None,
                    writable: false,
                    signer: false,
                    optional: false,
                    address: None,
                    pda: None
                },
            ],
            args_type_flat_fields: ToolboxIdlTypeFlatFields::Named(vec![(
                "arg".to_string(),
                ToolboxIdlTypeFlat::Vec {
                    items: Box::new(ToolboxIdlTypeFlat::Primitive {
                        primitive: ToolboxIdlTypePrimitive::U8
                    }),
                },
            )]),
            args_type_full_fields: ToolboxIdlTypeFullFields::Named(vec![(
                "arg".to_string(),
                ToolboxIdlTypeFull::Vec {
                    items: Box::new(ToolboxIdlTypeFull::Primitive {
                        primitive: ToolboxIdlTypePrimitive::U8
                    }),
                },
            )])
            .into(),
        }
        .into()
    )
}
