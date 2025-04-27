use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullEnumVariant;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;

impl ToolboxIdlTypeFull {
    pub fn explain(&self) -> Value {
        match self {
            ToolboxIdlTypeFull::Option { content, .. } => {
                json!({ "option": content.explain() })
            },
            ToolboxIdlTypeFull::Vec { items, .. } => {
                json!([items.explain()])
            },
            ToolboxIdlTypeFull::Array { items, length } => {
                json!([items.explain(), length])
            },
            ToolboxIdlTypeFull::Struct { fields, .. } => fields.explain(),
            ToolboxIdlTypeFull::Enum { variants, .. } => {
                let mut json_variants = vec![];
                for variant in variants {
                    json_variants.push(variant.explain());
                }
                json!({ "variants": json_variants })
            },
            ToolboxIdlTypeFull::Padded { content, .. } => content.explain(),
            ToolboxIdlTypeFull::Const { literal } => {
                json!(literal)
            },
            ToolboxIdlTypeFull::Primitive { primitive } => {
                json!(primitive.as_str())
            },
        }
    }
}

impl ToolboxIdlTypeFullEnumVariant {
    pub fn explain(&self) -> Value {
        if self.fields == ToolboxIdlTypeFullFields::None {
            json!(self.name)
        } else {
            json!({ self.name.to_string(): self.fields.explain()})
        }
    }
}

impl ToolboxIdlTypeFullFields {
    pub fn explain(&self) -> Value {
        match self {
            ToolboxIdlTypeFullFields::Named(fields) => {
                let mut json_fields = Map::new();
                for field in fields {
                    json_fields.insert(
                        field.name.to_string(),
                        field.content.explain(),
                    );
                }
                json!(json_fields)
            },
            ToolboxIdlTypeFullFields::Unnamed(fields) => {
                let mut json_fields = vec![];
                for field in fields {
                    json_fields.push(field.content.explain());
                }
                json!(json_fields)
            },
            ToolboxIdlTypeFullFields::None => {
                json!(null)
            },
        }
    }
}
