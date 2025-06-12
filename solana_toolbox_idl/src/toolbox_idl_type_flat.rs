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
    String {
        prefix: ToolboxIdlTypePrefix,
    },
    Struct {
        fields: ToolboxIdlTypeFlatFields,
    },
    Enum {
        prefix: ToolboxIdlTypePrefix,
        variants: Vec<ToolboxIdlTypeFlatEnumVariant>,
    },
    Padded {
        before: usize,
        min_size: usize,
        after: usize,
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
            fields: ToolboxIdlTypeFlatFields::nothing(),
        }
    }
}

impl ToolboxIdlTypeFlatFields {
    pub fn nothing() -> ToolboxIdlTypeFlatFields {
        ToolboxIdlTypeFlatFields::Unnamed(vec![])
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ToolboxIdlTypeFlatFields::Named(fields) => fields.is_empty(),
            ToolboxIdlTypeFlatFields::Unnamed(fields) => fields.is_empty(),
        }
    }
}
