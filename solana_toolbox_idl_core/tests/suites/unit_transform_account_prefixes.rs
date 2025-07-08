use serde_json::json;
use solana_toolbox_idl_core::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl_program1 = ToolboxIdlProgram::try_parse(&json!({
        "accounts": {
            "MyAccount": {
                "discriminator": [77],
                "fields": [
                    { "name": "option", "option": "u8" },
                    { "name": "option8", "option8": "u8" },
                    { "name": "option16", "option16": "u8" },
                    { "name": "option32", "option32": "u8" },
                    { "name": "vec", "vec": "u8" },
                    { "name": "vec8", "vec8": "u8" },
                    { "name": "vec16", "vec16": "u8" },
                    { "name": "vec32", "vec32": "u8" },
                    { "name": "variants", "variants": ["A", "B", "C", "D"] },
                    { "name": "variants8", "variants8": ["A", "B", "C", "D"] },
                    { "name": "variants16", "variants16": ["A", "B", "C", "D"] },
                    { "name": "variants32", "variants32": ["A", "B", "C", "D"] },
                ]
            }
        },
    }))
    .unwrap();
    let idl_program2 = ToolboxIdlProgram::try_parse(&json!({
        "accounts": {
            "MyAccount": {
                "discriminator": [77],
                "fields": [
                    { "name": "option", "type": {"option": "u8"} },
                    { "name": "option8", "type": {"option8": "u8"} },
                    { "name": "option16", "type": {"option16": "u8"} },
                    { "name": "option32", "type": {"option32": "u8"} },
                    { "name": "vec", "type": {"vec": "u8"} },
                    { "name": "vec8", "type": {"vec8": "u8"} },
                    { "name": "vec16", "type": {"vec16": "u8"} },
                    { "name": "vec32", "type": {"vec32": "u8"} },
                    { "name": "variants", "type": {"variants": ["A", "B", "C", "D"]} },
                    { "name": "variants8", "type": {"variants8": ["A", "B", "C", "D"]} },
                    { "name": "variants16", "type": {"variants16": ["A", "B", "C", "D"]} },
                    { "name": "variants32", "type": {"variants32": ["A", "B", "C", "D"]} },
                ]
            }
        },
    }))
    .unwrap();
    // Assert that all are equivalent
    assert_eq!(idl_program1, idl_program2);
    // Choose the account
    let idl_account = idl_program1.accounts.get("MyAccount").unwrap();
    // Dummy state we'll encode/decode
    let account_state = json!({
        "option": 40,
        "option8": 41,
        "option16": 42,
        "option32": 43,
        "vec": [50],
        "vec8": [51],
        "vec16": [52],
        "vec32": [53],
        "variants": "A",
        "variants8": "B",
        "variants16": "C",
        "variants32": "D",
    });
    // Check that we can use the manual IDL to encode/decode our account
    let account_data = idl_account.encode(&account_state).unwrap();
    assert_eq!(
        account_data,
        vec![
            77, 1, 40, 1, 41, 1, 0, 42, 1, 0, 0, 0, 43, 1, 0, 0, 0, 50, 1, 51,
            1, 0, 52, 1, 0, 0, 0, 53, 0, 1, 2, 0, 3, 0, 0, 0
        ],
    );
    assert_eq!(idl_account.decode(&account_data).unwrap(), account_state);
}
