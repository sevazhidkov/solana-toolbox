use serde_json::json;
use solana_toolbox_idl_core::ToolboxIdlAccount;
use solana_toolbox_idl_core::ToolboxIdlProgram;
use solana_toolbox_idl_core::ToolboxIdlTypeFlat;
use solana_toolbox_idl_core::ToolboxIdlTypeFull;
use solana_toolbox_idl_core::ToolboxIdlTypeFullFields;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl_program1 = ToolboxIdlProgram::try_parse(&json!({
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
    let idl_program2 = ToolboxIdlProgram::try_parse(&json!({
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
    let idl_program3 = ToolboxIdlProgram::try_parse(&json!({
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
    let idl_program4 = ToolboxIdlProgram::try_parse(&json!({
        "accounts": {
            "MyAccount": {},
        },
        "types": {
            "MyAccount": { "fields": [] },
        },
    }))
    .unwrap();
    // Assert that all are equivalent
    assert_eq!(idl_program1, idl_program2);
    assert_eq!(idl_program1, idl_program3);
    assert_eq!(idl_program1, idl_program4);
    // Assert that the content is correct
    assert_eq!(
        *idl_program1.accounts.get("MyAccount").unwrap(),
        ToolboxIdlAccount {
            name: "MyAccount".to_string(),
            docs: None,
            space: None,
            blobs: vec![],
            discriminator: vec![246, 28, 6, 87, 251, 45, 50, 42],
            content_type_flat: ToolboxIdlTypeFlat::Defined {
                name: "MyAccount".to_string(),
                generics: vec![]
            },
            content_type_full: ToolboxIdlTypeFull::Typedef {
                name: "MyAccount".to_string(),
                repr: None,
                content: Box::new(ToolboxIdlTypeFull::Struct {
                    fields: ToolboxIdlTypeFullFields::nothing()
                })
            },
        }
        .into()
    )
}
