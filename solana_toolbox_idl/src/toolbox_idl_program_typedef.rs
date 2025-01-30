use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_typedef_primitive::ToolboxIdlProgramTypedefPrimitive;
use crate::toolbox_idl_utils::idl_array_get_scoped_named_object_array_or_else;
use crate::toolbox_idl_utils::idl_array_get_scoped_object_array_or_else;
use crate::toolbox_idl_utils::idl_as_u128_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;
use crate::toolbox_idl_utils::idl_value_as_str_or_object_with_name_as_str_or_else;

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlProgramTypedef {
    Defined { name: String, generics: Vec<ToolboxIdlProgramTypedef> },
    Option { content_typedef: Box<ToolboxIdlProgramTypedef> },
    Vec { items_typedef: Box<ToolboxIdlProgramTypedef> },
    Array { length: u32, items_typedef: Box<ToolboxIdlProgramTypedef> },
    Struct { fields: Vec<(String, ToolboxIdlProgramTypedef)> },
    Enum { variants: Vec<(String, Vec<ToolboxIdlProgramTypedef>)> },
    Primitive(ToolboxIdlProgramTypedefPrimitive),
    Const { value: u64 }, // TODO - what kind of consts can be supported ?
    Generic { symbol: String },
}

impl ToolboxIdlProgramTypedef {
    pub(crate) fn try_parse(
        idl_typedef: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypedef, ToolboxIdlError> {
        if let Some(idl_typedef_object) = idl_typedef.as_object() {
            return ToolboxIdlProgramTypedef::try_parse_object(
                idl_typedef_object,
                breadcrumbs,
            );
        }
        if let Some(idl_typedef_array) = idl_typedef.as_array() {
            return ToolboxIdlProgramTypedef::try_parse_array(
                idl_typedef_array,
                breadcrumbs,
            );
        }
        if let Some(idl_typedef_str) = idl_typedef.as_str() {
            return ToolboxIdlProgramTypedef::try_parse_str(
                idl_typedef_str,
                breadcrumbs,
            );
        }
        if let Some(idl_typedef_u64) = idl_typedef.as_u64() {
            return ToolboxIdlProgramTypedef::try_parse_u64(
                idl_typedef_u64,
                breadcrumbs,
            );
        }
        idl_err(
            "Expected type object, array or string",
            &breadcrumbs.as_idl("typedef"),
        )
    }

    fn try_parse_object(
        idl_typedef_object: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypedef, ToolboxIdlError> {
        if let Some(idl_typedef_defined) = idl_typedef_object.get("defined") {
            return ToolboxIdlProgramTypedef::try_parse_defined(
                idl_typedef_defined,
                breadcrumbs,
            );
        }
        if let Some(idl_typedef_option) = idl_typedef_object.get("option") {
            return ToolboxIdlProgramTypedef::try_parse_option(
                idl_typedef_option,
                breadcrumbs,
            );
        }
        if let Some(idl_typedef_vec) = idl_typedef_object.get("vec") {
            return ToolboxIdlProgramTypedef::try_parse_vec(
                idl_typedef_vec,
                breadcrumbs,
            );
        }
        if let Some(idl_typedef_array) =
            idl_object_get_key_as_array(idl_typedef_object, "array")
        {
            return ToolboxIdlProgramTypedef::try_parse_array(
                idl_typedef_array,
                breadcrumbs,
            );
        }
        if let Some(idl_typedef_struct_fields) =
            idl_object_get_key_as_array(idl_typedef_object, "fields")
        {
            return ToolboxIdlProgramTypedef::try_parse_struct_fields(
                idl_typedef_struct_fields,
                breadcrumbs,
            );
        }
        if let Some(idl_typedef_enum_variants) =
            idl_object_get_key_as_array(idl_typedef_object, "variants")
        {
            return ToolboxIdlProgramTypedef::try_parse_enum_variants(
                idl_typedef_enum_variants,
                breadcrumbs,
            );
        }
        if let Some(idl_typedef_generic_symbol) =
            idl_object_get_key_as_str(idl_typedef_object, "generic")
        {
            return ToolboxIdlProgramTypedef::try_parse_generic_symbol(
                idl_typedef_generic_symbol,
                breadcrumbs,
            );
        }
        idl_err(
            "Missing type object key: defined/option/fields/variants/array/vec",
            &breadcrumbs.as_idl("typedef(object)"),
        )
    }

    fn try_parse_array(
        idl_typedef_array: &[Value],
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypedef, ToolboxIdlError> {
        if idl_typedef_array.len() == 1 {
            return ToolboxIdlProgramTypedef::try_parse_vec(
                &idl_typedef_array[0],
                breadcrumbs,
            );
        }
        if idl_typedef_array.len() == 2 {
            return Ok(ToolboxIdlProgramTypedef::Array {
                length: {
                    let context = &breadcrumbs.as_idl("array_length");
                    u32::try_from(idl_as_u128_or_else(
                        &idl_typedef_array[1],
                        context,
                    )?)
                    .map_err(|err| {
                        ToolboxIdlError::InvalidInteger {
                            conversion: err,
                            context: context.clone(),
                        }
                    })?
                },
                items_typedef: Box::new(ToolboxIdlProgramTypedef::try_parse(
                    &idl_typedef_array[0],
                    &breadcrumbs.with_idl("array"),
                )?),
            });
        }
        idl_err(
            "Array must be of either [{type}] or [{type}, {length}] format",
            &breadcrumbs.as_idl("typedef(array)"),
        )
    }

    fn try_parse_str(
        idl_typedef_str: &str,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypedef, ToolboxIdlError> {
        match ToolboxIdlProgramTypedefPrimitive::try_parse(idl_typedef_str) {
            Some(program_typedef_primitive) => Ok(
                ToolboxIdlProgramTypedef::Primitive(program_typedef_primitive),
            ),
            None => ToolboxIdlProgramTypedef::try_parse_defined(
                &Value::String(idl_typedef_str.to_string()),
                breadcrumbs,
            ),
        }
    }

    fn try_parse_u64(
        idl_typedef_u64: u64,
        _breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypedef, ToolboxIdlError> {
        Ok(ToolboxIdlProgramTypedef::Const { value: idl_typedef_u64 })
    }

    fn try_parse_defined(
        idl_typedef_defined: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypedef, ToolboxIdlError> {
        let program_typedef_defined_name =
            idl_value_as_str_or_object_with_name_as_str_or_else(
                idl_typedef_defined,
                &breadcrumbs.as_idl("defined"),
            )?;
        let mut program_typedef_defined_generics = vec![];
        if let Some(idl_typedef_defined_object) =
            idl_typedef_defined.as_object()
        {
            if let Some(idl_typedef_defined_generics) =
                idl_object_get_key_as_array(
                    idl_typedef_defined_object,
                    "generics",
                )
            {
                for (idl_typedef_defined_generic, breadcrumbs) in
                    idl_array_get_scoped_object_array_or_else(
                        idl_typedef_defined_generics,
                        &breadcrumbs.with_idl("generics"),
                    )?
                {
                    program_typedef_defined_generics.push(
                        ToolboxIdlProgramTypedef::try_parse_defined_generic(
                            idl_typedef_defined_generic,
                            &breadcrumbs,
                        )?,
                    );
                }
            }
        }
        Ok(ToolboxIdlProgramTypedef::Defined {
            name: program_typedef_defined_name.to_string(),
            generics: program_typedef_defined_generics,
        })
    }

    fn try_parse_defined_generic(
        idl_typedef_defined_generic: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypedef, ToolboxIdlError> {
        match idl_object_get_key_as_str_or_else(
            idl_typedef_defined_generic,
            "kind",
            &breadcrumbs.idl(),
        )? {
            "type" => ToolboxIdlProgramTypedef::try_parse(
                idl_object_get_key_or_else(
                    idl_typedef_defined_generic,
                    "type",
                    &breadcrumbs.idl(),
                )?,
                &breadcrumbs,
            ),
            "const" => Ok(ToolboxIdlProgramTypedef::Const {
                value: idl_object_get_key_as_str_or_else(
                    idl_typedef_defined_generic,
                    "value",
                    &breadcrumbs.idl(),
                )?
                .parse()
                .map_err(|err| {
                    ToolboxIdlError::InvalidConst {
                        parsing: err,
                        context: breadcrumbs.as_idl("value"),
                    }
                })?,
            }),
            _ => idl_err("Unknown generic kind", &breadcrumbs.idl()),
        }
    }

    fn try_parse_option(
        idl_typedef_option: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypedef, ToolboxIdlError> {
        Ok(ToolboxIdlProgramTypedef::Option {
            content_typedef: Box::new(ToolboxIdlProgramTypedef::try_parse(
                idl_typedef_option,
                &breadcrumbs.with_idl("option"),
            )?),
        })
    }

    fn try_parse_vec(
        idl_typedef_vec: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypedef, ToolboxIdlError> {
        Ok(ToolboxIdlProgramTypedef::Vec {
            items_typedef: Box::new(ToolboxIdlProgramTypedef::try_parse(
                idl_typedef_vec,
                &breadcrumbs.with_idl("vec"),
            )?),
        })
    }

    fn try_parse_struct_fields(
        idl_typedef_struct_fields: &[Value],
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypedef, ToolboxIdlError> {
        let mut program_typedef_struct_fields = vec![];
        for (
            idl_typedef_struct_field_name,
            idl_typedef_struct_field_object,
            breadcrumbs,
        ) in idl_array_get_scoped_named_object_array_or_else(
            idl_typedef_struct_fields,
            breadcrumbs,
        )? {
            let idl_typedef_struct_field_typedef = idl_object_get_key_or_else(
                idl_typedef_struct_field_object,
                "type",
                &breadcrumbs.idl(),
            )?;
            program_typedef_struct_fields.push((
                idl_typedef_struct_field_name.to_string(),
                ToolboxIdlProgramTypedef::try_parse(
                    idl_typedef_struct_field_typedef,
                    &breadcrumbs,
                )?,
            ));
        }
        Ok(ToolboxIdlProgramTypedef::Struct {
            fields: program_typedef_struct_fields,
        })
    }

    // TODO - support for enums with content fields ?
    fn try_parse_enum_variants(
        idl_typedef_enum_variants: &[Value],
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypedef, ToolboxIdlError> {
        let mut program_typedef_enum_variants = vec![];
        for (index, idl_typedef_variant) in
            idl_typedef_enum_variants.iter().enumerate()
        {
            let idl_typedef_variant_name =
                idl_value_as_str_or_object_with_name_as_str_or_else(
                    idl_typedef_variant,
                    &breadcrumbs.as_idl(&format!("variants[{}]", index)),
                )?;
            program_typedef_enum_variants.push((
                idl_typedef_variant_name.to_string(),
                vec![], // TODO - support variant fields
            ));
        }
        Ok(ToolboxIdlProgramTypedef::Enum {
            variants: program_typedef_enum_variants,
        })
    }

    fn try_parse_generic_symbol(
        idl_typedef_generic_symbol: &str,
        _breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypedef, ToolboxIdlError> {
        Ok(ToolboxIdlProgramTypedef::Generic {
            symbol: idl_typedef_generic_symbol.to_string(),
        })
    }
}
