use std::fs::read_to_string;

use solana_toolbox_idl_core::ToolboxIdlProgram;
use solana_toolbox_idl_core::ToolboxIdlTypeFull;
use solana_toolbox_idl_core::ToolboxIdlTypeFullEnumVariant;
use solana_toolbox_idl_core::ToolboxIdlTypeFullFieldNamed;
use solana_toolbox_idl_core::ToolboxIdlTypeFullFieldUnnamed;
use solana_toolbox_idl_core::ToolboxIdlTypeFullFields;
use solana_toolbox_idl_core::ToolboxIdlTypePrefix;
use solana_toolbox_idl_core::ToolboxIdlTypePrimitive;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl_program = ToolboxIdlProgram::try_parse_from_str(
        &read_to_string("./tests/fixtures/idl_anchor_generics.json").unwrap(),
    )
    .unwrap();
    // Check that the account was parsed correctly
    let idl_account = idl_program.accounts.get("GenericAccount").unwrap();
    assert_eq!(
        idl_account.content_type_full,
        ToolboxIdlTypeFull::Typedef {
            name: "GenericAccount".to_string(),
            repr: None,
            content: Box::new(ToolboxIdlTypeFull::Struct {
                fields: ToolboxIdlTypeFullFields::Named(vec![
                    ToolboxIdlTypeFullFieldNamed {
                        name: "data".to_string(),
                        content: make_type_full_generic_type(
                            make_type_full_u32(),
                            make_type_full_u64(),
                            10
                        ),
                    },
                ]),
            })
        }
    );
    // Check that the instruction was parsed correctly
    let idl_instruction = idl_program.instructions.get("generic").unwrap();
    assert_eq!(
        idl_instruction.args_type_full_fields,
        ToolboxIdlTypeFullFields::Named(vec![ToolboxIdlTypeFullFieldNamed {
            name: "generic_field".to_string(),
            content: make_type_full_generic_type(
                make_type_full_u32(),
                make_type_full_u64(),
                10
            ),
        }])
    );
}

fn make_type_full_generic_type(
    t: ToolboxIdlTypeFull,
    u: ToolboxIdlTypeFull,
    n: usize,
) -> ToolboxIdlTypeFull {
    ToolboxIdlTypeFull::Typedef {
        name: "GenericType".to_string(),
        repr: None,
        content: Box::new(ToolboxIdlTypeFull::Struct {
            fields: ToolboxIdlTypeFullFields::Named(vec![
                ToolboxIdlTypeFullFieldNamed {
                    name: "gen1".to_string(),
                    content: t.clone(),
                },
                ToolboxIdlTypeFullFieldNamed {
                    name: "gen2".to_string(),
                    content: u.clone(),
                },
                ToolboxIdlTypeFullFieldNamed {
                    name: "gen3".to_string(),
                    content: make_type_full_generic_nested(
                        make_type_full_u32(),
                        u.clone(),
                    ),
                },
                ToolboxIdlTypeFullFieldNamed {
                    name: "gen4".to_string(),
                    content: make_type_full_generic_nested(
                        t.clone(),
                        make_type_full_my_struct(),
                    ),
                },
                ToolboxIdlTypeFullFieldNamed {
                    name: "gen5".to_string(),
                    content: make_type_full_generic_nested(
                        t.clone(),
                        u.clone(),
                    ),
                },
                ToolboxIdlTypeFullFieldNamed {
                    name: "gen6".to_string(),
                    content: make_type_full_generic_nested(
                        make_type_full_u32(),
                        make_type_full_u64(),
                    ),
                },
                ToolboxIdlTypeFullFieldNamed {
                    name: "gen7".to_string(),
                    content: make_type_full_generic_nested(
                        t.clone(),
                        make_type_full_generic_nested(t.clone(), u.clone()),
                    ),
                },
                ToolboxIdlTypeFullFieldNamed {
                    name: "arr".to_string(),
                    content: make_type_full_array(make_type_full_u8(), n),
                },
                ToolboxIdlTypeFullFieldNamed {
                    name: "warr".to_string(),
                    content: make_type_full_wrapped_u8_array(n),
                },
                ToolboxIdlTypeFullFieldNamed {
                    name: "warrval".to_string(),
                    content: make_type_full_wrapped_u8_array(10),
                },
                ToolboxIdlTypeFullFieldNamed {
                    name: "enm1".to_string(),
                    content: make_type_full_generic_enum(
                        t.clone(),
                        u.clone(),
                        n,
                    ),
                },
                ToolboxIdlTypeFullFieldNamed {
                    name: "enm2".to_string(),
                    content: make_type_full_generic_enum(
                        make_type_full_generic_nested(
                            t.clone(),
                            make_type_full_u64(),
                        ),
                        make_type_full_u32(),
                        30,
                    ),
                },
            ]),
        }),
    }
}

