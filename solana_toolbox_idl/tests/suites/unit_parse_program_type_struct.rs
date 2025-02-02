use serde_json::json;
use solana_toolbox_idl::ToolboxIdl;
use solana_toolbox_idl::ToolboxIdlProgramType;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypeFlatFields;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl1 = ToolboxIdl::try_from_value(&json!({
        "types": [
            {
                "name": "MyStruct",
                "type": { "fields": [] }
            },
        ],
    }))
    .unwrap();
    let idl2 = ToolboxIdl::try_from_value(&json!({
        "types": [
            {
                "name": "MyStruct",
                "fields": [],
            },
        ],
    }))
    .unwrap();
    let idl3 = ToolboxIdl::try_from_value(&json!({
        "types": {
            "MyStruct": {
                "type": { "fields": [] }
            },
        },
    }))
    .unwrap();
    let idl4 = ToolboxIdl::try_from_value(&json!({
        "types": {
            "MyStruct": { "fields": [] },
        },
    }))
    .unwrap();
    // Assert that all are equivalent
    assert_eq!(idl1, idl2);
    assert_eq!(idl1, idl3);
    assert_eq!(idl1, idl4);
    // Assert that the content is correct
    assert_eq!(
        idl1.program_types.get("MyStruct").unwrap(),
        &ToolboxIdlProgramType {
            name: "MyStruct".to_string(),
            generics: vec![],
            type_flat: ToolboxIdlTypeFlat::Struct {
                fields: ToolboxIdlTypeFlatFields::None
            }
        }
    )
}
