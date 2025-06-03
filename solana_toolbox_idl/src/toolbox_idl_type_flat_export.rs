use std::ops::Deref;

use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_format::ToolboxIdlFormat;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatEnumVariant;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFieldNamed;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFieldUnnamed;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;
use crate::toolbox_idl_type_prefix::ToolboxIdlTypePrefix;
use crate::toolbox_idl_type_primitive::ToolboxIdlTypePrimitive;
use crate::toolbox_idl_utils::idl_convert_to_camel_case;

impl ToolboxIdlTypeFlat {
    pub fn export(&self, format: &ToolboxIdlFormat) -> Value {
        match self {
            ToolboxIdlTypeFlat::Defined { name, generics } => {
                if !generics.is_empty() {
                    let mut json_generics = vec![];
                    for generic in generics {
                        if format.can_skip_type_kind_key {
                            json_generics.push(generic.export(format));
                        } else {
                            json_generics.push(json!({
                                "kind": "type",
                                "type": generic.export(format),
                            }));
                        }
                    }
                    return json!({ "defined": { "name": name, "generics": json_generics }});
                }
                if format.can_shortcut_defined_name_to_string_if_no_generic {
                    return json!(name);
                }
                if format.can_skip_defined_name_object_wrap {
                    return json!({ "defined": name });
                }
                json!({ "defined": { "name": name }})
            },
            ToolboxIdlTypeFlat::Generic { symbol } => {
                json!({ "generic": symbol })
            },
            ToolboxIdlTypeFlat::Option { prefix, content } => {
                let key = match prefix {
                    ToolboxIdlTypePrefix::U8 => "option",
                    ToolboxIdlTypePrefix::U16 => "option16",
                    ToolboxIdlTypePrefix::U32 => "option32",
                    ToolboxIdlTypePrefix::U64 => "option64",
                    ToolboxIdlTypePrefix::U128 => "option128",
                };
                json!({ key: content.export(format) })
            },
            ToolboxIdlTypeFlat::Vec { prefix, items } => {
                let key = match prefix {
                    ToolboxIdlTypePrefix::U8 => "vec8",
                    ToolboxIdlTypePrefix::U16 => "vec16",
                    ToolboxIdlTypePrefix::U32 => "vec",
                    ToolboxIdlTypePrefix::U64 => "vec64",
                    ToolboxIdlTypePrefix::U128 => "vec128",
                };
                if key != "vec" {
                    return json!({ key: items.export(format) });
                }
                if format.can_shortcut_type_vec_notation {
                    return json!([items.export(format)]);
                }
                if items.deref().eq(&ToolboxIdlTypePrimitive::U8.into()) {
                    return json!("bytes");
                }
                json!({ "vec": items.export(format) })
            },
            ToolboxIdlTypeFlat::Array { items, length } => {
                if format.can_shortcut_type_array_notation {
                    return json!([
                        items.export(format),
                        length.export(format)
                    ]);
                }
                json!({ "array": [
                    items.export(format),
                    length.export(format)
                ]})
            },
            ToolboxIdlTypeFlat::String { prefix } => json!(match prefix {
                ToolboxIdlTypePrefix::U8 => "string8",
                ToolboxIdlTypePrefix::U16 => "string16",
                ToolboxIdlTypePrefix::U32 => "string",
                ToolboxIdlTypePrefix::U64 => "string64",
                ToolboxIdlTypePrefix::U128 => "string128",
            }),
            ToolboxIdlTypeFlat::Struct { fields } => {
                if format.can_skip_type_kind_key {
                    return json!({ "fields": fields.export(format) });
                }
                json!({
                    "kind": "struct",
                    "fields": fields.export(format)
                })
            },
            ToolboxIdlTypeFlat::Enum { prefix, variants } => {
                let mut json_variants = vec![];
                for (index, variant) in variants.iter().enumerate() {
                    json_variants.push(variant.export(index, format));
                }
                let mut json_enum = Map::new();
                if !format.can_skip_type_kind_key {
                    json_enum.insert("kind".to_string(), json!("enum"));
                }
                let key = match prefix {
                    ToolboxIdlTypePrefix::U8 => "variants",
                    ToolboxIdlTypePrefix::U16 => "variants16",
                    ToolboxIdlTypePrefix::U32 => "variants32",
                    ToolboxIdlTypePrefix::U64 => "variants64",
                    ToolboxIdlTypePrefix::U128 => "variants128",
                };
                json_enum.insert(key.to_string(), json!(json_variants));
                json!(json_enum)
            },
            ToolboxIdlTypeFlat::Padded {
                before,
                min_size,
                after,
                content,
            } => {
                let mut json_padded = Map::new();
                if *before != 0 {
                    json_padded.insert("before".to_string(), json!(before));
                }
                if *min_size != 0 {
                    json_padded.insert("min_size".to_string(), json!(min_size));
                }
                if *after != 0 {
                    json_padded.insert("after".to_string(), json!(after));
                }
                json_padded.insert("type".to_string(), content.export(format));
                json!(json_padded)
            },
            ToolboxIdlTypeFlat::Const { literal } => {
                if format.can_skip_type_kind_key {
                    return json!(literal);
                }
                json!({
                    "kind": "const",
                    "value": literal.to_string(),
                })
            },
            ToolboxIdlTypeFlat::Primitive { primitive } => {
                if format.use_camel_case_type_primitive_names
                    && primitive == &ToolboxIdlTypePrimitive::Pubkey
                {
                    return json!("publicKey");
                }
                json!(primitive.as_str())
            },
        }
    }
}

