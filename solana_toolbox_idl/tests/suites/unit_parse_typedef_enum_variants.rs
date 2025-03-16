use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypeFlatFields;
use solana_toolbox_idl::ToolboxIdlTypedef;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl_program1 = ToolboxIdlProgram::try_parse_from_value(&json!({
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
    let idl_program2 = ToolboxIdlProgram::try_parse_from_value(&json!({
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
    let idl_program3 = ToolboxIdlProgram::try_parse_from_value(&json!({
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
    assert_eq!(idl_program1, idl_program2);
    assert_eq!(idl_program1, idl_program3);
    // Assert that the content is correct
    assert_eq!(
        idl_program1.get_idl_typedef("MyEnum").unwrap(),
        ToolboxIdlTypedef {
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
        .into()
    )
}
