use crate::toolbox_idl_program_typedef::ToolboxIdlProgramTypedef;
use crate::toolbox_idl_program_typedef_primitive::ToolboxIdlProgramTypedefPrimitiveKind;

impl ToolboxIdlProgramTypedef {
    pub fn describe(&self) -> String {
        match self {
            ToolboxIdlProgramTypedef::Defined { name } => name.to_string(),
            ToolboxIdlProgramTypedef::Option { content } => {
                format!("Option<{}>", content.describe())
            },
            ToolboxIdlProgramTypedef::Vec { items } => {
                format!("Vec<{}>", items.describe())
            },
            ToolboxIdlProgramTypedef::Array { length, items } => {
                format!("[{}; {}]", items.describe(), length)
            },
            ToolboxIdlProgramTypedef::Struct { .. } => "Struct()".to_string(),
            ToolboxIdlProgramTypedef::Enum { .. } => "Enum()".to_string(),
            ToolboxIdlProgramTypedef::Primitive { kind } => {
                kind.as_str().to_string()
            },
        }
    }

    pub fn as_struct_fields(
        &self
    ) -> Option<&Vec<(String, ToolboxIdlProgramTypedef)>> {
        match self {
            ToolboxIdlProgramTypedef::Struct { fields } => Some(fields),
            _ => None,
        }
    }

    pub fn as_primitive_kind(
        &self
    ) -> Option<&ToolboxIdlProgramTypedefPrimitiveKind> {
        match self {
            ToolboxIdlProgramTypedef::Primitive { kind } => Some(kind),
            _ => None,
        }
    }
}
