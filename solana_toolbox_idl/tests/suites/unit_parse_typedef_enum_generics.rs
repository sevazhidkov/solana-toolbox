use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypePrefix;
use solana_toolbox_idl::ToolboxIdlTypedef;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl_program1 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "types": [
            {
                "name": "MyEnum",
                "generics": [
                    { "kind": "type", "name": "A" },
                    { "name": "B" },
                ],
                "type": { "variants": [] }
            },
        ],
    }))
    .unwrap();
    let idl_program2 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "types": [
            {
                "name": "MyEnum",
                "generics": [
                    { "kind": "type", "name": "A" },
                    { "name": "B" },
                ],
                "variants": [],
            },
        ],
    }))
    .unwrap();
    let idl_program3 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "types": [
            {
                "name": "MyEnum",
                "generics": ["A", "B"],
                "variants": [],
            },
        ],
    }))
    .unwrap();
    let idl_program4 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "types": {
            "MyEnum": {
                "generics": [
                    { "kind": "type", "name": "A" },
                    { "name": "B" },
                ],
                "type": { "variants": [] }
            },
        },
    }))
    .unwrap();
    let idl_program5 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "types": {
            "MyEnum": {
                "generics": ["A", "B"],
                "variants": []
            },
        },
    }))
    .unwrap();
    // Assert that all are equivalent
    assert_eq!(idl_program1, idl_program2);
    assert_eq!(idl_program1, idl_program3);
    assert_eq!(idl_program1, idl_program4);
    assert_eq!(idl_program1, idl_program5);
    // Assert that the content is correct
    assert_eq!(
        *idl_program1.typedefs.get("MyEnum").unwrap(),
        ToolboxIdlTypedef {
            name: "MyEnum".to_string(),
            docs: None,
            repr: None,
            generics: vec!["A".to_string(), "B".to_string()],
            type_flat: ToolboxIdlTypeFlat::Enum {
                prefix: ToolboxIdlTypePrefix::U8,
                variants: vec![]
            }
        }
        .into()
    )
}
