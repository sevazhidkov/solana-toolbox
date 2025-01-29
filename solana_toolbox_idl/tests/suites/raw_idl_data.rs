use serde_json::json;
use solana_toolbox_idl::ToolboxIdl;
use solana_toolbox_idl::ToolboxIdlAccount;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl = ToolboxIdl::try_from_value(&json!({
        "instructions": {},
        "types": {
            "MyAccount1": {
                "fields": [
                    { "name": "name", "type": "string" },
                    { "name": "my_struct", "type": "MyStruct" },
                    { "name": "array", "type": ["u16", 3] },
                    { "name": "vec", "type": ["i16"] },
                ]
            },
            "MyStruct": {
                "fields": [
                    { "name": "integer", "type": "u32" },
                    { "name": "my_enum", "type": { "defined": "MyEnum" } },
                    { "name": "byte", "type": "u8" },
                ]
            },
            "MyEnum": {
                "variants": ["Hello0", "Hello1", "Hello2"],
            },
        },
        "accounts": {
            "MyAccount1": {
                "discriminator": [74, 73, 72, 71],
            },
            "MyAccount2": {
                "fields": [
                    { "name": "val1", "type": "MyStruct" },
                    { "name": "val2", "type": { "defined": "MyStruct" } },
                ]
            },
        },
        "errors": {},
    }))
    .unwrap();
    // MyAccount1 prepared
    let account = ToolboxIdlAccount {
        name: "MyAccount1".to_string(),
        value: json!({
            "name": "ABCD",
            "my_struct": {
                "integer": 42,
                "my_enum": "Hello1",
                "byte": 77,
            },
            "array": [99, 98, 97],
            "vec": [-55, 56, 57],
        }),
    };
    // Check that we can use the manual IDL to compile/decompile our account 1
    let account_data = idl.compile_account(&account).unwrap();
    assert_eq!(
        vec![
            74, 73, 72, 71, 4, 0, 0, 0, 65, 66, 67, 68, 42, 0, 0, 0, 1, 77, 99,
            00, 98, 00, 97, 00, 3, 0, 0, 0, 201, 255, 56, 0, 57, 0,
        ],
        account_data,
    );
    assert_eq!(account, idl.decompile_account(&account_data).unwrap());
    // MyAccount2 prepared
    let account = ToolboxIdlAccount {
        name: "MyAccount2".to_string(),
        value: json!({
            "val1": {
                "integer": 43,
                "my_enum": "Hello0",
                "byte": 78
            },
            "val2": {
                "integer": 44,
                "my_enum": "Hello2",
                "byte": 79
            },
        }),
    };
    // Check that we can use the manual IDL to compile/decompile our account 2
    assert_eq!(
        account,
        idl.decompile_account(&idl.compile_account(&account).unwrap()).unwrap()
    );
}
