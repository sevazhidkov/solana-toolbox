use crate::toolbox_idl_primitive::ToolboxIdlPrimitive;

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
    Const {
        literal: usize, // TODO - what other kind of consts can be supported ?
    },
    Primitive {
        primitive: ToolboxIdlPrimitive,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlTypeFlatFields {
    Named(Vec<(String, ToolboxIdlTypeFlat)>),
    Unamed(Vec<ToolboxIdlTypeFlat>),
    None,
}

impl ToolboxIdlTypeFlat {
    pub fn describe(&self) -> String {
        match self {
            ToolboxIdlTypeFlat::Defined { name, generics } => {
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
            ToolboxIdlTypeFlat::Generic { symbol } => {
                format!("#{}", symbol)
            },
            ToolboxIdlTypeFlat::Option { content } => {
                format!("Option<{}>", content.describe())
            },
            ToolboxIdlTypeFlat::Vec { items } => {
                format!("Vec<{}>", items.describe())
            },
            ToolboxIdlTypeFlat::Array { items, length } => {
                format!("[{};{}]", items.describe(), length.describe())
            },
            ToolboxIdlTypeFlat::Struct { fields } => {
                format!("Struct{}", fields.describe())
            },
            ToolboxIdlTypeFlat::Enum { variants } => {
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
            ToolboxIdlTypeFlat::Const { literal } => {
                format!("{}", literal)
            },
            ToolboxIdlTypeFlat::Primitive { primitive } => {
                primitive.as_str().to_string()
            },
        }
    }
}

impl ToolboxIdlTypeFlatFields {
    pub fn describe(&self) -> String {
        match self {
            ToolboxIdlTypeFlatFields::Named(fields) => format!(
                "{{{}}}",
                fields
                    .iter()
                    .map(|field| format!("{}:{}", field.0, field.1.describe()))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            ToolboxIdlTypeFlatFields::Unamed(fields) => format!(
                "({})",
                fields
                    .iter()
                    .map(|field| field.describe())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            ToolboxIdlTypeFlatFields::None => "".to_string(),
        }
    }
}
