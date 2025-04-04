use serde_json::Value;

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
        variants: Vec<(String, Option<Value>, ToolboxIdlTypeFlatFields)>,
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

impl ToolboxIdlTypeFlat {
    pub fn nothing() -> ToolboxIdlTypeFlat {
        ToolboxIdlTypeFlat::Struct {
            fields: ToolboxIdlTypeFlatFields::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlTypeFlatFields {
    None,
    Named(Vec<(String, Option<Value>, ToolboxIdlTypeFlat)>),
    Unnamed(Vec<(Option<Value>, ToolboxIdlTypeFlat)>),
}

impl ToolboxIdlTypeFlatFields {
    pub fn nothing() -> ToolboxIdlTypeFlatFields {
        ToolboxIdlTypeFlatFields::None
    }
}
