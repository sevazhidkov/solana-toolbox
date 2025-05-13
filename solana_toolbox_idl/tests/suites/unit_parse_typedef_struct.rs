use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypeFlatFields;
use solana_toolbox_idl::ToolboxIdlTypedef;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl_program1 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "types": [
            {
                "name": "MyStruct",
                "docs": ["Hello world!"],
                "type": { "fields": [] }
            },
        ],
    }))
    .unwrap();
    let idl_program2 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "types": [
            {
                "name": "MyStruct",
                "docs": ["Hello world!"],
                "fields": [],
            },
        ],
    }))
    .unwrap();
    let idl_program3 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "types": {
            "MyStruct": {
                "docs": ["Hello world!"],
                "type": { "fields": [] }
            },
        },
    }))
    .unwrap();
    let idl_program4 = ToolboxIdlProgram::try_parse_from_value(&json!({
        "types": {
            "MyStruct": {
                "docs": ["Hello world!"],
                "fields": []
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
        *idl_program1.typedefs.get("MyStruct").unwrap(),
        ToolboxIdlTypedef {
            name: "MyStruct".to_string(),
            docs: Some(json!(vec!["Hello world!"])),
            serialization: None,
            repr: None,
            generics: vec![],
            type_flat: ToolboxIdlTypeFlat::Struct {
                fields: ToolboxIdlTypeFlatFields::None
            }
        }
        .into()
    )
}
