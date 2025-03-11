use crate::toolbox_idl_program_type_primitive::ToolboxIdlProgramTypePrimitive;

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlProgramTypeFull {
    Option {
        content: Box<ToolboxIdlProgramTypeFull>,
    },
    Vec {
        items: Box<ToolboxIdlProgramTypeFull>,
    },
    Array {
        items: Box<ToolboxIdlProgramTypeFull>,
        length: usize,
    },
    Struct {
        fields: ToolboxIdlProgramTypeFullFields,
    },
    Enum {
        variants: Vec<(String, ToolboxIdlProgramTypeFullFields)>,
    },
    Const {
        literal: usize,
    },
    Primitive {
        primitive: ToolboxIdlProgramTypePrimitive,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlProgramTypeFullFields {
    Named(Vec<(String, ToolboxIdlProgramTypeFull)>),
    Unamed(Vec<ToolboxIdlProgramTypeFull>),
    None,
}

impl ToolboxIdlProgramTypeFull {
    pub fn as_const_literal(&self) -> Option<&usize> {
        match self {
            ToolboxIdlProgramTypeFull::Const { literal } => Some(literal),
            _ => None,
        }
    }
}
