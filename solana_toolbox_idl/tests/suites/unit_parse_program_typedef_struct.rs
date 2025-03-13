use serde_json::json;
use solana_toolbox_idl::ToolboxIdl;
use solana_toolbox_idl::ToolboxIdlProgramTypeFlat;
use solana_toolbox_idl::ToolboxIdlProgramTypeFlatFields;
use solana_toolbox_idl::ToolboxIdlProgramTypedef;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl1 = ToolboxIdl::try_parse_from_value(&json!({
        "types": [
            {
                "name": "MyStruct",
                "type": { "fields": [] }
            },
        ],
    }))
    .unwrap();
    let idl2 = ToolboxIdl::try_parse_from_value(&json!({
        "types": [
            {
                "name": "MyStruct",
                "fields": [],
            },
        ],
    }))
    .unwrap();
    let idl3 = ToolboxIdl::try_parse_from_value(&json!({
        "types": {
            "MyStruct": {
                "type": { "fields": [] }
            },
        },
    }))
    .unwrap();
    let idl4 = ToolboxIdl::try_parse_from_value(&json!({
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
        idl1.program_typedefs.get("MyStruct").unwrap(),
        &ToolboxIdlProgramTypedef {
            name: "MyStruct".to_string(),
            generics: vec![],
            type_flat: ToolboxIdlProgramTypeFlat::Struct {
                fields: ToolboxIdlProgramTypeFlatFields::None
            }
        }
    )
}
