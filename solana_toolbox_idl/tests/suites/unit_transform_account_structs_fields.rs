use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgramRoot;
use solana_toolbox_idl::ToolboxIdlAccount;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl = ToolboxIdlProgramRoot::try_parse_from_value(&json!({
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
                        "name": "unamed",
                        "type": "MyStructUnamed"
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
            "MyStructUnamed": {
                "fields": ["u16", "u8"],
            },
        },
    }))
    .unwrap();
    // MyAccount prepared
    let account = ToolboxIdlAccount {
        name: "MyAccount".to_string(),
        state: json!({
            "named": {
                "field1": 42,
                "field2": [66, 67],
            },
            "unamed": [22, 23],
        }),
    };
    // Check that we can use the manual IDL to compile/decompile our account 1
    let account_data = idl.compile_account(&account).unwrap();
    assert_eq!(vec![77, 78, 42, 0, 0, 0, 66, 67, 22, 0, 23], account_data);
    assert_eq!(account, idl.decompile_account(&account_data).unwrap());
}