impl ToolboxIdlTypeFlatEnumVariant {
    pub fn export(&self, index: usize, format: &ToolboxIdlFormat) -> Value {
        let index_code = u64::try_from(index).unwrap();
        if format.can_shortcut_enum_variant_to_string_if_no_fields
            && self.docs.is_none()
            && self.code == index_code
            && self.fields.len() == 0
        {
            return json!(self.name);
        }
        let mut json_variant = Map::new();
        json_variant.insert("name".to_string(), json!(self.name));
        if let Some(variant_docs) = &self.docs {
            json_variant.insert("docs".to_string(), json!(variant_docs));
        }
        if self.code != index_code {
            json_variant.insert("code".to_string(), json!(self.code));
        }
        if self.fields.len() > 0 {
            json_variant
                .insert("fields".to_string(), self.fields.export(format));
        }
        json!(json_variant)
    }
}

impl ToolboxIdlTypeFlatFields {
    pub fn export(&self, format: &ToolboxIdlFormat) -> Value {
        match self {
            ToolboxIdlTypeFlatFields::Named(fields) => {
                let mut json_fields = vec![];
                for field in fields {
                    json_fields.push(field.export(format));
                }
                json!(json_fields)
            },
            ToolboxIdlTypeFlatFields::Unnamed(fields) => {
                let mut json_fields = vec![];
                for field in fields {
                    json_fields.push(field.export(format));
                }
                json!(json_fields)
            },
        }
    }
}

impl ToolboxIdlTypeFlatFieldNamed {
    pub fn export(&self, format: &ToolboxIdlFormat) -> Value {
        let mut json_field = Map::new();
        json_field.insert("name".to_string(), self.export_name(format));
        json_field.insert("type".to_string(), self.content.export(format));
        if let Some(docs) = &self.docs {
            json_field.insert("docs".to_string(), json!(docs));
        }
        json!(json_field)
    }

    fn export_name(&self, format: &ToolboxIdlFormat) -> Value {
        if format.use_camel_case_type_fields_names {
            json!(idl_convert_to_camel_case(&self.name))
        } else {
            json!(self.name)
        }
    }
}

impl ToolboxIdlTypeFlatFieldUnnamed {
    pub fn export(&self, format: &ToolboxIdlFormat) -> Value {
        if let Some(docs) = &self.docs {
            return json!({
                "docs": docs,
                "type": self.content.export(format)
            });
        }
        if format.can_skip_unnamed_field_type_object_wrap {
            return self.content.export(format);
        }
        json!({ "type": self.content.export(format) })
    }
}
