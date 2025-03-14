use serde_json::json;
use solana_toolbox_idl::ToolboxIdlAccount;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypeFlatFields;
use solana_toolbox_idl::ToolboxIdlTypeFull;
use solana_toolbox_idl::ToolboxIdlTypeFullFields;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl1 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "accounts": [
            {
                "name": "MyAccount",
                "discriminator": [246, 28, 6, 87, 251, 45, 50, 42],
                "type": { "fields": [] }
            },
        ],
    }))
    .unwrap();
    let idl2 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "accounts": [
            {
                "name": "MyAccount",
                "type": { "fields": [] }
            },
        ],
    }))
    .unwrap();
    let idl3 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "accounts": [
            {
                "name": "MyAccount",
                "discriminator": [246, 28, 6, 87, 251, 45, 50, 42],
                "fields": [],
            },
        ],
    }))
    .unwrap();
    let idl4 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "accounts": [
            {
                "name": "MyAccount",
                "fields": [],
            },
        ],
    }))
    .unwrap();
    let idl5 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "accounts": {
            "MyAccount": {
                "discriminator": [246, 28, 6, 87, 251, 45, 50, 42],
                "type": { "fields": [] }
            },
        },
    }))
    .unwrap();
    let idl6 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "accounts": {
            "MyAccount": {
                "type": { "fields": [] }
            },
        },
    }))
    .unwrap();
    let idl7 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "accounts": {
            "MyAccount": {
                "discriminator": [246, 28, 6, 87, 251, 45, 50, 42],
                "fields": []
            },
        },
    }))
    .unwrap();
    let idl8 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "accounts": {
            "MyAccount": { "fields": [] },
        },
    }))
    .unwrap();
    // Assert that all are equivalent
    assert_eq!(idl1, idl2);
    assert_eq!(idl1, idl3);
    assert_eq!(idl1, idl4);
    assert_eq!(idl1, idl5);
    assert_eq!(idl1, idl6);
    assert_eq!(idl1, idl7);
    assert_eq!(idl1, idl8);
    // Assert that the content is correct
    assert_eq!(
        idl1.accounts.get("MyAccount").unwrap(),
        &ToolboxIdlAccount {
            name: "MyAccount".to_string(),
            discriminator: vec![246, 28, 6, 87, 251, 45, 50, 42],
            data_type_flat: ToolboxIdlTypeFlat::Struct {
                fields: ToolboxIdlTypeFlatFields::None
            },
            data_type_full: ToolboxIdlTypeFull::Struct {
                fields: ToolboxIdlTypeFullFields::None
            },
        }
    )
}
