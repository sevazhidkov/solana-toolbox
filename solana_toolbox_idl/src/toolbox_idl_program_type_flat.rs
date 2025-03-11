use crate::toolbox_idl_program_type_primitive::ToolboxIdlProgramTypePrimitive;

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlProgramTypeFlat {
    Defined {
        name: String,
        generics: Vec<ToolboxIdlProgramTypeFlat>,
    },
    Generic {
        symbol: String,
    },
    Option {
        content: Box<ToolboxIdlProgramTypeFlat>,
    },
    Vec {
        items: Box<ToolboxIdlProgramTypeFlat>,
    },
    Array {
        items: Box<ToolboxIdlProgramTypeFlat>,
        length: Box<ToolboxIdlProgramTypeFlat>,
    },
    Struct {
        fields: ToolboxIdlProgramTypeFlatFields,
    },
    Enum {
        variants: Vec<(String, ToolboxIdlProgramTypeFlatFields)>,
    },
    Const {
        literal: usize, // TODO - what other kind of consts can be supported ?
    },
    Primitive {
        primitive: ToolboxIdlProgramTypePrimitive,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlProgramTypeFlatFields {
    Named(Vec<(String, ToolboxIdlProgramTypeFlat)>),
    Unamed(Vec<ToolboxIdlProgramTypeFlat>),
    None,
}
