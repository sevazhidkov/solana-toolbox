use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypeFlatEnumVariant;
use solana_toolbox_idl::ToolboxIdlTypeFlatFields;
use solana_toolbox_idl::ToolboxIdlTypePrefix;
use solana_toolbox_idl::ToolboxIdlTypedef;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl_program1 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "types": {
            "MyEnum": {
                "variants": [
                    { "name": "Case1", "fields": [] },
                    { "name": "Case2", "fields": [], "code": 42 },
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
                    { "name": "Case2", "code": 42 },
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
                    {"name": "Case2", "code": 42},
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
        *idl_program1.typedefs.get("MyEnum").unwrap(),
        ToolboxIdlTypedef {
            name: "MyEnum".to_string(),
            docs: None,
            serialization: None,
            repr: None,
            generics: vec![],
            type_flat: ToolboxIdlTypeFlat::Enum {
                prefix: ToolboxIdlTypePrefix::U8,
                variants: vec![
                    ToolboxIdlTypeFlatEnumVariant {
                        name: "Case1".to_string(),
                        code: 0,
                        docs: None,
                        fields: ToolboxIdlTypeFlatFields::None
                    },
                    ToolboxIdlTypeFlatEnumVariant {
                        name: "Case2".to_string(),
                        code: 42,
                        docs: None,
                        fields: ToolboxIdlTypeFlatFields::None
                    },
                    ToolboxIdlTypeFlatEnumVariant {
                        name: "Case3".to_string(),
                        code: 2,
                        docs: None,
                        fields: ToolboxIdlTypeFlatFields::None
                    },
                ]
            }
        }
        .into()
    )
}
