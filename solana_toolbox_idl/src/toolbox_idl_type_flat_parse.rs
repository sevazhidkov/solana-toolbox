use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatEnumVariant;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFieldNamed;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFieldUnnamed;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;
use crate::toolbox_idl_type_prefix::ToolboxIdlTypePrefix;
use crate::toolbox_idl_type_primitive::ToolboxIdlTypePrimitive;
use crate::toolbox_idl_utils::idl_convert_to_snake_case;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_object;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_u64;
use crate::toolbox_idl_utils::idl_value_as_object_get_key;
use crate::toolbox_idl_utils::idl_value_as_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_value_as_object_get_key_as_u64;
use crate::toolbox_idl_utils::idl_value_as_str_or_object_with_key_as_str_or_else;

impl ToolboxIdlTypeFlat {
    pub fn try_parse_object_is_possible(
        idl_type_object: &Map<String, Value>,
    ) -> bool {
        if idl_type_object.contains_key("type")
            || idl_type_object.contains_key("defined")
            || idl_type_object.contains_key("generic")
            || idl_type_object.contains_key("option")
            || idl_type_object.contains_key("option8")
            || idl_type_object.contains_key("option16")
            || idl_type_object.contains_key("option32")
            || idl_type_object.contains_key("option64")
            || idl_type_object.contains_key("vec")
            || idl_type_object.contains_key("vec8")
            || idl_type_object.contains_key("vec16")
            || idl_type_object.contains_key("vec32")
            || idl_type_object.contains_key("vec64")
            || idl_type_object.contains_key("array")
            || idl_type_object.contains_key("fields")
            || idl_type_object.contains_key("variants")
            || idl_type_object.contains_key("variants8")
            || idl_type_object.contains_key("variants16")
            || idl_type_object.contains_key("variants32")
            || idl_type_object.contains_key("variants64")
            || idl_type_object.contains_key("padded")
        {
            return true;
        }
        false
    }

    pub fn try_parse(idl_type: &Value) -> Result<ToolboxIdlTypeFlat> {
        if let Some(idl_type_object) = idl_type.as_object() {
            return ToolboxIdlTypeFlat::try_parse_object(idl_type_object);
        }
        if let Some(idl_type_array) = idl_type.as_array() {
            return ToolboxIdlTypeFlat::try_parse_array(idl_type_array);
        }
        if let Some(idl_type_str) = idl_type.as_str() {
            return ToolboxIdlTypeFlat::try_parse_str(idl_type_str);
        }
        if let Some(idl_type_u64) = idl_type.as_u64() {
            return ToolboxIdlTypeFlat::try_parse_u64(idl_type_u64);
        }
        Err(anyhow!(
            "Could not parse type value, expected: object, array, string or number",
        ))
    }

