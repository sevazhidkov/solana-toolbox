use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;

impl ToolboxIdlTypeFull {
    pub fn explained(&self) -> Value {
        match self {
            ToolboxIdlTypeFull::Typedef { content, .. } => content.explained(),
            ToolboxIdlTypeFull::Option { content, .. } => {
                json!({ "option": content.explained() })
            },
            ToolboxIdlTypeFull::Vec { items, .. } => {
                json!([items.explained()])
            },
            ToolboxIdlTypeFull::Array { items, length } => {
                json!([items.explained(), length])
            },
            ToolboxIdlTypeFull::String { .. } => json!("string"),
            ToolboxIdlTypeFull::Struct { fields, .. } => fields.explained(),
            ToolboxIdlTypeFull::Enum { variants, .. } => {
                let mut json_variants = vec![];
                for variant in variants {
                    json_variants.push(if variant.fields.is_empty() {
                        json!(variant.name)
                    } else {
                        json!({ variant.name.to_string(): variant.fields.explained()})
                    });
                }
                json!({ "variants": json_variants })
            },
            ToolboxIdlTypeFull::Padded { content, .. } => content.explained(),
            ToolboxIdlTypeFull::Const { literal } => {
                json!(literal) // TODO - this makes no sense
            },
            ToolboxIdlTypeFull::Primitive { primitive } => {
                json!(primitive.as_str())
            },
        }
    }
}

impl ToolboxIdlTypeFullFields {
    pub fn explained(&self) -> Value {
        match self {
            ToolboxIdlTypeFullFields::Named(fields) => {
                let mut json_fields = Map::new();
                for field in fields {
                    json_fields.insert(
                        field.name.to_string(),
                        field.content.explained(),
                    );
                }
                json!(json_fields)
            },
            ToolboxIdlTypeFullFields::Unnamed(fields) => {
                let mut json_fields = vec![];
                for field in fields {
                    json_fields.push(field.content.explained());
                }
                json!(json_fields)
            },
        }
    }
}
