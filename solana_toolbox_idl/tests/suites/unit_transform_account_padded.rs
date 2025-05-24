use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program1 = ToolboxIdlProgram::try_parse(&json!({
        "accounts": {
            "MyAccount": {
                "discriminator": [22],
                "fields": [
                    { "name": "padded_before", "padded": { "before": 3, "type": "u8" }},
                    { "name": "padded_size1", "padded": { "min_size": 3, "type": ["u8", 2] }},
                    { "name": "padded_size2", "padded": { "min_size": 3, "type": ["u8", 4] }},
                    { "name": "padded_after", "padded": { "after": 3, "type": "u8" }},
                ]
            }
        },
    }))
    .unwrap();
    let idl_program2 = ToolboxIdlProgram::try_parse(&json!({
        "accounts": {
            "MyAccount": {
                "discriminator": [22],
                "fields": [
                    { "name": "padded_before", "padded": { "before": 3, "type": "u8" }},
                    { "name": "padded_size1", "padded": { "min_size": 3, "array": ["u8", 2] }},
                    { "name": "padded_size2", "padded": { "min_size": 3, "array": ["u8", 4] }},
                    { "name": "padded_after", "padded": { "after": 3, "type": "u8" }},
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
        "padded_before": 40,
        "padded_size1": [50, 51],
        "padded_size2": [60, 61, 62, 63],
        "padded_after": 70,
    });
    // Check that we can use the manual IDL to encode/decode our account
    let account_data = idl_account.encode(&account_state).unwrap();
    assert_eq!(
        vec![22, 0, 0, 0, 40, 50, 51, 0, 60, 61, 62, 63, 70, 0, 0, 0],
        account_data,
    );
    assert_eq!(account_state, idl_account.decode(&account_data).unwrap());
}
