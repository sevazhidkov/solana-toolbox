use serde_json::json;
use solana_toolbox_idl_core::ToolboxIdlProgram;
use solana_toolbox_idl_core::ToolboxIdlTypeFlat;
use solana_toolbox_idl_core::ToolboxIdlTypePrefix;
use solana_toolbox_idl_core::ToolboxIdlTypedef;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl_program1 = ToolboxIdlProgram::try_parse(&json!({
        "types": [
            {
                "name": "MyEnum",
                "docs": ["Hello world!"],
                "type": { "variants": [] }
            },
        ],
    }))
    .unwrap();
    let idl_program2 = ToolboxIdlProgram::try_parse(&json!({
        "types": [
            {
                "name": "MyEnum",
                "docs": ["Hello world!"],
                "variants": [],
            },
        ],
    }))
    .unwrap();
    let idl_program3 = ToolboxIdlProgram::try_parse(&json!({
        "types": {
            "MyEnum": {
                "docs": ["Hello world!"],
                "type": { "variants": [] }
            },
        },
    }))
    .unwrap();
    let idl_program4 = ToolboxIdlProgram::try_parse(&json!({
        "types": {
            "MyEnum": {
                "docs": ["Hello world!"],
                "variants": []
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
            docs: Some(json!(vec!["Hello world!"])),
            serialization: None,
            repr: None,
            generics: vec![],
            type_flat: ToolboxIdlTypeFlat::Enum {
                prefix: ToolboxIdlTypePrefix::U8,
                variants: vec![]
            }
        }
        .into()
    )
}