    pub fn try_parse_object(
        idl_type_object: &Map<String, Value>,
    ) -> Result<ToolboxIdlTypeFlat> {
        if let Some(idl_type) = idl_type_object.get("type") {
            return ToolboxIdlTypeFlat::try_parse(idl_type);
        }
        if let Some(idl_defined) = idl_type_object.get("defined") {
            return ToolboxIdlTypeFlat::try_parse_defined(idl_defined)
                .context("Defined");
        }
        if let Some(idl_generic_symbol) =
            idl_object_get_key_as_str(idl_type_object, "generic")
        {
            return ToolboxIdlTypeFlat::try_parse_generic(idl_generic_symbol)
                .context("Generic");
        }
        if let Some(idl_option) = idl_type_object.get("option") {
            return ToolboxIdlTypeFlat::try_parse_option(
                ToolboxIdlTypePrefix::U8,
                idl_option,
            )
            .context("Option");
        }
        if let Some(idl_option) = idl_type_object.get("option8") {
            return ToolboxIdlTypeFlat::try_parse_option(
                ToolboxIdlTypePrefix::U8,
                idl_option,
            )
            .context("Option8");
        }
        if let Some(idl_option) = idl_type_object.get("option16") {
            return ToolboxIdlTypeFlat::try_parse_option(
                ToolboxIdlTypePrefix::U16,
                idl_option,
            )
            .context("Option16");
        }
        if let Some(idl_option) = idl_type_object.get("option32") {
            return ToolboxIdlTypeFlat::try_parse_option(
                ToolboxIdlTypePrefix::U32,
                idl_option,
            )
            .context("Option32");
        }
        if let Some(idl_option) = idl_type_object.get("option64") {
            return ToolboxIdlTypeFlat::try_parse_option(
                ToolboxIdlTypePrefix::U64,
                idl_option,
            )
            .context("Option64");
        }
        if let Some(idl_vec) = idl_type_object.get("vec") {
            return ToolboxIdlTypeFlat::try_parse_vec(
                ToolboxIdlTypePrefix::U32,
                idl_vec,
            )
            .context("Vec");
        }
        if let Some(idl_vec) = idl_type_object.get("vec8") {
            return ToolboxIdlTypeFlat::try_parse_vec(
                ToolboxIdlTypePrefix::U8,
                idl_vec,
            )
            .context("Vec8");
        }
        if let Some(idl_vec) = idl_type_object.get("vec16") {
            return ToolboxIdlTypeFlat::try_parse_vec(
                ToolboxIdlTypePrefix::U16,
                idl_vec,
            )
            .context("Vec16");
        }
        if let Some(idl_vec) = idl_type_object.get("vec32") {
            return ToolboxIdlTypeFlat::try_parse_vec(
                ToolboxIdlTypePrefix::U32,
                idl_vec,
            )
            .context("Vec32");
        }
        if let Some(idl_vec) = idl_type_object.get("vec64") {
            return ToolboxIdlTypeFlat::try_parse_vec(
                ToolboxIdlTypePrefix::U64,
                idl_vec,
            )
            .context("Vec64");
        }
        if let Some(idl_array) =
            idl_object_get_key_as_array(idl_type_object, "array")
        {
            return ToolboxIdlTypeFlat::try_parse_array(idl_array)
                .context("Array");
        }
        if let Some(idl_struct_fields) =
            idl_object_get_key_as_array(idl_type_object, "fields")
        {
            return ToolboxIdlTypeFlat::try_parse_struct(idl_struct_fields)
                .context("Struct Fields");
        }
        if let Some(idl_enum_variants) =
            idl_object_get_key_as_array(idl_type_object, "variants")
        {
            return ToolboxIdlTypeFlat::try_parse_enum(
                ToolboxIdlTypePrefix::U8,
                idl_enum_variants,
            )
            .context("Enum Variants");
        }
        if let Some(idl_enum_variants) =
            idl_object_get_key_as_array(idl_type_object, "variants8")
        {
            return ToolboxIdlTypeFlat::try_parse_enum(
                ToolboxIdlTypePrefix::U8,
                idl_enum_variants,
            )
            .context("Enum Variants8");
        }
        if let Some(idl_enum_variants) =
            idl_object_get_key_as_array(idl_type_object, "variants16")
        {
            return ToolboxIdlTypeFlat::try_parse_enum(
                ToolboxIdlTypePrefix::U16,
                idl_enum_variants,
            )
            .context("Enum Variants16");
        }
        if let Some(idl_enum_variants) =
            idl_object_get_key_as_array(idl_type_object, "variants32")
        {
            return ToolboxIdlTypeFlat::try_parse_enum(
                ToolboxIdlTypePrefix::U32,
                idl_enum_variants,
            )
            .context("Enum Variants32");
        }
        if let Some(idl_enum_variants) =
            idl_object_get_key_as_array(idl_type_object, "variants64")
        {
            return ToolboxIdlTypeFlat::try_parse_enum(
                ToolboxIdlTypePrefix::U64,
                idl_enum_variants,
            )
            .context("Enum Variants64");
        }
        if let Some(idl_padded) =
            idl_object_get_key_as_object(idl_type_object, "padded")
        {
            return ToolboxIdlTypeFlat::try_parse_padded(idl_padded)
                .context("Padded");
        }
        if let Some(idl_value_literal) =
            idl_object_get_key_as_str(idl_type_object, "value")
        {
            return ToolboxIdlTypeFlat::try_parse_const_value(
                idl_value_literal,
            )
            .context("Const Value");
        }
        Err(anyhow!(
            "Could not parse type object: Missing type object key: {:?}",
            vec![
                "defined", "generic", "option", "array", "vec", "fields",
                "variants", "padded", "value"
            ]
        ))
    }

