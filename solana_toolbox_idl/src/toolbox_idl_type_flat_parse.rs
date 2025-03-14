use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;
use crate::toolbox_idl_type_primitive::ToolboxIdlTypePrimitive;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_iter_get_scoped_values;
use crate::toolbox_idl_utils::idl_map_err_invalid_integer;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_str_to_usize_or_else;
use crate::toolbox_idl_utils::idl_value_as_object_get_key;
use crate::toolbox_idl_utils::idl_value_as_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_value_as_str_or_object_with_name_as_str_or_else;

impl ToolboxIdlTypeFlat {
    pub fn try_parse(
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

    fn try_parse_object(
        idl_object: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFlat, ToolboxIdlError> {
        if let Some(idl_type) = idl_object.get("type") {
            return ToolboxIdlTypeFlat::try_parse(idl_type, breadcrumbs);
        }
        if let Some(idl_defined) = idl_object.get("defined") {
            return ToolboxIdlTypeFlat::try_parse_defined(
                idl_defined,
                breadcrumbs,
            );
        }
        if let Some(idl_option) = idl_object.get("option") {
            return ToolboxIdlTypeFlat::try_parse_option(
                idl_option,
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
        if let Some(idl_generic_symbol) =
            idl_object_get_key_as_str(idl_object, "generic")
        {
            return ToolboxIdlTypeFlat::try_parse_generic_symbol(
                idl_generic_symbol,
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
            "Missing type object key: defined/option/fields/variants/array/vec/generic",
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
                items: Box::new(ToolboxIdlTypeFlat::try_parse(
                    &idl_array[0],
                    &breadcrumbs.with_idl("items"),
                )?),
                length: Box::new(ToolboxIdlTypeFlat::try_parse(
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
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFlat, ToolboxIdlError> {
        Ok(ToolboxIdlTypeFlat::Const {
            literal: idl_map_err_invalid_integer(
                usize::try_from(idl_u64),
                &breadcrumbs.idl(),
            )?,
        })
    }

    fn try_parse_defined(
        idl_defined: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFlat, ToolboxIdlError> {
        let defined_name = idl_value_as_str_or_object_with_name_as_str_or_else(
            idl_defined,
            &breadcrumbs.as_idl("defined"),
        )?;
        let mut defined_generics = vec![];
        if let Some(idl_defined_generics) =
            idl_value_as_object_get_key_as_array(idl_defined, "generics")
        {
            for (_, idl_defined_generic, breadcrumbs) in
                idl_iter_get_scoped_values(idl_defined_generics, breadcrumbs)?
            {
                defined_generics.push(ToolboxIdlTypeFlat::try_parse(
                    idl_defined_generic,
                    &breadcrumbs,
                )?);
            }
        }
        Ok(ToolboxIdlTypeFlat::Defined {
            name: defined_name.to_string(),
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
            literal: idl_str_to_usize_or_else(
                idl_value_literal,
                &breadcrumbs.idl(),
            )?,
        })
    }

    fn try_parse_option(
        idl_option: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFlat, ToolboxIdlError> {
        Ok(ToolboxIdlTypeFlat::Option {
            content: Box::new(ToolboxIdlTypeFlat::try_parse(
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
            items: Box::new(ToolboxIdlTypeFlat::try_parse(
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
            let enum_variant_name =
                idl_value_as_str_or_object_with_name_as_str_or_else(
                    idl_enum_variant,
                    &breadcrumbs.idl(),
                )?;
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
            enum_variants
                .push((enum_variant_name.to_string(), enum_variant_fields));
        }
        Ok(ToolboxIdlTypeFlat::Enum {
            variants: enum_variants,
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
                .map(|name| name.to_string());
            if field_name.is_some() {
                fields_named = true;
            }
            let field_name_or_index =
                field_name.unwrap_or(format!("{}", idl_field_index));
            let field_type_flat = ToolboxIdlTypeFlat::try_parse(
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
