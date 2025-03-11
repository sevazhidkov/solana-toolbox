use serde_json::json;
use solana_toolbox_idl::ToolboxIdl;
use solana_toolbox_idl::ToolboxIdlProgramAccount;
use solana_toolbox_idl::ToolboxIdlProgramTypeFlat;
use solana_toolbox_idl::ToolboxIdlProgramTypeFull;
use solana_toolbox_idl::ToolboxIdlProgramTypeFullFields;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl1 = ToolboxIdl::try_parse_from_value(&json!({
        "accounts": [
            {
                "name": "MyAccount",
                "discriminator": [246, 28, 6, 87, 251, 45, 50, 42],
            },
        ],
        "types": {
            "MyAccount": { "fields": [] },
        },
    }))
    .unwrap();
    let idl2 = ToolboxIdl::try_parse_from_value(&json!({
        "accounts": [
            {
                "name": "MyAccount",
            },
        ],
        "types": {
            "MyAccount": { "fields": [] },
        },
    }))
    .unwrap();
    let idl3 = ToolboxIdl::try_parse_from_value(&json!({
        "accounts": {
            "MyAccount": {
                "discriminator": [246, 28, 6, 87, 251, 45, 50, 42],
            },
        },
        "types": {
            "MyAccount": { "fields": [] },
        },
    }))
    .unwrap();
    let idl4 = ToolboxIdl::try_parse_from_value(&json!({
        "accounts": {
            "MyAccount": {},
        },
        "types": {
            "MyAccount": { "fields": [] },
        },
    }))
    .unwrap();
    // Assert that all are equivalent
    assert_eq!(idl1, idl2);
    assert_eq!(idl1, idl3);
    assert_eq!(idl1, idl4);
    // Assert that the content is correct
    assert_eq!(
        idl1.program_accounts.get("MyAccount").unwrap(),
        &ToolboxIdlProgramAccount {
            name: "MyAccount".to_string(),
            discriminator: vec![246, 28, 6, 87, 251, 45, 50, 42],
            data_type_flat: ToolboxIdlProgramTypeFlat::Defined {
                name: "MyAccount".to_string(),
                generics: vec![]
            },
            data_type_full: ToolboxIdlProgramTypeFull::Struct {
                fields: ToolboxIdlProgramTypeFullFields::None
            },
        }
    )
}
