use serde_json::json;
use solana_toolbox_idl_core::ToolboxIdlProgram;
use solana_toolbox_idl_core::ToolboxIdlTypeFlat;
use solana_toolbox_idl_core::ToolboxIdlTypeFlatEnumVariant;
use solana_toolbox_idl_core::ToolboxIdlTypeFlatFields;
use solana_toolbox_idl_core::ToolboxIdlTypePrefix;
use solana_toolbox_idl_core::ToolboxIdlTypedef;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl_program1 = ToolboxIdlProgram::try_parse(&json!({
        "types": {
            "MyEnum": {
                "variants": [
                    { "name": "77", "fields": [], "code": 77 },
                    { "name": "Case1", "fields": [] },
                    { "name": "Case2", "fields": [], "code": 42 },
                    { "name": "Case3", "fields": [] },
                ]
            },
        },
    }))
    .unwrap();
    let idl_program2 = ToolboxIdlProgram::try_parse(&json!({
        "types": {
            "MyEnum": {
                "variants": [
                    { "name": "77", "code": 77 },
                    { "name": "Case1" },
                    { "name": "Case2", "code": 42 },
                    { "name": "Case3" },
                ]
            },
        },
    }))
    .unwrap();
    let idl_program3 = ToolboxIdlProgram::try_parse(&json!({
        "types": {
            "MyEnum": {
                "variants": [
                    77,
                    "Case1",
                    {"name": "Case2", "code": 42},
                    "Case3",
                ]
            },
        },
    }))
    .unwrap();
    let idl_program4 = ToolboxIdlProgram::try_parse(&json!({
        "types": {
            "MyEnum": {
                "variants": {
                    "77": 77,
                    "Case1": 1,
                    "Case2": 42,
                    "Case3": { "code": 3, "fields": [] },
                }
            },
        },
    }))
    .unwrap();
    // Assert that all are equivalent
    assert_eq!(idl_program1, idl_program2);
    assert_eq!(idl_program1, idl_program3);
    assert_eq!(idl_program1, idl_program4);
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
                        name: "77".to_string(),
                        code: 77,
                        docs: None,
                        fields: ToolboxIdlTypeFlatFields::nothing()
                    },
                    ToolboxIdlTypeFlatEnumVariant {
                        name: "Case1".to_string(),
                        code: 1,
                        docs: None,
                        fields: ToolboxIdlTypeFlatFields::nothing()
                    },
                    ToolboxIdlTypeFlatEnumVariant {
                        name: "Case2".to_string(),
                        code: 42,
                        docs: None,
                        fields: ToolboxIdlTypeFlatFields::nothing()
                    },
                    ToolboxIdlTypeFlatEnumVariant {
                        name: "Case3".to_string(),
                        code: 3,
                        docs: None,
                        fields: ToolboxIdlTypeFlatFields::nothing()
                    },
                ]
            }
        }
        .into()
    )
}
