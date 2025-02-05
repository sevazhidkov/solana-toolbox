use crate::toolbox_idl_primitive::ToolboxIdlPrimitive;

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlTypeFull {
    Option { content: Box<ToolboxIdlTypeFull> },
    Vec { items: Box<ToolboxIdlTypeFull> },
    Array { items: Box<ToolboxIdlTypeFull>, length: usize },
    Struct { fields: ToolboxIdlTypeFullFields },
    Enum { variants: Vec<(String, ToolboxIdlTypeFullFields)> },
    Const { literal: usize },
    Primitive { primitive: ToolboxIdlPrimitive },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlTypeFullFields {
    Named(Vec<(String, ToolboxIdlTypeFull)>),
    Unamed(Vec<ToolboxIdlTypeFull>),
    None,
}

impl ToolboxIdlTypeFull {
    pub fn describe(&self) -> String {
        match self {
            ToolboxIdlTypeFull::Option { content } => {
                format!("Option<{}>", content.describe())
            },
            ToolboxIdlTypeFull::Vec { items } => {
                format!("Vec<{}>", items.describe())
            },
            ToolboxIdlTypeFull::Array { items, length } => {
                format!("[{};{}]", items.describe(), length)
            },
            ToolboxIdlTypeFull::Struct { fields } => {
                format!("Struct{}", fields.describe())
            },
            ToolboxIdlTypeFull::Enum { variants } => {
                format!(
                    "Enum{{{}}}",
                    variants
                        .iter()
                        .map(|variant| {
                            format!("{}{}", variant.0, variant.1.describe())
                        })
                        .collect::<Vec<_>>()
                        .join("/")
                )
            },
            ToolboxIdlTypeFull::Primitive { primitive } => {
                primitive.as_str().to_string()
            },
            ToolboxIdlTypeFull::Const { literal } => {
                format!("{}", literal)
            },
        }
    }
}

impl ToolboxIdlTypeFullFields {
    pub fn describe(&self) -> String {
        match self {
            ToolboxIdlTypeFullFields::Named(fields) => {
                format!(
                    "{{{}}}",
                    fields
                        .iter()
                        .map(|field| {
                            format!("{}:{}", field.0, field.1.describe())
                        })
                        .collect::<Vec<_>>()
                        .join(",")
                )
            },
            ToolboxIdlTypeFullFields::Unamed(fields) => {
                format!(
                    "({})",
                    fields
                        .iter()
                        .map(|field| field.describe())
                        .collect::<Vec<_>>()
                        .join(",")
                )
            },
            ToolboxIdlTypeFullFields::None => "".to_string(),
        }
    }
}

impl ToolboxIdlTypeFull {
    pub fn as_const_literal(&self) -> Option<&usize> {
        match self {
            ToolboxIdlTypeFull::Const { literal } => Some(literal),
            _ => None,
        }
    }
}
