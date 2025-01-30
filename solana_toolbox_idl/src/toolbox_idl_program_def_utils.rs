use crate::toolbox_idl_program_def::ToolboxIdlProgramDef;
use crate::toolbox_idl_program_def_primitive::ToolboxIdlProgramDefPrimitive;

impl ToolboxIdlProgramDef {
    pub fn describe(&self) -> String {
        match self {
            ToolboxIdlProgramDef::Defined { name, generics } => {
                if generics.is_empty() {
                    format!("@{}", name)
                } else {
                    format!(
                        "@{}<{}>",
                        name,
                        generics
                            .iter()
                            .map(|generic| generic.describe())
                            .collect::<Vec<_>>()
                            .join(",")
                    )
                }
            },
            ToolboxIdlProgramDef::Option { content } => {
                format!("Option<{}>", content.describe())
            },
            ToolboxIdlProgramDef::Vec { items } => {
                format!("Vec<{}>", items.describe())
            },
            ToolboxIdlProgramDef::Array { items, length } => {
                format!("[{};{}]", items.describe(), length.describe())
            },
            ToolboxIdlProgramDef::Struct { fields } => {
                format!(
                    "Struct{{{}}}",
                    fields
                        .iter()
                        .map(|field| {
                            format!("{}:{}", field.0, field.1.describe())
                        })
                        .collect::<Vec<_>>()
                        .join(",")
                )
            },
            ToolboxIdlProgramDef::Enum { variants } => {
                format!(
                    "Enum{{{}}}",
                    variants
                        .iter()
                        .map(|variant| {
                            if variant.1.is_empty() {
                                variant.0.to_string()
                            } else {
                                format!(
                                    "{}[{}]",
                                    variant.0,
                                    variant
                                        .1
                                        .iter()
                                        .map(|field| field.describe())
                                        .collect::<Vec<_>>()
                                        .join(",")
                                )
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("/")
                )
            },
            ToolboxIdlProgramDef::Primitive { primitive } => {
                primitive.as_str().to_string()
            },
            ToolboxIdlProgramDef::Const { literal } => {
                format!("{}", literal)
            },
            ToolboxIdlProgramDef::Generic { symbol } => {
                format!("#{}", symbol)
            },
        }
    }

    pub fn as_struct_fields(
        &self
    ) -> Option<&Vec<(String, ToolboxIdlProgramDef)>> {
        match self {
            ToolboxIdlProgramDef::Struct { fields } => Some(fields),
            _ => None,
        }
    }

    // TODO - need to be able to lookup defined automatically
    pub fn as_const_literal(&self) -> Option<&usize> {
        match self {
            ToolboxIdlProgramDef::Const { literal } => Some(literal),
            _ => None,
        }
    }

    pub fn as_primitive(&self) -> Option<&ToolboxIdlProgramDefPrimitive> {
        match self {
            ToolboxIdlProgramDef::Primitive { primitive } => Some(primitive),
            _ => None,
        }
    }
}
