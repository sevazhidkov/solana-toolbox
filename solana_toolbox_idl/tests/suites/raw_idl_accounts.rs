use serde_json::json;
use solana_toolbox_idl::ToolboxIdl;
use solana_toolbox_idl::ToolboxIdlAccount;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl = ToolboxIdl::try_from_value(&json!({
        "instructions": {},
        "types": {
            "MyArg": {
                "kind": "struct",
                "fields": [
                    { "name": "info", "type": "u32" },
                    { "name": "postfix", "type": "u8" },
                ]
            },
            "MyAccount1": {
                "kind": "struct",
                "fields": [
                    { "name": "prefix", "type": "string" },
                    { "name": "val", "type": {"defined": "MyArg"} }
                ]
            },
        },
        "accounts": {
            "MyAccount1": {
                "discriminator": [4, 3, 2, 1],
            },
            "MyAccount2": {
                "kind": "struct",
                "fields": [
                    { "name": "val1", "type": {"defined": "MyArg"} },
                    { "name": "val2", "type": {"defined": "MyArg"} },
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
            "prefix": "ABCD",
            "val": { "info": 42, "postfix": 77 },
        }),
    };
    // Check that we can use the manual IDL to compile/decompile our account 1
    let account_data = idl.compile_account(&account).unwrap();
    assert_eq!(
        vec![4, 3, 2, 1, 4, 0, 0, 0, 65, 66, 67, 68, 42, 0, 0, 0, 77],
        account_data,
    );
    assert_eq!(account, idl.decompile_account(&account_data).unwrap());
    // MyAccount2 prepared
    let account = ToolboxIdlAccount {
        name: "MyAccount2".to_string(),
        value: json!({
            "val1": { "info": 43, "postfix": 78 },
            "val2": { "info": 44, "postfix": 79 },
        }),
    };
    // Check that we can use the manual IDL to compile/decompile our account 2
    assert_eq!(
        account,
        idl.decompile_account(&idl.compile_account(&account).unwrap()).unwrap()
    );
}