    fn try_parse_array(idl_type_array: &[Value]) -> Result<ToolboxIdlTypeFlat> {
        if idl_type_array.len() == 1 {
            return ToolboxIdlTypeFlat::try_parse_vec(
                ToolboxIdlTypePrefix::U32,
                &idl_type_array[0],
            );
        }
        if idl_type_array.len() == 2 {
            return Ok(ToolboxIdlTypeFlat::Array {
                items: Box::new(
                    ToolboxIdlTypeFlat::try_parse(&idl_type_array[0])
                        .context("Items")?,
                ),
                length: Box::new(
                    ToolboxIdlTypeFlat::try_parse(&idl_type_array[1])
                        .context("Length")?,
                ),
            });
        }
        Err(anyhow!(
            "Could not parse array type, expected either [type] or [type, length] format",
        ))
    }

    fn try_parse_str(idl_type_str: &str) -> Result<ToolboxIdlTypeFlat> {
        Ok(match idl_type_str {
            "bytes" => ToolboxIdlTypeFlat::Vec {
                prefix: ToolboxIdlTypePrefix::U32,
                items: Box::new(ToolboxIdlTypePrimitive::U8.into()),
            },
            "string" => ToolboxIdlTypeFlat::String {
                prefix: ToolboxIdlTypePrefix::U32,
            },
            "string8" => ToolboxIdlTypeFlat::String {
                prefix: ToolboxIdlTypePrefix::U8,
            },
            "string16" => ToolboxIdlTypeFlat::String {
                prefix: ToolboxIdlTypePrefix::U16,
            },
            "string32" => ToolboxIdlTypeFlat::String {
                prefix: ToolboxIdlTypePrefix::U32,
            },
            "string64" => ToolboxIdlTypeFlat::String {
                prefix: ToolboxIdlTypePrefix::U64,
            },
            "publicKey" => ToolboxIdlTypePrimitive::Pubkey.into(),
            _ => match ToolboxIdlTypePrimitive::try_parse(idl_type_str) {
                Some(primitive) => primitive.into(),
                None => ToolboxIdlTypeFlat::Defined {
                    name: idl_type_str.to_string(),
                    generics: vec![],
                },
            },
        })
    }

    fn try_parse_u64(idl_type_u64: u64) -> Result<ToolboxIdlTypeFlat> {
        Ok(ToolboxIdlTypeFlat::Const {
            literal: idl_type_u64,
        })
    }

    fn try_parse_defined(idl_defined: &Value) -> Result<ToolboxIdlTypeFlat> {
        let defined_name = idl_value_as_str_or_object_with_key_as_str_or_else(
            idl_defined,
            "name",
        )
        .context("Parse Defined Name")?
        .to_string();
        let mut defined_generics = vec![];
        if let Some(idl_defined_generics) =
            idl_value_as_object_get_key_as_array(idl_defined, "generics")
        {
            for (index, idl_defined_generic) in
                idl_defined_generics.iter().enumerate()
            {
                defined_generics.push(
                    ToolboxIdlTypeFlat::try_parse(idl_defined_generic)
                        .with_context(|| {
                            format!("Parse Defined's Generic Type: {}", index)
                        })?,
                );
            }
        }
        Ok(ToolboxIdlTypeFlat::Defined {
            name: defined_name,
            generics: defined_generics,
        })
    }

    fn try_parse_generic(
        idl_generic_symbol: &str,
    ) -> Result<ToolboxIdlTypeFlat> {
        Ok(ToolboxIdlTypeFlat::Generic {
            symbol: idl_generic_symbol.to_string(),
        })
    }

    fn try_parse_option(
        idl_option_prefix: ToolboxIdlTypePrefix,
        idl_option_content: &Value,
    ) -> Result<ToolboxIdlTypeFlat> {
        Ok(ToolboxIdlTypeFlat::Option {
            prefix: idl_option_prefix,
            content: Box::new(ToolboxIdlTypeFlat::try_parse(
                idl_option_content,
            )?),
        })
    }

    fn try_parse_vec(
        idl_vec_prefix: ToolboxIdlTypePrefix,
        idl_vec_items: &Value,
    ) -> Result<ToolboxIdlTypeFlat> {
        Ok(ToolboxIdlTypeFlat::Vec {
            prefix: idl_vec_prefix,
            items: Box::new(ToolboxIdlTypeFlat::try_parse(idl_vec_items)?),
        })
    }

    fn try_parse_struct(
        idl_struct_fields: &[Value],
    ) -> Result<ToolboxIdlTypeFlat> {
        Ok(ToolboxIdlTypeFlat::Struct {
            fields: ToolboxIdlTypeFlatFields::try_parse(idl_struct_fields)?,
        })
    }

