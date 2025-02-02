use serde_json::json;
use solana_toolbox_idl::ToolboxIdl;
use solana_toolbox_idl::ToolboxIdlAccount;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl = ToolboxIdl::try_from_value(&json!({
        "accounts": {
            "MyAccount": {
                "discriminator": [77, 78],
                "fields": [
                    "MyStructNamed",
                    "MyStructUnamed",
                ]
            },
        },
        "types": {
            "MyStructNamed": {
                "fields": [
                    { "name": "field1", "type": "u32"},
                ],
            },
            "MyStructUnamed": {
                "fields": ["u16", "u8"],
            },
        },
    }))
    .unwrap();
    // MyAccount prepared
    let account = ToolboxIdlAccount {
        name: "MyAccount".to_string(),
        state: json!([
            { "field1": 42 },
            [22, 23],
        ]),
    };
    // Check that we can use the manual IDL to compile/decompile our account 1
    let account_data = idl.compile_account(&account).unwrap();
    assert_eq!(vec![77, 78, 42, 0, 0, 0, 22, 0, 23], account_data);
    assert_eq!(account, idl.decompile_account(&account_data).unwrap());
}
