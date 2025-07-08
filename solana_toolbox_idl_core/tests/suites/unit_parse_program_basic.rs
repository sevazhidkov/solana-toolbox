use serde_json::json;
use solana_toolbox_idl_core::ToolboxIdlAccount;
use solana_toolbox_idl_core::ToolboxIdlError;
use solana_toolbox_idl_core::ToolboxIdlInstruction;
use solana_toolbox_idl_core::ToolboxIdlInstructionAccount;
use solana_toolbox_idl_core::ToolboxIdlProgram;
use solana_toolbox_idl_core::ToolboxIdlTypeFlat;
use solana_toolbox_idl_core::ToolboxIdlTypeFlatFieldNamed;
use solana_toolbox_idl_core::ToolboxIdlTypeFlatFields;
use solana_toolbox_idl_core::ToolboxIdlTypeFull;
use solana_toolbox_idl_core::ToolboxIdlTypeFullFieldNamed;
use solana_toolbox_idl_core::ToolboxIdlTypeFullFields;
use solana_toolbox_idl_core::ToolboxIdlTypePrimitive;

#[tokio::test]
pub async fn run() {
    // Create IDLs on the fly
    let idl_program1 = ToolboxIdlProgram::try_parse(&json!({
        "instructions": {
            "my_ix": {
                "docs": ["my ix doc"],
                "accounts": [
                    { "name": "authority", "signer": true },
                    { "name": "content", "writable": true },
                    { "name": "optional", "optional": true },
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
                "docs": ["This is an error"],
                "msg": "My error message",
            },
        },
    }))
    .unwrap();
    let idl_program2 = ToolboxIdlProgram::try_parse(&json!({
        "instructions": [
            {
                "name": "my_ix",
                "docs": ["my ix doc"],
                "accounts": [
                    { "name": "authority", "isSigner": true },
                    { "name": "content", "isMut": true },
                    { "name": "optional", "isOptional": true },
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
                "docs": ["This is an error"],
            },
        ],
    }))
    .unwrap();
    // Assert that both versions are equivalent
    assert_eq!(idl_program1, idl_program2);
    // Assert instruction was parsed correctly
    assert_eq!(
        *idl_program1.instructions.get("my_ix").unwrap(),
        ToolboxIdlInstruction {
            name: "my_ix".to_string(),
            docs: Some(json!(["my ix doc"])),
            discriminator: vec![38, 19, 70, 194, 0, 59, 80, 114],
            accounts: vec![
                ToolboxIdlInstructionAccount {
                    name: "authority".to_string(),
                    docs: None,
                    writable: false,
                    signer: true,
                    optional: false,
                    address: None,
                    pda: None
                },
                ToolboxIdlInstructionAccount {
                    name: "content".to_string(),
                    docs: None,
                    writable: true,
                    signer: false,
                    optional: false,
                    address: None,
                    pda: None
                },
                ToolboxIdlInstructionAccount {
                    name: "optional".to_string(),
                    docs: None,
                    writable: false,
                    signer: false,
                    optional: true,
                    address: None,
                    pda: None
                }
            ],
            args_type_flat_fields: ToolboxIdlTypeFlatFields::Named(vec![
                ToolboxIdlTypeFlatFieldNamed {
                    name: "index".to_string(),
                    docs: None,
                    content: ToolboxIdlTypePrimitive::U32.into()
                },
                ToolboxIdlTypeFlatFieldNamed {
                    name: "id".to_string(),
                    docs: None,
                    content: ToolboxIdlTypePrimitive::I64.into()
                },
            ]),
            args_type_full_fields: ToolboxIdlTypeFullFields::Named(vec![
                ToolboxIdlTypeFullFieldNamed {
                    name: "index".to_string(),
                    content: ToolboxIdlTypePrimitive::U32.into()
                },
                ToolboxIdlTypeFullFieldNamed {
                    name: "id".to_string(),
                    content: ToolboxIdlTypePrimitive::I64.into()
                },
            ]),
            return_type_flat: ToolboxIdlTypeFlat::nothing(),
            return_type_full: ToolboxIdlTypeFull::nothing()
        }
        .into()
    );
    // Assert account was parsed correctly
    assert_eq!(
        *idl_program1.accounts.get("MyAccount").unwrap(),
        ToolboxIdlAccount {
            name: "MyAccount".to_string(),
            docs: Some(json!(vec!["My Account doc"])),
            space: None,
            blobs: vec![],
            discriminator: vec![246, 28, 6, 87, 251, 45, 50, 42],
            content_type_flat: ToolboxIdlTypeFlat::Struct {
                fields: ToolboxIdlTypeFlatFields::Named(vec![
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "field1".to_string(),
                        docs: None,
                        content: ToolboxIdlTypePrimitive::U64.into()
                    },
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "field2".to_string(),
                        docs: None,
                        content: ToolboxIdlTypePrimitive::U32.into()
                    }
                ])
            },
            content_type_full: ToolboxIdlTypeFull::Struct {
                fields: ToolboxIdlTypeFullFields::Named(vec![
                    ToolboxIdlTypeFullFieldNamed {
                        name: "field1".to_string(),
                        content: ToolboxIdlTypePrimitive::U64.into()
                    },
                    ToolboxIdlTypeFullFieldNamed {
                        name: "field2".to_string(),
                        content: ToolboxIdlTypePrimitive::U32.into()
                    }
                ])
            }
        }
        .into()
    );
    // Assert error was parsed correctly
    assert_eq!(
        *idl_program1.errors.get("MyError").unwrap(),
        ToolboxIdlError {
            name: "MyError".to_string(),
            docs: Some(json!(["This is an error"])),
            code: 4242,
            msg: Some("My error message".to_string())
        }
        .into()
    )
}
