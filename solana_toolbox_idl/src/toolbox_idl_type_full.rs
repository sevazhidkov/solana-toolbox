use crate::toolbox_idl_type_prefix::ToolboxIdlTypePrefix;
use crate::toolbox_idl_type_primitive::ToolboxIdlTypePrimitive;

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlTypeFull {
    Typedef {
        name: String,
        repr: Option<String>,
        content: Box<ToolboxIdlTypeFull>,
    },
    Pod {
        // TODO - is this really necessary ?
        alignment: usize,
        size: usize,
        content: Box<ToolboxIdlTypeFull>,
    },
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
        length: usize,
    },
    String {
        prefix: ToolboxIdlTypePrefix,
    },
    Struct {
        fields: ToolboxIdlTypeFullFields,
    },
    Enum {
        prefix: ToolboxIdlTypePrefix,
        variants: Vec<ToolboxIdlTypeFullEnumVariant>,
    },
    Padded {
        before: usize,
        min_size: usize,
        after: usize,
        content: Box<ToolboxIdlTypeFull>,
    },
    Const {
        literal: u64, // TODO - this should not be needed
    },
    Primitive {
        primitive: ToolboxIdlTypePrimitive,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlTypeFullEnumVariant {
    pub name: String,
    pub code: u64,
    pub fields: ToolboxIdlTypeFullFields,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlTypeFullFields {
    Named(Vec<ToolboxIdlTypeFullFieldNamed>),
    Unnamed(Vec<ToolboxIdlTypeFullFieldUnnamed>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlTypeFullFieldNamed {
    pub name: String,
    pub content: ToolboxIdlTypeFull,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlTypeFullFieldUnnamed {
    pub position: usize,
    pub content: ToolboxIdlTypeFull,
}

impl From<ToolboxIdlTypePrimitive> for ToolboxIdlTypeFull {
    fn from(primitive: ToolboxIdlTypePrimitive) -> ToolboxIdlTypeFull {
        ToolboxIdlTypeFull::Primitive { primitive }
    }
}

impl ToolboxIdlTypeFull {
    pub fn nothing() -> ToolboxIdlTypeFull {
        ToolboxIdlTypeFull::Struct {
            fields: ToolboxIdlTypeFullFields::nothing(),
        }
    }

    pub fn is_vec32_u8(&self) -> bool {
        match self {
            ToolboxIdlTypeFull::Vec { prefix, items, .. } => {
                prefix == &ToolboxIdlTypePrefix::U32
                    && items.is_primitive(&ToolboxIdlTypePrimitive::U8)
            },
            _ => false,
        }
    }

    pub fn is_string32(&self) -> bool {
        match self {
            ToolboxIdlTypeFull::String { prefix } => {
                prefix == &ToolboxIdlTypePrefix::U32
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
}

impl ToolboxIdlTypeFullFields {
    pub fn nothing() -> ToolboxIdlTypeFullFields {
        ToolboxIdlTypeFullFields::Unnamed(vec![])
    }

    pub fn len(&self) -> usize {
        match self {
            ToolboxIdlTypeFullFields::Named(fields) => fields.len(),
            ToolboxIdlTypeFullFields::Unnamed(fields) => fields.len(),
        }
    }
}
