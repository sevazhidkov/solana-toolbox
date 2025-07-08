use serde_json::json;
use solana_toolbox_idl_core::ToolboxIdlProgram;
use solana_toolbox_idl_core::ToolboxIdlTypeFlat;
use solana_toolbox_idl_core::ToolboxIdlTypeFlatEnumVariant;
use solana_toolbox_idl_core::ToolboxIdlTypeFlatFieldNamed;
use solana_toolbox_idl_core::ToolboxIdlTypeFlatFieldUnnamed;
use solana_toolbox_idl_core::ToolboxIdlTypeFlatFields;
use solana_toolbox_idl_core::ToolboxIdlTypePrefix;
use solana_toolbox_idl_core::ToolboxIdlTypePrimitive;
use solana_toolbox_idl_core::ToolboxIdlTypedef;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl_program1 = ToolboxIdlProgram::try_parse(&json!({
        "types": {
            "MyEnum": {
                "variants": [
                    {
                        "name": "Named",
                        "fields": [
                            { "name": "f1", "type": {"defined": "Other"} },
                            { "name": "f2", "type": {"vec": "u8"} },
                            { "name": "f3", "type": {"generic": "G"} },
                        ]
                    },
                    {
                        "name": "Unnamed",
                        "fields": [
                            { "type": "u64" },
                            { "type": ["u8"] },
                            { "type": {"vec": "u8"} },
                        ]
                    },
                    { "name": "Empty", "fields": [] },
                ]
            },
        },
    }))
    .unwrap();
    let idl_program2 = ToolboxIdlProgram::try_parse(&json!({
        "types": {
            "MyEnum": {
                "variants": [
                    {
                        "name": "Named",
                        "fields": [
                            { "name": "f1", "defined": "Other" },
                            { "name": "f2", "vec": "u8" },
                            { "name": "f3", "generic": "G" },
                        ]
                    },
                    {
                        "name": "Unnamed",
                        "fields": [
                            "u64",
                            ["u8"],
                            {"vec": "u8"},
                        ],
                    },
                    { "name": "Empty" },
                ]
            },
        },
    }))
    .unwrap();
    // Assert that all are equivalent
    assert_eq!(idl_program1, idl_program2);
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
                        name: "Named".to_string(),
                        code: 0,
                        docs: None,
                        fields: ToolboxIdlTypeFlatFields::Named(vec![
                            ToolboxIdlTypeFlatFieldNamed {
                                docs: None,
                                name: "f1".to_string(),
                                content: ToolboxIdlTypeFlat::Defined {
                                    name: "Other".to_string(),
                                    generics: vec![]
                                }
                            },
                            ToolboxIdlTypeFlatFieldNamed {
                                docs: None,
                                name: "f2".to_string(),
                                content: ToolboxIdlTypeFlat::Vec {
                                    prefix: ToolboxIdlTypePrefix::U32,
                                    items: Box::new(
                                        ToolboxIdlTypePrimitive::U8.into()
                                    )
                                }
                            },
                            ToolboxIdlTypeFlatFieldNamed {
                                docs: None,
                                name: "f3".to_string(),
                                content: ToolboxIdlTypeFlat::Generic {
                                    symbol: "G".to_string()
                                }
                            },
                        ])
                    },
                    ToolboxIdlTypeFlatEnumVariant {
                        name: "Unnamed".to_string(),
                        code: 1,
                        docs: None,
                        fields: ToolboxIdlTypeFlatFields::Unnamed(vec![
                            ToolboxIdlTypeFlatFieldUnnamed {
                                docs: None,
                                content: ToolboxIdlTypePrimitive::U64.into()
                            },
                            ToolboxIdlTypeFlatFieldUnnamed {
                                docs: None,
                                content: ToolboxIdlTypeFlat::Vec {
                                    prefix: ToolboxIdlTypePrefix::U32,
                                    items: Box::new(
                                        ToolboxIdlTypePrimitive::U8.into()
                                    )
                                }
                            },
                            ToolboxIdlTypeFlatFieldUnnamed {
                                docs: None,
                                content: ToolboxIdlTypeFlat::Vec {
                                    prefix: ToolboxIdlTypePrefix::U32,
                                    items: Box::new(
                                        ToolboxIdlTypePrimitive::U8.into()
                                    )
                                }
                            },
                        ]),
                    },
                    ToolboxIdlTypeFlatEnumVariant {
                        name: "Empty".to_string(),
                        code: 2,
                        docs: None,
                        fields: ToolboxIdlTypeFlatFields::nothing()
                    },
                ]
            }
        }
        .into()
    )
}
