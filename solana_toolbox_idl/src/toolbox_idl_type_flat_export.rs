use std::ops::Deref;

use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_format::ToolboxIdlFormat;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
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
                };
                json!({ key: content.export(format) })
            },
            ToolboxIdlTypeFlat::Vec { prefix, items } => {
                let key = match prefix {
                    ToolboxIdlTypePrefix::U8 => "vec8",
                    ToolboxIdlTypePrefix::U16 => "vec16",
                    ToolboxIdlTypePrefix::U32 => "vec",
                    ToolboxIdlTypePrefix::U64 => "vec64",
                };
                if key != "vec" {
                    return json!({ key: items.export(format) });
                }
                if format.can_shortcut_type_vec_notation {
                    return json!([items.export(format)]);
                }
                if items.deref().eq(&ToolboxIdlTypeFlat::Primitive {
                    primitive: ToolboxIdlTypePrimitive::U8,
                }) {
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
                    let index_code = u64::try_from(index).unwrap();
                    if format.can_shortcut_enum_variant_to_string_if_no_fields
                        && variant.docs.is_none()
                        && variant.code == index_code
                        && variant.fields == ToolboxIdlTypeFlatFields::None
                    {
                        json_variants.push(json!(variant.name));
                        continue;
                    }
                    let mut json_variant = Map::new();
                    json_variant
                        .insert("name".to_string(), json!(variant.name));
                    if let Some(variant_docs) = &variant.docs {
                        json_variant
                            .insert("docs".to_string(), json!(variant_docs));
                    }
                    if variant.code != index_code {
                        json_variant
                            .insert("code".to_string(), json!(variant.code));
                    }
                    if variant.fields != ToolboxIdlTypeFlatFields::None {
                        json_variant.insert(
                            "fields".to_string(),
                            variant.fields.export(format),
                        );
                    }
                    json_variants.push(json!(json_variant));
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
                };
                json_enum.insert(key.to_string(), json!(json_variants));
                json!(json_enum)
            },
            ToolboxIdlTypeFlat::Padded {
                size_bytes,
                content,
            } => {
                json!({
                    "padded": {
                        "size": size_bytes,
                        "type": content.export(format)
                    }
                })
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
                    && primitive == &ToolboxIdlTypePrimitive::PublicKey
                {
                    return json!("publicKey");
                }
                json!(primitive.as_str())
            },
        }
    }
}

impl ToolboxIdlTypeFlatFields {
    pub fn export(&self, format: &ToolboxIdlFormat) -> Value {
        match self {
            ToolboxIdlTypeFlatFields::Named(fields) => {
                let mut json_fields = vec![];
                for field in fields {
                    let mut json_field = Map::new();
                    json_field.insert(
                        "name".to_string(),
                        ToolboxIdlTypeFlatFields::export_field_name(
                            &field.name,
                            format,
                        ),
                    );
                    json_field.insert(
                        "type".to_string(),
                        field.type_flat.export(format),
                    );
                    if let Some(field_docs) = &field.docs {
                        json_field
                            .insert("docs".to_string(), json!(field_docs));
                    }
                    json_fields.push(json_field);
                }
                json!(json_fields)
            },
            ToolboxIdlTypeFlatFields::Unnamed(fields) => {
                let mut json_fields = vec![];
                for field in fields {
                    if let Some(field_docs) = &field.docs {
                        json_fields.push(json!({
                            "docs": field_docs,
                            "type": field.type_flat.export(format)
                        }));
                    } else if format.can_skip_unamed_field_type_object_wrap {
                        json_fields.push(field.type_flat.export(format));
                    } else {
                        json_fields.push(json!({
                            "type": field.type_flat.export(format)
                        }));
                    }
                }
                json!(json_fields)
            },
            ToolboxIdlTypeFlatFields::None => {
                json!([])
            },
        }
    }

    fn export_field_name(field_name: &str, format: &ToolboxIdlFormat) -> Value {
        if format.use_camel_case_type_fields_names {
            json!(idl_convert_to_camel_case(field_name))
        } else {
            json!(field_name)
        }
    }
}
