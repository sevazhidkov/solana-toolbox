use serde_json::json;
use solana_toolbox_idl::ToolboxIdl;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_shortened = ToolboxIdl::try_from_value(&json!({
        "instructions": {
            "my_instruction": {
                "accounts": [
                    { "name": "payer" },
                    { "name": "authority" },
                ],
                "args": [
                    { "name": "index", "type": "u32" },
                ]
            }
        },
        "accounts": {
            "MyAccount": {
                "kind": "struct",
                "fields": [
                    { "name": "my_field", "type": "u64" }
                ],
            }
        },
        "types": {
            "MyStruct": {
                "kind": "struct",
                "fields": [
                    { "name": "addr", "type": "pubkey" }
                ]
            }
        },
        "errors": {
            "MyError": {
                "code": 4242,
                "msg": "My error message",
            }
        },
    }))
    .unwrap();
    // Lookup instructions and print them
    for lookup_instruction in idl_shortened.lookup_instructions().unwrap() {
        lookup_instruction.print();
    }
    // Lookup accounts and print them
    for lookup_account in idl_shortened.lookup_accounts().unwrap() {
        lookup_account.print();
    }
    // Lookup types and print them
    for lookup_type in idl_shortened.lookup_types().unwrap() {
        lookup_type.print();
    }
    // Lookup errors and print them
    for lookup_error in idl_shortened.lookup_errors().unwrap() {
        lookup_error.print();
    }
    // Create an IDL on the fly
    let idl_standard = ToolboxIdl::try_from_value(&json!({
        "instructions": [
            {
                "name": "my_instruction",
                "accounts": [
                    { "name": "payer" },
                    { "name": "authority" },
                ],
                "args": [
                    { "name": "index", "type": "u32" },
                ]
            }
        ],
        "accounts": [
            {
                "name": "MyAccount",
                "type": {
                    "kind": "struct",
                    "fields": [
                        { "name": "my_field", "type": "u64" }
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
                        { "name": "addr", "type": "pubkey" }
                    ]
                }
            }
        ],
        "errors": [
            {
                "name": "MyError",
                "code": 4242,
                "msg": "My error message",
            }
        ],
    }))
    .unwrap();
    // Lookup instructions and print them
    for lookup_instruction in idl_standard.lookup_instructions().unwrap() {
        lookup_instruction.print();
    }
    // Lookup accounts and print them
    for lookup_account in idl_standard.lookup_accounts().unwrap() {
        lookup_account.print();
    }
    // Lookup types and print them
    for lookup_type in idl_standard.lookup_types().unwrap() {
        lookup_type.print();
    }
    // Lookup errors and print them
    for lookup_error in idl_standard.lookup_errors().unwrap() {
        lookup_error.print();
    }
    // Assert that both versions are equivalent
    assert_eq!(idl_shortened, idl_standard);
    // Assert instruction was parsed correctly
    let my_instruction =
        idl_standard.lookup_instruction("my_instruction").unwrap();
        assert_eq!("my_instruction", my_instruction.name);
        assert_eq!("payer", my_instruction.accounts[0].name);
    assert_eq!("authority", my_instruction.accounts[1].name);
    assert_eq!("index", my_instruction.args[0].name);
    assert_eq!("u32", my_instruction.args[0].description);
    // Assert account was parsed correctly
    let my_account = idl_standard.lookup_account("MyAccount").unwrap();
    assert_eq!("MyAccount", my_account.name);
    assert_eq!("my_field", my_account.fields[0].name);
    assert_eq!("u64", my_account.fields[0].description);
    // Assert struct was parsed correctly
    let my_struct = idl_standard.lookup_type("MyStruct").unwrap();
    assert_eq!("addr", my_struct.items[0].name);
    assert_eq!("pubkey", my_struct.items[0].description);
    // Assert error was parsed correctly
    let my_error = idl_standard.lookup_error_by_code(4242).unwrap();
    assert_eq!("MyError", my_error.name);
    assert_eq!(4242, my_error.code);
    assert_eq!("My error message", my_error.msg);
}
