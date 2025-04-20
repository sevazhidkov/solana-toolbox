use crate::toolbox_idl_type_prefix::ToolboxIdlTypePrefix;
use crate::toolbox_idl_type_primitive::ToolboxIdlTypePrimitive;

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlTypeFull {
    Option {
        prefix: ToolboxIdlTypePrefix,
        content: Box<ToolboxIdlTypeFull>,
    },
    Vec {
        prefix: ToolboxIdlTypePrefix,
        items: Box<ToolboxIdlTypeFull>,
    },
    Array {
        items: Box<ToolboxIdlTypeFull>,
        length: u64,
    },
    Struct {
        fields: ToolboxIdlTypeFullFields,
    },
    Enum {
        prefix: ToolboxIdlTypePrefix,
        variants: Vec<ToolboxIdlTypeFullEnumVariant>,
    },
    Padded {
        size_bytes: u64,
        content: Box<ToolboxIdlTypeFull>,
    },
    Const {
        literal: u64,
    },
    Primitive {
        primitive: ToolboxIdlTypePrimitive,
    },
}

impl From<ToolboxIdlTypePrimitive> for ToolboxIdlTypeFull {
    fn from(primitive: ToolboxIdlTypePrimitive) -> ToolboxIdlTypeFull {
        ToolboxIdlTypeFull::Primitive { primitive }
    }
}

impl ToolboxIdlTypeFull {
    pub fn nothing() -> ToolboxIdlTypeFull {
        ToolboxIdlTypeFull::Struct {
            fields: ToolboxIdlTypeFullFields::None,
        }
    }

    pub fn is_bytes(&self) -> bool {
        match self {
            ToolboxIdlTypeFull::Vec { prefix, items } => {
                prefix == &ToolboxIdlTypePrefix::U32
                    && items.is_primitive(&ToolboxIdlTypePrimitive::U8)
            },
            _ => false,
        }
    }

    pub fn is_primitive(&self, value: &ToolboxIdlTypePrimitive) -> bool {
        match self {
            ToolboxIdlTypeFull::Primitive { primitive } => primitive == value,
            _ => false,
        }
    }

    pub fn as_const_literal(&self) -> Option<&u64> {
        match self {
            ToolboxIdlTypeFull::Const { literal } => Some(literal),
            _ => None,
        }
    }

    pub fn as_struct_fields(&self) -> Option<&ToolboxIdlTypeFullFields> {
        match self {
            ToolboxIdlTypeFull::Struct { fields } => Some(fields),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlTypeFullEnumVariant {
    pub name: String,
    pub code: u64,
    pub fields: ToolboxIdlTypeFullFields,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlTypeFullFields {
    None,
    Named(Vec<ToolboxIdlTypeFullFieldNamed>),
    Unnamed(Vec<ToolboxIdlTypeFullFieldUnnamed>),
}

impl ToolboxIdlTypeFullFields {
    pub fn nothing() -> ToolboxIdlTypeFullFields {
        ToolboxIdlTypeFullFields::None
    }
}

impl ToolboxIdlTypeFullFields {
    pub fn as_named(&self) -> Option<&Vec<ToolboxIdlTypeFullFieldNamed>> {
        match self {
            ToolboxIdlTypeFullFields::Named(named) => Some(named),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlTypeFullFieldNamed {
    pub name: String,
    pub type_full: ToolboxIdlTypeFull,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlTypeFullFieldUnnamed {
    pub type_full: ToolboxIdlTypeFull,
}
