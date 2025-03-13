use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgramRoot;
use solana_toolbox_idl::ToolboxIdlProgramTypeFlat;
use solana_toolbox_idl::ToolboxIdlProgramTypeFlatFields;
use solana_toolbox_idl::ToolboxIdlProgramTypedef;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl1 = ToolboxIdlProgramRoot::try_parse_from_value(&json!({
        "types": [
            {
                "name": "MyStruct",
                "type": { "fields": [] }
            },
        ],
    }))
    .unwrap();
    let idl2 = ToolboxIdlProgramRoot::try_parse_from_value(&json!({
        "types": [
            {
                "name": "MyStruct",
                "fields": [],
            },
        ],
    }))
    .unwrap();
    let idl3 = ToolboxIdlProgramRoot::try_parse_from_value(&json!({
        "types": {
            "MyStruct": {
                "type": { "fields": [] }
            },
        },
    }))
    .unwrap();
    let idl4 = ToolboxIdlProgramRoot::try_parse_from_value(&json!({
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
        idl1.typedefs.get("MyStruct").unwrap(),
        &ToolboxIdlProgramTypedef {
            name: "MyStruct".to_string(),
            generics: vec![],
            type_flat: ToolboxIdlProgramTypeFlat::Struct {
                fields: ToolboxIdlProgramTypeFlatFields::None
            }
        }
    )
}
