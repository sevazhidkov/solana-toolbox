use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgram;

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
            {
                "code": 4243,
                "name": "MyError2",
                "msg": "",
            }
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
            "MyError2": 4243,
        },
    }))
    .unwrap();
    // Assert that both versions are equivalent
    assert_eq!(idl_program2, idl_program1);
    // Assert instruction was parsed correctly
    let idl_instruction = idl_program1.get_idl_instruction("my_ix").unwrap();
    assert_eq!("my_ix", idl_instruction.name);
    assert_eq!("payer", idl_instruction.accounts[0].name);
    assert_eq!("authority", idl_instruction.accounts[1].name);
    assert_eq!(
        json!({}), // TODO - proper check
        idl_instruction.args_type_flat_fields.as_json(false)
    );
    // Assert account was parsed correctly
    let idl_account = idl_program1.get_idl_account("MyAccount").unwrap();
    assert_eq!("MyAccount", idl_account.name);
    assert_eq!(
        json!({}), // TODO - proper check
        idl_account.content_type_flat.as_json(false)
    );
    // Assert struct was parsed correctly
    let idl_typedef = idl_program1.get_idl_typedef("MyStruct").unwrap();
    assert_eq!("MyStruct", idl_typedef.name);
    assert_eq!(
        json!({}), // TODO - proper check
        idl_typedef.type_flat.as_json(false)
    );
    // Assert error was parsed correctly
    let idl_error = idl_program1.get_idl_error("MyError").unwrap();
    assert_eq!(4242, idl_error.code);
    assert_eq!("MyError", idl_error.name);
    assert_eq!("My error message", idl_error.msg);
}