fn make_type_full_generic_enum(
    t: ToolboxIdlTypeFull,
    u: ToolboxIdlTypeFull,
    n: usize,
) -> ToolboxIdlTypeFull {
    ToolboxIdlTypeFull::Typedef {
        name: "GenericEnum".to_string(),
        repr: None,
        content: Box::new(ToolboxIdlTypeFull::Enum {
            prefix: ToolboxIdlTypePrefix::U8,
            variants: vec![
                ToolboxIdlTypeFullEnumVariant {
                    name: "Unnamed".to_string(),
                    code: 0,
                    fields: ToolboxIdlTypeFullFields::Unnamed(vec![
                        ToolboxIdlTypeFullFieldUnnamed {
                            position: 0,
                            content: t.clone(),
                        },
                        ToolboxIdlTypeFullFieldUnnamed {
                            position: 1,
                            content: u.clone(),
                        },
                    ]),
                },
                ToolboxIdlTypeFullEnumVariant {
                    name: "Named".to_string(),
                    code: 1,
                    fields: ToolboxIdlTypeFullFields::Named(vec![
                        ToolboxIdlTypeFullFieldNamed {
                            name: "gen1".to_string(),
                            content: t.clone(),
                        },
                        ToolboxIdlTypeFullFieldNamed {
                            name: "gen2".to_string(),
                            content: u.clone(),
                        },
                    ]),
                },
                ToolboxIdlTypeFullEnumVariant {
                    name: "Struct".to_string(),
                    code: 2,
                    fields: ToolboxIdlTypeFullFields::Unnamed(vec![
                        ToolboxIdlTypeFullFieldUnnamed {
                            position: 0,
                            content: make_type_full_generic_nested(
                                t.clone(),
                                u.clone(),
                            ),
                        },
                    ]),
                },
                ToolboxIdlTypeFullEnumVariant {
                    name: "Arr".to_string(),
                    code: 3,
                    fields: ToolboxIdlTypeFullFields::Unnamed(vec![
                        ToolboxIdlTypeFullFieldUnnamed {
                            position: 0,
                            content: make_type_full_array(t.clone(), n),
                        },
                    ]),
                },
            ],
        }),
    }
}

fn make_type_full_my_struct() -> ToolboxIdlTypeFull {
    ToolboxIdlTypeFull::Typedef {
        name: "MyStruct".to_string(),
        repr: None,
        content: Box::new(ToolboxIdlTypeFull::Struct {
            fields: ToolboxIdlTypeFullFields::Named(vec![
                ToolboxIdlTypeFullFieldNamed {
                    name: "some_field".to_string(),
                    content: make_type_full_u8(),
                },
            ]),
        }),
    }
}

fn make_type_full_generic_nested(
    v: ToolboxIdlTypeFull,
    z: ToolboxIdlTypeFull,
) -> ToolboxIdlTypeFull {
    ToolboxIdlTypeFull::Typedef {
        name: "GenericNested".to_string(),
        repr: None,
        content: Box::new(ToolboxIdlTypeFull::Struct {
            fields: ToolboxIdlTypeFullFields::Named(vec![
                ToolboxIdlTypeFullFieldNamed {
                    name: "gen1".to_string(),
                    content: v,
                },
                ToolboxIdlTypeFullFieldNamed {
                    name: "gen2".to_string(),
                    content: z,
                },
            ]),
        }),
    }
}

fn make_type_full_wrapped_u8_array(_n: usize) -> ToolboxIdlTypeFull {
    ToolboxIdlTypeFull::Typedef {
        name: "WrappedU8Array".to_string(),
        repr: None,
        content: Box::new(ToolboxIdlTypeFull::Struct {
            fields: ToolboxIdlTypeFullFields::Unnamed(vec![
                ToolboxIdlTypeFullFieldUnnamed {
                    position: 0,
                    content: make_type_full_u8(),
                },
            ]),
        }),
    }
}

fn make_type_full_array(
    items: ToolboxIdlTypeFull,
    length: usize,
) -> ToolboxIdlTypeFull {
    ToolboxIdlTypeFull::Array {
        items: items.into(),
        length,
    }
}

fn make_type_full_u8() -> ToolboxIdlTypeFull {
    ToolboxIdlTypePrimitive::U8.into()
}

fn make_type_full_u32() -> ToolboxIdlTypeFull {
    ToolboxIdlTypePrimitive::U32.into()
}

fn make_type_full_u64() -> ToolboxIdlTypeFull {
    ToolboxIdlTypePrimitive::U64.into()
}
