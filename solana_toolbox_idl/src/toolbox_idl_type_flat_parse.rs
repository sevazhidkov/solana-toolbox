use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;
use crate::toolbox_idl_type_primitive::ToolboxIdlTypePrimitive;
use crate::toolbox_idl_utils::idl_convert_to_type_name;
use crate::toolbox_idl_utils::idl_convert_to_value_name;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_iter_get_scoped_values;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_object;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_u64_or_else;
use crate::toolbox_idl_utils::idl_str_to_u64_or_else;
use crate::toolbox_idl_utils::idl_value_as_object_get_key;
use crate::toolbox_idl_utils::idl_value_as_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_value_as_str_or_object_with_name_as_str_or_else;

impl ToolboxIdlTypeFlat {
    pub fn try_parse_value(
        idl_value: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFlat, ToolboxIdlError> {
        if let Some(idl_object) = idl_value.as_object() {
            return ToolboxIdlTypeFlat::try_parse_object(
                idl_object,
                breadcrumbs,
            );
        }
        if let Some(idl_array) = idl_value.as_array() {
            return ToolboxIdlTypeFlat::try_parse_array(idl_array, breadcrumbs);
        }
        if let Some(idl_str) = idl_value.as_str() {
            return ToolboxIdlTypeFlat::try_parse_str(idl_str, breadcrumbs);
        }
        if let Some(idl_u64) = idl_value.as_u64() {
            return ToolboxIdlTypeFlat::try_parse_u64(idl_u64, breadcrumbs);
        }
        idl_err(
            "Expected type value: object, array, string or number",
            &breadcrumbs.as_idl("def"),
        )
    }

    pub fn try_parse_object(
        idl_object: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFlat, ToolboxIdlError> {
        if let Some(idl_type) = idl_object.get("type") {
            return ToolboxIdlTypeFlat::try_parse_value(idl_type, breadcrumbs);
        }
        if let Some(idl_defined) = idl_object.get("defined") {
            return ToolboxIdlTypeFlat::try_parse_defined(
                idl_defined,
                breadcrumbs,
            );
        }
        if let Some(idl_generic_symbol) =
            idl_object_get_key_as_str(idl_object, "generic")
        {
            return ToolboxIdlTypeFlat::try_parse_generic_symbol(
                idl_generic_symbol,
                breadcrumbs,
            );
        }
        if let Some(idl_option) = idl_object.get("option") {
            return ToolboxIdlTypeFlat::try_parse_option(
                idl_option,
                1,
                breadcrumbs,
            );
        }
        if let Some(idl_option) = idl_object.get("option32") {
            return ToolboxIdlTypeFlat::try_parse_option(
                idl_option,
                4,
                breadcrumbs,
            );
        }
        if let Some(idl_vec) = idl_object.get("vec") {
            return ToolboxIdlTypeFlat::try_parse_vec(idl_vec, breadcrumbs);
        }
        if let Some(idl_array) =
            idl_object_get_key_as_array(idl_object, "array")
        {
            return ToolboxIdlTypeFlat::try_parse_array(idl_array, breadcrumbs);
        }
        if let Some(idl_struct_fields) =
            idl_object_get_key_as_array(idl_object, "fields")
        {
            return ToolboxIdlTypeFlat::try_parse_struct_fields(
                idl_struct_fields,
                breadcrumbs,
            );
        }
        if let Some(idl_enum_variants) =
            idl_object_get_key_as_array(idl_object, "variants")
        {
            return ToolboxIdlTypeFlat::try_parse_enum_variants(
                idl_enum_variants,
                breadcrumbs,
            );
        }
        if let Some(idl_padded) =
            idl_object_get_key_as_object(idl_object, "padded")
        {
            return ToolboxIdlTypeFlat::try_parse_padded(
                idl_padded,
                breadcrumbs,
            );
        }
        if let Some(idl_value_literal) =
            idl_object_get_key_as_str(idl_object, "value")
        {
            return ToolboxIdlTypeFlat::try_parse_value_literal(
                idl_value_literal,
                breadcrumbs,
            );
        }
        idl_err(
            "Missing type object key: defined/generic/option/array/vec/fields/variants/padded/value",
            &breadcrumbs.as_idl("def(object)"),
        )
    }

    fn try_parse_array(
        idl_array: &[Value],
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFlat, ToolboxIdlError> {
        if idl_array.len() == 1 {
            return ToolboxIdlTypeFlat::try_parse_vec(
                &idl_array[0],
                breadcrumbs,
            );
        }
        if idl_array.len() == 2 {
            return Ok(ToolboxIdlTypeFlat::Array {
                items: Box::new(ToolboxIdlTypeFlat::try_parse_value(
                    &idl_array[0],
                    &breadcrumbs.with_idl("items"),
                )?),
                length: Box::new(ToolboxIdlTypeFlat::try_parse_value(
                    &idl_array[1],
                    &breadcrumbs.with_idl("length"),
                )?),
            });
        }
        idl_err(
            "Array must be of either [{type}] or [{type}, {length}] format",
            &breadcrumbs.as_idl("def(array)"),
        )
    }

    fn try_parse_str(
        idl_str: &str,
        _breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFlat, ToolboxIdlError> {
        if idl_str == "bytes" {
            return Ok(ToolboxIdlTypeFlat::Vec {
                items: Box::new(ToolboxIdlTypeFlat::Primitive {
                    primitive: ToolboxIdlTypePrimitive::U8,
                }),
            });
        }
        if idl_str == "publicKey" {
            return Ok(ToolboxIdlTypeFlat::Primitive {
                primitive: ToolboxIdlTypePrimitive::PublicKey,
            });
        }
        Ok(match ToolboxIdlTypePrimitive::try_parse(idl_str) {
            Some(primitive) => ToolboxIdlTypeFlat::Primitive { primitive },
            None => ToolboxIdlTypeFlat::Defined {
                name: idl_str.to_string(),
                generics: vec![],
            },
        })
    }

    fn try_parse_u64(
        idl_u64: u64,
        _breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFlat, ToolboxIdlError> {
        Ok(ToolboxIdlTypeFlat::Const { literal: idl_u64 })
    }

    fn try_parse_defined(
        idl_defined: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFlat, ToolboxIdlError> {
        let defined_name = idl_convert_to_type_name(
            idl_value_as_str_or_object_with_name_as_str_or_else(
                idl_defined,
                &breadcrumbs.as_idl("defined"),
            )?,
        );
        let mut defined_generics = vec![];
        if let Some(idl_defined_generics) =
            idl_value_as_object_get_key_as_array(idl_defined, "generics")
        {
            for (_, idl_defined_generic, breadcrumbs) in
                idl_iter_get_scoped_values(idl_defined_generics, breadcrumbs)?
            {
                defined_generics.push(ToolboxIdlTypeFlat::try_parse_value(
                    idl_defined_generic,
                    &breadcrumbs,
                )?);
            }
        }
        Ok(ToolboxIdlTypeFlat::Defined {
            name: defined_name,
            generics: defined_generics,
        })
    }

    fn try_parse_generic_symbol(
        idl_generic_symbol: &str,
        _breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFlat, ToolboxIdlError> {
        Ok(ToolboxIdlTypeFlat::Generic {
            symbol: idl_generic_symbol.to_string(),
        })
    }

    fn try_parse_value_literal(
        idl_value_literal: &str,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFlat, ToolboxIdlError> {
        Ok(ToolboxIdlTypeFlat::Const {
            literal: idl_str_to_u64_or_else(
                idl_value_literal,
                &breadcrumbs.idl(),
            )?,
        })
    }

    fn try_parse_option(
        idl_option: &Value,
        idl_option_prefix_bytes: u8,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFlat, ToolboxIdlError> {
        Ok(ToolboxIdlTypeFlat::Option {
            prefix_bytes: idl_option_prefix_bytes,
            content: Box::new(ToolboxIdlTypeFlat::try_parse_value(
                idl_option,
                &breadcrumbs.with_idl("option"),
            )?),
        })
    }

    fn try_parse_vec(
        idl_vec: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFlat, ToolboxIdlError> {
        Ok(ToolboxIdlTypeFlat::Vec {
            items: Box::new(ToolboxIdlTypeFlat::try_parse_value(
                idl_vec,
                &breadcrumbs.with_idl("vec"),
            )?),
        })
    }

    fn try_parse_struct_fields(
        idl_struct_fields: &[Value],
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFlat, ToolboxIdlError> {
        Ok(ToolboxIdlTypeFlat::Struct {
            fields: ToolboxIdlTypeFlatFields::try_parse(
                idl_struct_fields,
                breadcrumbs,
            )?,
        })
    }

    fn try_parse_enum_variants(
        idl_enum_variants: &[Value],
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFlat, ToolboxIdlError> {
        let mut enum_variants = vec![];
        for (_, idl_enum_variant, breadcrumbs) in
            idl_iter_get_scoped_values(idl_enum_variants, breadcrumbs)?
        {
            let enum_variant_name = idl_convert_to_type_name(
                idl_value_as_str_or_object_with_name_as_str_or_else(
                    idl_enum_variant,
                    &breadcrumbs.idl(),
                )?,
            );
            let enum_variant_fields = if let Some(idl_enum_variant_fields) =
                idl_value_as_object_get_key_as_array(idl_enum_variant, "fields")
            {
                ToolboxIdlTypeFlatFields::try_parse(
                    idl_enum_variant_fields,
                    &breadcrumbs.with_idl("fields"),
                )?
            } else {
                ToolboxIdlTypeFlatFields::None
            };
            enum_variants.push((enum_variant_name, enum_variant_fields));
        }
        Ok(ToolboxIdlTypeFlat::Enum {
            variants: enum_variants,
        })
    }

    fn try_parse_padded(
        idl_padded: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFlat, ToolboxIdlError> {
        let idl_padded_size = idl_object_get_key_as_u64_or_else(
            idl_padded,
            "size",
            &breadcrumbs.as_idl("size"),
        )?;
        Ok(ToolboxIdlTypeFlat::Padded {
            size_bytes: idl_padded_size,
            content: Box::new(ToolboxIdlTypeFlat::try_parse_object(
                idl_padded,
                &breadcrumbs.with_idl("padded"),
            )?),
        })
    }
}

impl ToolboxIdlTypeFlatFields {
    pub fn try_parse(
        idl_fields: &[Value],
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFlatFields, ToolboxIdlError> {
        if idl_fields.is_empty() {
            return Ok(ToolboxIdlTypeFlatFields::None);
        }
        let mut fields_named = false;
        let mut fields_info = vec![];
        for (idl_field_index, idl_field, breadcrumbs) in
            idl_iter_get_scoped_values(idl_fields, breadcrumbs)?
        {
            let field_name = idl_value_as_object_get_key(idl_field, "name")
                .and_then(|name| name.as_str())
                .map(idl_convert_to_value_name);
            if field_name.is_some() {
                fields_named = true;
            }
            let field_name_or_index =
                field_name.unwrap_or(format!("{}", idl_field_index));
            let field_type_flat = ToolboxIdlTypeFlat::try_parse_value(
                idl_field,
                &breadcrumbs.with_idl(&field_name_or_index),
            )?;
            fields_info.push((field_name_or_index, field_type_flat));
        }
        if !fields_named {
            let mut fields = vec![];
            for (_, field_type) in fields_info {
                fields.push(field_type);
            }
            Ok(ToolboxIdlTypeFlatFields::Unamed(fields))
        } else {
            Ok(ToolboxIdlTypeFlatFields::Named(fields_info))
        }
    }
}
