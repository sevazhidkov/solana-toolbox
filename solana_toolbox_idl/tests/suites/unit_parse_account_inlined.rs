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
    let idl_program1 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "accounts": [
            {
                "name": "MyAccount",
                "discriminator": [246, 28, 6, 87, 251, 45, 50, 42],
                "type": { "fields": [] }
            },
        ],
    }))
    .unwrap();
    let idl_program2 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "accounts": [
            {
                "name": "MyAccount",
                "type": { "fields": [] }
            },
        ],
    }))
    .unwrap();
    let idl_program3 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "accounts": [
            {
                "name": "MyAccount",
                "discriminator": [246, 28, 6, 87, 251, 45, 50, 42],
                "fields": [],
            },
        ],
    }))
    .unwrap();
    let idl_program4 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "accounts": [
            {
                "name": "MyAccount",
                "fields": [],
            },
        ],
    }))
    .unwrap();
    let idl_program5 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "accounts": {
            "MyAccount": {
                "discriminator": [246, 28, 6, 87, 251, 45, 50, 42],
                "type": { "fields": [] }
            },
        },
    }))
    .unwrap();
    let idl_program6 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "accounts": {
            "MyAccount": {
                "type": { "fields": [] }
            },
        },
    }))
    .unwrap();
    let idl_program7 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "accounts": {
            "MyAccount": {
                "discriminator": [246, 28, 6, 87, 251, 45, 50, 42],
                "fields": []
            },
        },
    }))
    .unwrap();
    let idl_program8 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "accounts": {
            "MyAccount": { "fields": [] },
        },
    }))
    .unwrap();
    // Assert that all are equivalent
    assert_eq!(idl_program1, idl_program2);
    assert_eq!(idl_program1, idl_program3);
    assert_eq!(idl_program1, idl_program4);
    assert_eq!(idl_program1, idl_program5);
    assert_eq!(idl_program1, idl_program6);
    assert_eq!(idl_program1, idl_program7);
    assert_eq!(idl_program1, idl_program8);
    // Assert that the content is correct
    assert_eq!(
        *idl_program1.accounts.get("MyAccount").unwrap(),
        ToolboxIdlAccount {
            name: "MyAccount".to_string(),
            docs: None,
            space: None,
            discriminator: vec![246, 28, 6, 87, 251, 45, 50, 42],
            content_type_flat: ToolboxIdlTypeFlat::Struct {
                fields: ToolboxIdlTypeFlatFields::None
            },
            content_type_full: ToolboxIdlTypeFull::Struct {
                fields: ToolboxIdlTypeFullFields::None
            },
        }
        .into()
    )
}
