use serde_json::json;
use solana_toolbox_idl::ToolboxIdl;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_standard = ToolboxIdl::try_from_value(&json!({
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
    // Lookup instructions and print them
    for program_instruction in idl_standard.program_instructions.values() {
        program_instruction.print();
    }
    // Lookup accounts and print them
    for program_account in idl_standard.program_accounts.values() {
        program_account.print();
    }
    // Lookup types and print them
    for program_type in idl_standard.program_types.values() {
        program_type.print();
    }
    // Lookup errors and print them
    for program_error in idl_standard.program_errors.values() {
        program_error.print();
    }
    // Create an IDL on the fly
    let idl_shortened = ToolboxIdl::try_from_value(&json!({
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
    // Lookup instructions and print them
    for program_instruction in idl_shortened.program_instructions.values() {
        program_instruction.print();
    }
    // Lookup accounts and print them
    for program_account in idl_shortened.program_accounts.values() {
        program_account.print();
    }
    // Lookup errors and print them
    for program_error in idl_shortened.program_errors.values() {
        program_error.print();
    }
    // Assert that both versions are equivalent
    assert_eq!(idl_shortened, idl_standard);
    // Assert instruction was parsed correctly
    let my_instruction =
        idl_standard.program_instructions.get("my_instruction").unwrap();
    assert_eq!("my_instruction", my_instruction.name);
    assert_eq!("payer", my_instruction.accounts[0].name);
    assert_eq!("authority", my_instruction.accounts[1].name);
    assert_eq!("index", my_instruction.args[0].0);
    assert_eq!("u32", my_instruction.args[0].1.describe());
    assert_eq!("id", my_instruction.args[1].0);
    assert_eq!("i64", my_instruction.args[1].1.describe());
    // Assert account was parsed correctly
    let my_account = idl_standard.program_accounts.get("MyAccount").unwrap();
    assert_eq!("MyAccount", my_account.name);
    assert_eq!("Struct{field1:u64,field2:u32}", my_account.typedef.describe());
    // Assert struct was parsed correctly
    let my_struct = idl_standard.program_types.get("MyStruct").unwrap();
    assert_eq!("MyStruct", my_struct.name);
    assert_eq!("Struct{addr:pubkey,name:string}", my_struct.typedef.describe());
    // Assert error was parsed correctly
    let my_error = idl_standard.program_errors.get(&4242).unwrap();
    assert_eq!(4242, my_error.code);
    assert_eq!("MyError", my_error.name);
    assert_eq!("My error message", my_error.msg);
}
