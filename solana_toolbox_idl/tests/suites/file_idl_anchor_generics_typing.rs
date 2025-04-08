use std::fs::read_to_string;

use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlTypeFull;
use solana_toolbox_idl::ToolboxIdlTypeFullFields;
use solana_toolbox_idl::ToolboxIdlTypePrimitive;

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
        ToolboxIdlTypeFull::Struct {
            fields: ToolboxIdlTypeFullFields::Named(vec![(
                "data".to_string(),
                make_type_full_generic_type(
                    make_type_full_u32(),
                    make_type_full_u64(),
                    make_type_full_const(10)
                ),
            )]),
        }
    );
    // Check that the instruction was parsed correctly
    let idl_instruction = idl_program.instructions.get("generic").unwrap();
    assert_eq!(
        idl_instruction.args_type_full_fields,
        ToolboxIdlTypeFullFields::Named(vec![(
            "generic_field".to_string(),
            make_type_full_generic_type(
                make_type_full_u32(),
                make_type_full_u64(),
                make_type_full_const(10)
            )
        )])
    );
}

fn make_type_full_generic_type(
    t: ToolboxIdlTypeFull,
    u: ToolboxIdlTypeFull,
    n: ToolboxIdlTypeFull,
) -> ToolboxIdlTypeFull {
    ToolboxIdlTypeFull::Struct {
        fields: ToolboxIdlTypeFullFields::Named(vec![
            ("gen1".to_string(), t.clone()),
            ("gen2".to_string(), u.clone()),
            (
                "gen3".to_string(),
                make_type_full_generic_nested(make_type_full_u32(), u.clone()),
            ),
            (
                "gen4".to_string(),
                make_type_full_generic_nested(
                    t.clone(),
                    make_type_full_my_struct(),
                ),
            ),
            (
                "gen5".to_string(),
                make_type_full_generic_nested(t.clone(), u.clone()),
            ),
            (
                "gen6".to_string(),
                make_type_full_generic_nested(
                    make_type_full_u32(),
                    make_type_full_u64(),
                ),
            ),
            (
                "gen7".to_string(),
                make_type_full_generic_nested(
                    t.clone(),
                    make_type_full_generic_nested(t.clone(), u.clone()),
                ),
            ),
            (
                "arr".to_string(),
                make_type_full_array(make_type_full_u8(), n.clone()),
            ),
            (
                "warr".to_string(),
                make_type_full_wrapped_u8_array(n.clone()),
            ),
            (
                "warrval".to_string(),
                make_type_full_wrapped_u8_array(make_type_full_const(10)),
            ),
            (
                "enm1".to_string(),
                make_type_full_generic_enum(t.clone(), u.clone(), n.clone()),
            ),
            (
                "enm2".to_string(),
                make_type_full_generic_enum(
                    make_type_full_generic_nested(
                        t.clone(),
                        make_type_full_u64(),
                    ),
                    make_type_full_u32(),
                    make_type_full_const(30),
                ),
            ),
        ]),
    }
}

fn make_type_full_generic_enum(
    t: ToolboxIdlTypeFull,
    u: ToolboxIdlTypeFull,
    n: ToolboxIdlTypeFull,
) -> ToolboxIdlTypeFull {
    ToolboxIdlTypeFull::Enum {
        variants: vec![
            (
                "Unnamed".to_string(),
                ToolboxIdlTypeFullFields::Unnamed(vec![t.clone(), u.clone()]),
            ),
            (
                "Named".to_string(),
                ToolboxIdlTypeFullFields::Named(vec![
                    ("gen1".to_string(), t.clone()),
                    ("gen2".to_string(), u.clone()),
                ]),
            ),
            (
                "Struct".to_string(),
                ToolboxIdlTypeFullFields::Unnamed(vec![
                    make_type_full_generic_nested(t.clone(), u.clone()),
                ]),
            ),
            (
                "Arr".to_string(),
                ToolboxIdlTypeFullFields::Unnamed(vec![make_type_full_array(
                    t.clone(),
                    n.clone(),
                )]),
            ),
        ],
    }
}

fn make_type_full_my_struct() -> ToolboxIdlTypeFull {
    ToolboxIdlTypeFull::Struct {
        fields: ToolboxIdlTypeFullFields::Named(vec![(
            "some_field".to_string(),
            make_type_full_u8(),
        )]),
    }
}

fn make_type_full_generic_nested(
    v: ToolboxIdlTypeFull,
    z: ToolboxIdlTypeFull,
) -> ToolboxIdlTypeFull {
    ToolboxIdlTypeFull::Struct {
        fields: ToolboxIdlTypeFullFields::Named(vec![
            ("gen1".to_string(), v),
            ("gen2".to_string(), z),
        ]),
    }
}

fn make_type_full_wrapped_u8_array(
    _n: ToolboxIdlTypeFull,
) -> ToolboxIdlTypeFull {
    ToolboxIdlTypeFull::Struct {
        fields: ToolboxIdlTypeFullFields::Unnamed(vec![make_type_full_u8()]),
    }
}

fn make_type_full_array(
    items: ToolboxIdlTypeFull,
    length: ToolboxIdlTypeFull,
) -> ToolboxIdlTypeFull {
    ToolboxIdlTypeFull::Array {
        items: items.into(),
        length: *length.as_const_literal().unwrap(),
    }
}

fn make_type_full_u8() -> ToolboxIdlTypeFull {
    ToolboxIdlTypeFull::Primitive {
        primitive: ToolboxIdlTypePrimitive::U8,
    }
}

fn make_type_full_u32() -> ToolboxIdlTypeFull {
    ToolboxIdlTypeFull::Primitive {
        primitive: ToolboxIdlTypePrimitive::U32,
    }
}

fn make_type_full_u64() -> ToolboxIdlTypeFull {
    ToolboxIdlTypeFull::Primitive {
        primitive: ToolboxIdlTypePrimitive::U64,
    }
}

fn make_type_full_const(literal: u64) -> ToolboxIdlTypeFull {
    ToolboxIdlTypeFull::Const { literal }
}
