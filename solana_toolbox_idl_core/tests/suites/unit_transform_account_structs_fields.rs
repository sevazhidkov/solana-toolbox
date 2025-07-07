use serde_json::json;
use solana_toolbox_idl_core::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse(&json!({
        "accounts": {
            "MyAccount": {
                "discriminator": [77, 78],
                "fields": [
                    {
                        "name": "named",
                        "type": {
                            "defined": {
                                "name": "MyStructNamed",
                                "generics": ["u8"],
                            }
                        },
                    },
                    {
                        "name": "unnamed",
                        "type": "MyStructUnnamed"
                    },
                ]
            },
        },
        "types": {
            "MyStructNamed": {
                "generics": ["T"],
                "fields": [
                    { "name": "field1", "type": "u32"},
                    { "name": "field2", "type": [{"generic": "T"}, 2]},
                ],
            },
            "MyStructUnnamed": {
                "fields": ["u16", "u8"],
            },
        },
    }))
    .unwrap();
    // MyAccount prepared
    let idl_account = idl_program.accounts.get("MyAccount").unwrap();
    let account_state = json!({
        "named": {
            "field1": 42,
            "field2": [66, 67],
        },
        "unnamed": [22, 23],
    });
    // Check that we can use the manual IDL to encode/decode our account
    let account_data = idl_account.encode(&account_state).unwrap();
    assert_eq!(vec![77, 78, 42, 0, 0, 0, 66, 67, 22, 0, 23], account_data);
    assert_eq!(account_state, idl_account.decode(&account_data).unwrap());
}
