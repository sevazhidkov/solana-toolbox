use serde_json::json;
use solana_toolbox_idl::ToolboxIdl;
use solana_toolbox_idl::ToolboxIdlProgramType;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypeFlatFields;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl1 = ToolboxIdl::try_from_value(&json!({
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
    let idl2 = ToolboxIdl::try_from_value(&json!({
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
    let idl3 = ToolboxIdl::try_from_value(&json!({
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
        idl1.program_types.get("MyEnum").unwrap(),
        &ToolboxIdlProgramType {
            name: "MyEnum".to_string(),
            generics: vec![],
            type_flat: ToolboxIdlTypeFlat::Enum {
                variants: vec![
                    ("Case1".to_string(), ToolboxIdlTypeFlatFields::None),
                    ("Case2".to_string(), ToolboxIdlTypeFlatFields::None),
                    ("Case3".to_string(), ToolboxIdlTypeFlatFields::None),
                ]
            }
        }
    )
}
