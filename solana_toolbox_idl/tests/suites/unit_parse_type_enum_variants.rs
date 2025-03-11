use serde_json::json;
use solana_toolbox_idl::ToolboxIdl;
use solana_toolbox_idl::ToolboxIdlProgramTypedef;
use solana_toolbox_idl::ToolboxIdlProgramTypeFlat;
use solana_toolbox_idl::ToolboxIdlProgramTypeFlatFields;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl1 = ToolboxIdl::try_parse_from_value(&json!({
        "types": {
            "MyEnum": {
                "variants": [
                    { "name": "Case1", "fields": [] },
                    { "name": "Case2", "fields": [] },
                    { "name": "Case3", "fields": [] },
                ]
            },
        },
    }))
    .unwrap();
    let idl2 = ToolboxIdl::try_parse_from_value(&json!({
        "types": {
            "MyEnum": {
                "variants": [
                    { "name": "Case1" },
                    { "name": "Case2" },
                    { "name": "Case3" },
                ]
            },
        },
    }))
    .unwrap();
    let idl3 = ToolboxIdl::try_parse_from_value(&json!({
        "types": {
            "MyEnum": {
                "variants": [
                    "Case1",
                    "Case2",
                    "Case3",
                ]
            },
        },
    }))
    .unwrap();
    // Assert that all are equivalent
    assert_eq!(idl1, idl2);
    assert_eq!(idl1, idl3);
    // Assert that the content is correct
    assert_eq!(
        idl1.program_typedefs.get("MyEnum").unwrap(),
        &ToolboxIdlProgramTypedef {
            name: "MyEnum".to_string(),
            generics: vec![],
            type_flat: ToolboxIdlProgramTypeFlat::Enum {
                variants: vec![
                    ("Case1".to_string(), ToolboxIdlProgramTypeFlatFields::None),
                    ("Case2".to_string(), ToolboxIdlProgramTypeFlatFields::None),
                    ("Case3".to_string(), ToolboxIdlProgramTypeFlatFields::None),
                ]
            }
        }
    )
}
