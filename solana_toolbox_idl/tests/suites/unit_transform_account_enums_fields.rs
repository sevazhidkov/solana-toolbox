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
                        "name": "Unamed",
                        "fields": ["u8", "u8"],
                    },
                    {
                        "name": "Empty",
                    }
                ],
            },
        },
    }))
    .unwrap();
    // MyAccount prepared
    let account = ToolboxIdlAccount {
        name: "MyAccount".to_string(),
        state: json!([
            "Empty",
            ["Named", {"field1": 42}],
            ["Unamed", [22, 23]],
        ]),
    };
    // Check that we can use the manual IDL to compile/decompile our account 1
    let account_data = idl.compile_account(&account).unwrap();
    assert_eq!(vec![77, 78, 2, 0, 42, 0, 0, 0, 1, 22, 23], account_data);
    assert_eq!(account, idl.decompile_account(&account_data).unwrap());
}
