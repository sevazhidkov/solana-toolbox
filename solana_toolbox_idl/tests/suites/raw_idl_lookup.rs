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
                    { "name": "authority" },
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
                        { "name": "my_field1", "type": "u64" },
                        { "name": "my_field2", "type": "u32" },
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
                "name": "MyError",
                "code": 4242,
                "msg": "My error message",
            }
        ],
    }))
    .unwrap();
    // Create an IDL on the fly
    let idl_shortened = ToolboxIdl::try_from_value(&json!({
        "instructions": {
            "my_instruction": {
                "accounts": [
                    { "name": "payer", "isSigner": true },
                    { "name": "authority" },
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
                    { "name": "my_field1", "type": "u64" },
                    { "name": "my_field2", "type": "u32" },
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
            }
        },
    }))
    .unwrap();
    eprintln!("idl_shortened:{:#?}", idl_shortened);
    eprintln!("idl_standard:{:#?}", idl_standard);
    // Assert that both versions are equivalent
    assert_eq!(idl_shortened, idl_standard);
    /* // TODO - re-establish something like that
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
    for program_error in idl_shortened.program_errors.values() {
        program_error.print();
    }*/
    /* // TODO - re-establish something like that
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
    for program_error in idl_standard.program_errors.values() {
        program_error.print();
    }*/
    /* // TODO - re-establish something like that
    // Assert instruction was parsed correctly
    let my_instruction =
        idl_standard.lookup_instruction("my_instruction").unwrap();
    assert_eq!("my_instruction", my_instruction.name);
    assert_eq!("payer", my_instruction.accounts[0].name);
    assert_eq!("authority", my_instruction.accounts[1].name);
    assert_eq!("index", my_instruction.args[0].name);
    assert_eq!("u32", my_instruction.args[0].kind.describe());
    assert_eq!("id", my_instruction.args[1].name);
    assert_eq!("i64", my_instruction.args[1].kind.describe());
    // Assert account was parsed correctly
    let my_account = idl_standard.lookup_account("MyAccount").unwrap();
    assert_eq!("MyAccount", my_account.name);
    assert_eq!("my_field1", my_account.fields[0].name);
    assert_eq!("u64", my_account.fields[0].kind.describe());
    assert_eq!("my_field2", my_account.fields[1].name);
    assert_eq!("u32", my_account.fields[1].kind.describe());
    // Assert struct was parsed correctly
    let my_struct = idl_standard.lookup_type("MyStruct").unwrap();
    assert_eq!("MyStruct", my_struct.name);
    assert_eq!("Struct()", my_struct.kind.describe());
    // Assert error was parsed correctly
    let my_error = idl_standard.program_errors.get(&4242).unwrap();
    assert_eq!("MyError", my_error.name);
    assert_eq!(4242, my_error.code);
    assert_eq!("My error message", my_error.msg);
    */
}
