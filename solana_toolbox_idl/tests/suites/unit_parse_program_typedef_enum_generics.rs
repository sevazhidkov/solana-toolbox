use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypedef;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl1 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "types": [
            {
                "name": "MyEnum",
                "generics": [
                    { "name": "A" },
                    { "name": "B" },
                ],
                "type": { "variants": [] }
            },
        ],
    }))
    .unwrap();
    let idl2 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "types": [
            {
                "name": "MyEnum",
                "generics": [
                    { "name": "A" },
                    { "name": "B" },
                ],
                "variants": [],
            },
        ],
    }))
    .unwrap();
    let idl3 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "types": [
            {
                "name": "MyEnum",
                "generics": ["A", "B"],
                "variants": [],
            },
        ],
    }))
    .unwrap();
    let idl4 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "types": {
            "MyEnum": {
                "generics": [
                    { "name": "A" },
                    { "name": "B" },
                ],
                "type": { "variants": [] }
            },
        },
    }))
    .unwrap();
    let idl5 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "types": {
            "MyEnum": {
                "generics": ["A", "B"],
                "variants": []
            },
        },
    }))
    .unwrap();
    // Assert that all are equivalent
    assert_eq!(idl1, idl2);
    assert_eq!(idl1, idl3);
    assert_eq!(idl1, idl4);
    assert_eq!(idl1, idl5);
    // Assert that the content is correct
    assert_eq!(
        idl1.typedefs.get("MyEnum").unwrap(),
        &ToolboxIdlTypedef {
            name: "MyEnum".to_string(),
            generics: vec!["A".to_string(), "B".to_string()],
            type_flat: ToolboxIdlTypeFlat::Enum { variants: vec![] }
        }
    )
}
