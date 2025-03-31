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
    Named(Vec<(String, ToolboxIdlTypeFlatField)>),
    Unamed(Vec<ToolboxIdlTypeFlatField>),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlTypeFlatField {
    pub docs: Option<Value>,
    pub type_flat: ToolboxIdlTypeFlat,
}

impl Into<ToolboxIdlTypeFlatField> for ToolboxIdlTypeFlat {
    fn into(self) -> ToolboxIdlTypeFlatField {
        ToolboxIdlTypeFlatField {
            docs: None,
            type_flat: self,
        }
    }
}
