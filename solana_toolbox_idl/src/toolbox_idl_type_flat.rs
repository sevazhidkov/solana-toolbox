use crate::toolbox_idl_type_primitive::ToolboxIdlTypePrimitive;

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlTypeFlat {
    Defined {
        name: String,
        generics: Vec<ToolboxIdlTypeFlat>,
    },
    Generic {
        symbol: String,
    },
    Option {
        prefix_bytes: u8,
        content: Box<ToolboxIdlTypeFlat>,
    },
    Vec {
        items: Box<ToolboxIdlTypeFlat>,
    },
    Array {
        items: Box<ToolboxIdlTypeFlat>,
        length: Box<ToolboxIdlTypeFlat>,
    },
    Struct {
        fields: ToolboxIdlTypeFlatFields,
    },
    Enum {
        variants: Vec<(String, ToolboxIdlTypeFlatFields)>,
    },
    Padded {
        size_bytes: u64,
        content: Box<ToolboxIdlTypeFlat>,
    },
    Const {
        literal: u64,
    },
    Primitive {
        primitive: ToolboxIdlTypePrimitive,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlTypeFlatFields {
    Named(Vec<(String, ToolboxIdlTypeFlat)>),
    Unamed(Vec<ToolboxIdlTypeFlat>),
    None,
}
