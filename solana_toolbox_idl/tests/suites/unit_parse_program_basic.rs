use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypeFlatFields;
use solana_toolbox_idl::ToolboxIdlTypePrimitive;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program1 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "instructions": [
            {
                "name": "my_ix",
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
                "type": {
                    "kind": "struct",
                    "fields": [
                        { "name": "field1", "type": "u64" },
                        { "name": "field2", "type": "u32" },
                    ],
                }
            }
        ],
        "types": [
            {
                "name": "MyStruct",
                "type": {
                    "kind": "struct",
                    "fields": [
                        { "name": "addr", "type": "pubkey" },
                        { "name": "name", "type": "string" },
                    ]
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
                "fields": [
                    { "name": "field1", "type": "u64" },
                    { "name": "field2", "type": "u32" },
                ],
            }
        },
        "types": {
            "MyStruct": {
                "fields": [
                    { "name": "addr", "type": "pubkey" },
                    { "name": "name", "type": "string" },
                ]
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
    let idl_account = idl_program1.accounts.get("MyAccount").unwrap();
    assert_eq!(idl_account.name, "MyAccount");
    assert_eq!(
        idl_account.discriminator,
        &[246, 28, 6, 87, 251, 45, 50, 42]
    );
    assert_eq!(
        idl_account.content_type_flat,
        ToolboxIdlTypeFlat::Struct {
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
        }
    );
    // Assert struct was parsed correctly
    let idl_typedef = idl_program1.typedefs.get("MyStruct").unwrap();
    assert_eq!(idl_typedef.name, "MyStruct");
    assert_eq!(
        idl_typedef.type_flat,
        ToolboxIdlTypeFlat::Struct {
            fields: ToolboxIdlTypeFlatFields::Named(vec![
                (
                    "addr".to_string(),
                    ToolboxIdlTypeFlat::Primitive {
                        primitive: ToolboxIdlTypePrimitive::PublicKey
                    }
                ),
                (
                    "name".to_string(),
                    ToolboxIdlTypeFlat::Primitive {
                        primitive: ToolboxIdlTypePrimitive::String
                    }
                )
            ])
        }
    );
    // Assert error was parsed correctly
    let idl_error = idl_program1.errors.get("MyError").unwrap();
    assert_eq!(idl_error.name, "MyError");
    assert_eq!(idl_error.code, 4242);
    assert_eq!(idl_error.msg, "My error message");
}
