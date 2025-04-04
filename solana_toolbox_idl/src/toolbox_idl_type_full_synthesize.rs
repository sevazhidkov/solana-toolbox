use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
use crate::toolbox_idl_type_primitive::ToolboxIdlTypePrimitive;

impl ToolboxIdlTypeFull {
    pub fn synthesize(&self) -> Value {
        match self {
            ToolboxIdlTypeFull::Option { content, .. } => content.synthesize(),
            ToolboxIdlTypeFull::Vec { items } => {
                json!([items.synthesize()])
            },
            ToolboxIdlTypeFull::Array { items, length } => {
                let mut json_values = vec![];
                for _ in 0..usize::try_from(*length).unwrap() {
                    json_values.push(items.synthesize());
                }
                json!(json_values)
            },
            ToolboxIdlTypeFull::Struct { fields } => fields.synthesize(),
            ToolboxIdlTypeFull::Enum { variants } => {
                let mut json_variants = Map::new();
                for (variant_name, variant_fields) in variants {
                    json_variants.insert(
                        variant_name.to_string(),
                        variant_fields.synthesize(),
                    );
                }
                json!(json_variants)
            },
            ToolboxIdlTypeFull::Padded { content, .. } => content.synthesize(),
            ToolboxIdlTypeFull::Const { literal } => {
                json!(literal)
            },
            ToolboxIdlTypeFull::Primitive { primitive } => match primitive {
                ToolboxIdlTypePrimitive::U8 => json!(0),
                ToolboxIdlTypePrimitive::U16 => json!(0),
                ToolboxIdlTypePrimitive::U32 => json!(0),
                ToolboxIdlTypePrimitive::U64 => json!(0),
                ToolboxIdlTypePrimitive::U128 => json!(0),
                ToolboxIdlTypePrimitive::I8 => json!(0),
                ToolboxIdlTypePrimitive::I16 => json!(0),
                ToolboxIdlTypePrimitive::I32 => json!(0),
                ToolboxIdlTypePrimitive::I64 => json!(0),
                ToolboxIdlTypePrimitive::I128 => json!(0),
                ToolboxIdlTypePrimitive::F32 => json!(0),
                ToolboxIdlTypePrimitive::F64 => json!(0),
                ToolboxIdlTypePrimitive::Boolean => json!(false),
                ToolboxIdlTypePrimitive::String => json!(""),
                ToolboxIdlTypePrimitive::PublicKey => json!("PublicKey"),
            },
        }
    }
}

impl ToolboxIdlTypeFullFields {
    pub fn synthesize(&self) -> Value {
        match self {
            ToolboxIdlTypeFullFields::Named(fields) => {
                let mut json_fields = Map::new();
                for (field_name, field_type) in fields {
                    json_fields.insert(
                        field_name.to_string(),
                        field_type.synthesize(),
                    );
                }
                json!(json_fields)
            },
            ToolboxIdlTypeFullFields::Unnamed(fields) => {
                let mut json_fields = vec![];
                for field_type in fields {
                    json_fields.push(field_type.synthesize());
                }
                json!(json_fields)
            },
            ToolboxIdlTypeFullFields::None => {
                json!(null)
            },
        }
    }
}
