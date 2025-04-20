use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse_from_value(&json!({
        "accounts": {
            "MyAccount": {
                "discriminator": [77, 78],
                "fields": [
                    "MyEnum",
                    "MyEnum",
                    "MyEnum",
                    "MyEnum",
                ]
            },
        },
        "types": {
            "MyEnum": {
                "variants": [
                    {
                        "name": "Named",
                        "fields": [
                            { "name": "field1", "type": "u32"},
                        ]
                    },
                    {
                        "name": "Unnamed",
                        "code": 99,
                        "fields": ["u8", "u8"],
                    },
                    {
                        "name": "Empty",
                    },
                    "Shortened",
                ],
            },
        },
    }))
    .unwrap();
    // MyAccount info
    let idl_account = idl_program.accounts.get("MyAccount").unwrap();
    let account_state = json!([
        "Empty",
        {"Named": {"field1": 42}},
        {"Unnamed": [22, 23]},
        "Shortened",
    ]);
    // Check that we can use the manual IDL to encode/decode our account
    let account_data = idl_account.encode(&account_state).unwrap();
    assert_eq!(vec![77, 78, 2, 0, 42, 0, 0, 0, 99, 22, 23, 3], account_data);
    assert_eq!(account_state, idl_account.decode(&account_data).unwrap());
}
