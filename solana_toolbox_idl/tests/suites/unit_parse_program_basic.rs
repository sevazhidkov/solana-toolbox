use serde_json::json;
use solana_toolbox_idl::ToolboxIdl;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_standard = ToolboxIdl::try_parse_from_value(&json!({
        "instructions": [
            {
                "name": "my_instruction",
                "accounts": [
                    { "name": "payer", "isSigner": true },
                    { "name": "authority", "signer": true },
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
    let idl_shortened = ToolboxIdl::try_parse_from_value(&json!({
        "instructions": {
            "my_instruction": {
                "accounts": [
                    { "name": "payer", "signer": true },
                    { "name": "authority", "signer": true },
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
    assert_eq!(idl_shortened, idl_standard);
    // Assert instruction was parsed correctly
    let my_instruction = idl_standard
        .program_instructions
        .get("my_instruction")
        .unwrap();
    assert_eq!("my_instruction", my_instruction.name);
    assert_eq!("payer", my_instruction.accounts[0].name);
    assert_eq!("authority", my_instruction.accounts[1].name);
    assert_eq!(
        "Struct{index:u32,id:i64}",
        my_instruction.args_type_flat.describe()
    );
    // Assert account was parsed correctly
    let my_account = idl_standard.program_accounts.get("MyAccount").unwrap();
    assert_eq!("MyAccount", my_account.name);
    assert_eq!(
        "Struct{field1:u64,field2:u32}",
        my_account.data_type_flat.describe()
    );
    // Assert struct was parsed correctly
    let my_struct = idl_standard.program_typedefs.get("MyStruct").unwrap();
    assert_eq!("MyStruct", my_struct.name);
    assert_eq!(
        "Struct{addr:pubkey,name:string}",
        my_struct.type_flat.describe()
    );
    // Assert error was parsed correctly
    let my_error = idl_standard.program_errors.get("MyError").unwrap();
    assert_eq!(4242, my_error.code);
    assert_eq!("MyError", my_error.name);
    assert_eq!("My error message", my_error.msg);
}
