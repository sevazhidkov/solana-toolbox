use crate::toolbox_idl_type_primitive::ToolboxIdlTypePrimitive;

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlTypeFullFields {
    Named(Vec<(String, ToolboxIdlTypeFull)>),
    Unamed(Vec<ToolboxIdlTypeFull>),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlTypeFull {
    Option {
        content: Box<ToolboxIdlTypeFull>,
    },
    Vec {
        items: Box<ToolboxIdlTypeFull>,
    },
    Array {
        items: Box<ToolboxIdlTypeFull>,
        length: usize,
    },
    Struct {
        fields: ToolboxIdlTypeFullFields,
    },
    Enum {
        variants: Vec<(String, ToolboxIdlTypeFullFields)>,
    },
    Const {
        literal: usize,
    },
    Primitive {
        primitive: ToolboxIdlTypePrimitive,
    },
}

impl ToolboxIdlTypeFull {
    pub fn as_const_literal(&self) -> Option<&usize> {
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
