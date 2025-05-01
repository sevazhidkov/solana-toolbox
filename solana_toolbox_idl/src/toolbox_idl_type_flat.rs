use serde_json::Value;

use crate::toolbox_idl_type_prefix::ToolboxIdlTypePrefix;
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
        prefix: ToolboxIdlTypePrefix,
        content: Box<ToolboxIdlTypeFlat>,
    },
    Vec {
        prefix: ToolboxIdlTypePrefix,
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
        prefix: ToolboxIdlTypePrefix,
        variants: Vec<ToolboxIdlTypeFlatEnumVariant>,
    },
    Padded {
        before: u64,
        size: u64,
        after: u64,
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
pub struct ToolboxIdlTypeFlatEnumVariant {
    pub name: String,
    pub code: u64,
    pub docs: Option<Value>,
    pub fields: ToolboxIdlTypeFlatFields,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlTypeFlatFields {
    None,
    Named(Vec<ToolboxIdlTypeFlatFieldNamed>),
    Unnamed(Vec<ToolboxIdlTypeFlatFieldUnnamed>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlTypeFlatFieldNamed {
    pub name: String,
    pub docs: Option<Value>,
    pub content: ToolboxIdlTypeFlat,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlTypeFlatFieldUnnamed {
    pub docs: Option<Value>,
    pub content: ToolboxIdlTypeFlat,
}

impl From<ToolboxIdlTypePrimitive> for ToolboxIdlTypeFlat {
    fn from(primitive: ToolboxIdlTypePrimitive) -> ToolboxIdlTypeFlat {
        ToolboxIdlTypeFlat::Primitive { primitive }
    }
}

impl ToolboxIdlTypeFlat {
    pub fn nothing() -> ToolboxIdlTypeFlat {
        ToolboxIdlTypeFlat::Struct {
            fields: ToolboxIdlTypeFlatFields::None,
        }
    }
}

impl ToolboxIdlTypeFlatFields {
    pub fn nothing() -> ToolboxIdlTypeFlatFields {
        ToolboxIdlTypeFlatFields::None
    }
}
