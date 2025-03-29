use serde_json::json;
use solana_toolbox_idl::ToolboxIdlAccount;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlTransactionError;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypeFlatFields;
use solana_toolbox_idl::ToolboxIdlTypeFull;
use solana_toolbox_idl::ToolboxIdlTypeFullFields;
use solana_toolbox_idl::ToolboxIdlTypePrimitive;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program1 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "instructions": [
            {
                "name": "my_ix",
                "docs": ["my ix doc"],
                "accounts": [
                    { "name": "payer", "isSigner": true },
                    { "name": "authority", "isMut": true },
                ],
                "args": [
                    { "name": "index", "type": "u32" },
                    { "name": "id", "type": "i64" },
                ]
            }
        ],
        "accounts": [
            {
                "name": "MyAccount",
                "docs": ["My Account doc"],
                "type": {
                    "kind": "struct",
                    "fields": [
                        { "name": "field1", "type": "u64" },
                        { "name": "field2", "type": "u32" },
                    ],
                }
            }
        ],
        "errors": [
            {
                "code": 4242,
                "name": "MyError",
                "msg": "My error message",
            },
        ],
    }))
    .unwrap();
    // Create an IDL on the fly
    let idl_program2 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "instructions": {
            "my_ix": {
                "docs": ["my ix doc"],
                "accounts": [
                    { "name": "payer", "signer": true },
                    { "name": "authority", "writable": true },
                ],
                "args": [
                    { "name": "index", "type": "u32" },
                    { "name": "id", "type": "i64" },
                ]
            }
        },
        "accounts": {
            "MyAccount": {
                "docs": ["My Account doc"],
                "fields": [
                    { "name": "field1", "type": "u64" },
                    { "name": "field2", "type": "u32" },
                ],
            }
        },
        "errors": {
            "MyError": {
                "code": 4242,
                "msg": "My error message",
            },
        },
    }))
    .unwrap();
    // Assert that both versions are equivalent
    assert_eq!(idl_program1, idl_program2);
    // Assert instruction was parsed correctly
    let idl_instruction = idl_program1.instructions.get("my_ix").unwrap();
    assert_eq!(idl_instruction.name, "my_ix");
    assert_eq!(
        idl_instruction.discriminator,
        &[38, 19, 70, 194, 0, 59, 80, 114]
    );
    assert_eq!(idl_instruction.accounts[0].name, "payer");
    assert_eq!(idl_instruction.accounts[0].signer, true);
    assert_eq!(idl_instruction.accounts[0].writable, false);
    assert_eq!(idl_instruction.accounts[0].address, None);
    assert_eq!(idl_instruction.accounts[0].pda, None);
    assert_eq!(idl_instruction.accounts[1].name, "authority");
    assert_eq!(idl_instruction.accounts[1].signer, false);
    assert_eq!(idl_instruction.accounts[1].writable, true);
    assert_eq!(idl_instruction.accounts[1].address, None);
    assert_eq!(idl_instruction.accounts[1].pda, None);
    assert_eq!(
        idl_instruction.args_type_flat_fields,
        ToolboxIdlTypeFlatFields::Named(vec![
            (
                "index".to_string(),
                ToolboxIdlTypeFlat::Primitive {
                    primitive: ToolboxIdlTypePrimitive::U32
                }
            ),
            (
                "id".to_string(),
                ToolboxIdlTypeFlat::Primitive {
                    primitive: ToolboxIdlTypePrimitive::I64
                }
            ),
        ])
    );
    // Assert account was parsed correctly
    assert_eq!(
        *idl_program1.accounts.get("MyAccount").unwrap(),
        ToolboxIdlAccount {
            name: "MyAccount".to_string(),
            docs: Some(json!(vec!["My Account doc"])),
            space: None,
            discriminator: vec![246, 28, 6, 87, 251, 45, 50, 42],
            content_type_flat: ToolboxIdlTypeFlat::Struct {
                fields: ToolboxIdlTypeFlatFields::Named(vec![
                    (
                        "field1".to_string(),
                        ToolboxIdlTypeFlat::Primitive {
                            primitive: ToolboxIdlTypePrimitive::U64
                        }
                    ),
                    (
                        "field2".to_string(),
                        ToolboxIdlTypeFlat::Primitive {
                            primitive: ToolboxIdlTypePrimitive::U32
                        }
                    )
                ])
            },
            content_type_full: ToolboxIdlTypeFull::Struct {
                fields: ToolboxIdlTypeFullFields::Named(vec![
                    (
                        "field1".to_string(),
                        ToolboxIdlTypeFull::Primitive {
                            primitive: ToolboxIdlTypePrimitive::U64
                        }
                    ),
                    (
                        "field2".to_string(),
                        ToolboxIdlTypeFull::Primitive {
                            primitive: ToolboxIdlTypePrimitive::U32
                        }
                    )
                ])
            }
            .into()
        }
        .into()
    );
    // Assert error was parsed correctly
    assert_eq!(
        *idl_program1.errors.get("MyError").unwrap(),
        ToolboxIdlTransactionError {
            name: "MyError".to_string(),
            code: 4242,
            msg: "My error message".to_string()
        }
        .into()
    )
}
