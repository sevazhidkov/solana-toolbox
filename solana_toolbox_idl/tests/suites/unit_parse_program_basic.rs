use serde_json::json;
use solana_toolbox_idl::ToolboxIdlAccount;
use solana_toolbox_idl::ToolboxIdlError;
use solana_toolbox_idl::ToolboxIdlInstruction;
use solana_toolbox_idl::ToolboxIdlInstructionAccount;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypeFlatFieldNamed;
use solana_toolbox_idl::ToolboxIdlTypeFlatFields;
use solana_toolbox_idl::ToolboxIdlTypeFull;
use solana_toolbox_idl::ToolboxIdlTypeFullFieldNamed;
use solana_toolbox_idl::ToolboxIdlTypeFullFields;
use solana_toolbox_idl::ToolboxIdlTypePrimitive;

#[tokio::test]
pub async fn run() {
    // Create IDLs on the fly
    let idl_program1 = ToolboxIdlProgram::try_parse_from_value(&json!({
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
                "msg": "My error message",
            },
        },
    }))
    .unwrap();
    let idl_program2 = ToolboxIdlProgram::try_parse_from_value(&json!({
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
            },
        ],
    }))
    .unwrap(); // Assert that both versions are equivalent
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
                    type_flat: ToolboxIdlTypePrimitive::U32.into()
                },
                ToolboxIdlTypeFlatFieldNamed {
                    name: "id".to_string(),
                    docs: None,
                    type_flat: ToolboxIdlTypePrimitive::I64.into()
                },
            ]),
            args_type_full_fields: ToolboxIdlTypeFullFields::Named(vec![
                ToolboxIdlTypeFullFieldNamed {
                    name: "index".to_string(),
                    type_full: ToolboxIdlTypePrimitive::U32.into()
                },
                ToolboxIdlTypeFullFieldNamed {
                    name: "id".to_string(),
                    type_full: ToolboxIdlTypePrimitive::I64.into()
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
                        type_flat: ToolboxIdlTypePrimitive::U64.into()
                    },
                    ToolboxIdlTypeFlatFieldNamed {
                        name: "field2".to_string(),
                        docs: None,
                        type_flat: ToolboxIdlTypePrimitive::U32.into()
                    }
                ])
            },
            content_type_full: ToolboxIdlTypeFull::Struct {
                fields: ToolboxIdlTypeFullFields::Named(vec![
                    ToolboxIdlTypeFullFieldNamed {
                        name: "field1".to_string(),
                        type_full: ToolboxIdlTypePrimitive::U64.into()
                    },
                    ToolboxIdlTypeFullFieldNamed {
                        name: "field2".to_string(),
                        type_full: ToolboxIdlTypePrimitive::U32.into()
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
            code: 4242,
            msg: "My error message".to_string()
        }
        .into()
    )
}