    fn try_parse_enum(
        idl_enum_prefix: ToolboxIdlTypePrefix,
        idl_enum_variants: &[Value],
    ) -> Result<ToolboxIdlTypeFlat> {
        let mut variants = vec![];
        for (index, idl_enum_variant) in idl_enum_variants.iter().enumerate() {
            variants.push(ToolboxIdlTypeFlat::try_parse_enum_variant(
                index,
                idl_enum_variant,
            )?);
        }
        Ok(ToolboxIdlTypeFlat::Enum {
            prefix: idl_enum_prefix,
            variants,
        })
    }

    fn try_parse_enum_variant(
        idl_enum_variant_index: usize,
        idl_enum_variant: &Value,
    ) -> Result<ToolboxIdlTypeFlatEnumVariant> {
        let name = idl_value_as_str_or_object_with_key_as_str_or_else(
            idl_enum_variant,
            "name",
        )
        .with_context(|| {
            format!("Parse Enum Variant Name: {}", idl_enum_variant_index)
        })?
        .to_string();
        let docs =
            idl_value_as_object_get_key(idl_enum_variant, "docs").cloned();
        let code = idl_value_as_object_get_key_as_u64(idl_enum_variant, "code")
            .unwrap_or(u64::try_from(idl_enum_variant_index)?);
        let fields = if let Some(idl_enum_variant_fields) =
            idl_value_as_object_get_key_as_array(idl_enum_variant, "fields")
        {
            ToolboxIdlTypeFlatFields::try_parse(idl_enum_variant_fields)
                .with_context(|| format!("Parse Enum Variant Type: {}", name))?
        } else {
            ToolboxIdlTypeFlatFields::Unnamed(vec![])
        };
        Ok(ToolboxIdlTypeFlatEnumVariant {
            name,
            docs,
            code,
            fields,
        })
    }

    fn try_parse_padded(
        idl_padded: &Map<String, Value>,
    ) -> Result<ToolboxIdlTypeFlat> {
        let before = usize::try_from(
            idl_object_get_key_as_u64(idl_padded, "before").unwrap_or(0),
        )?;
        let min_size = usize::try_from(
            idl_object_get_key_as_u64(idl_padded, "min_size").unwrap_or(0),
        )?;
        let after = usize::try_from(
            idl_object_get_key_as_u64(idl_padded, "after").unwrap_or(0),
        )?;
        Ok(ToolboxIdlTypeFlat::Padded {
            before,
            min_size,
            after,
            content: Box::new(ToolboxIdlTypeFlat::try_parse_object(
                idl_padded,
            )?),
        })
    }

    fn try_parse_const_value(
        idl_value_literal: &str,
    ) -> Result<ToolboxIdlTypeFlat> {
        Ok(ToolboxIdlTypeFlat::Const {
            literal: idl_value_literal.parse().map_err(|error| {
                anyhow!("Parse int literal error: {}", error)
            })?,
        })
    }
}

impl ToolboxIdlTypeFlatFields {
    pub fn try_parse(idl_fields: &[Value]) -> Result<ToolboxIdlTypeFlatFields> {
        if idl_fields.is_empty() {
            return Ok(ToolboxIdlTypeFlatFields::nothing());
        }
        let mut fields_named = false;
        let mut fields_info = vec![];
        for (index, idl_field) in idl_fields.iter().enumerate() {
            let field_name = idl_value_as_object_get_key(idl_field, "name")
                .and_then(|name| name.as_str())
                .map(idl_convert_to_snake_case);
            if field_name.is_some() {
                fields_named = true;
            }
            let field_name_or_index =
                field_name.unwrap_or_else(|| format!("{}", index));
            let field_docs =
                idl_value_as_object_get_key(idl_field, "docs").cloned();
            let field_type_flat = ToolboxIdlTypeFlat::try_parse(idl_field)
                .with_context(|| {
                    format!("Parse Field Type: {}", field_name_or_index)
                })?;
            fields_info.push(ToolboxIdlTypeFlatFieldNamed {
                name: field_name_or_index,
                docs: field_docs,
                content: field_type_flat,
            });
        }
        if !fields_named {
            let mut fields = vec![];
            for field in fields_info {
                fields.push(ToolboxIdlTypeFlatFieldUnnamed {
                    docs: field.docs,
                    content: field.content,
                });
            }
            Ok(ToolboxIdlTypeFlatFields::Unnamed(fields))
        } else {
            Ok(ToolboxIdlTypeFlatFields::Named(fields_info))
        }
    }
}
