use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_typedef_primitive::ToolboxIdlProgramTypedefPrimitiveKind;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_as_u128_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;
use crate::toolbox_idl_utils::idl_value_as_str_or_object_with_name_as_str_or_else;

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlProgramTypedef {
    Defined { name: String },
    Option { content: Box<ToolboxIdlProgramTypedef> },
    Vec { items: Box<ToolboxIdlProgramTypedef> },
    Array { length: u32, items: Box<ToolboxIdlProgramTypedef> },
    Struct { fields: Vec<(String, ToolboxIdlProgramTypedef)> },
    Enum { variants: Vec<String> },
    Primitive { kind: ToolboxIdlProgramTypedefPrimitiveKind },
}

impl ToolboxIdlProgramTypedef {
    pub(crate) fn try_parse(
        idl_typedef_value: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypedef, ToolboxIdlError> {
        if let Some(idl_typedef_object) = idl_typedef_value.as_object() {
            return ToolboxIdlProgramTypedef::try_parse_object(
                idl_typedef_object,
                breadcrumbs,
            );
        }
        if let Some(idl_typedef_array) = idl_typedef_value.as_array() {
            return ToolboxIdlProgramTypedef::try_parse_array(
                idl_typedef_array,
                breadcrumbs,
            );
        }
        if let Some(idl_typedef_str) = idl_typedef_value.as_str() {
            return ToolboxIdlProgramTypedef::try_parse_str(
                idl_typedef_str,
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
        if let Some(idl_typedef_fields) =
            idl_object_get_key_as_array(idl_typedef_object, "fields")
        {
            return ToolboxIdlProgramTypedef::try_parse_struct_fields(
                idl_typedef_fields,
                breadcrumbs,
            );
        }
        if let Some(idl_typedef_variants) =
            idl_object_get_key_as_array(idl_typedef_object, "variants")
        {
            return ToolboxIdlProgramTypedef::try_parse_enum_variants(
                idl_typedef_variants,
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
                items: Box::new(ToolboxIdlProgramTypedef::try_parse(
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
        Ok(
            match ToolboxIdlProgramTypedefPrimitiveKind::from_str(
                idl_typedef_str,
            ) {
                Some(program_typedef_primitive_kind) => {
                    ToolboxIdlProgramTypedef::Primitive {
                        kind: program_typedef_primitive_kind,
                    }
                },
                None => {
                    ToolboxIdlProgramTypedef::try_parse_defined(
                        &Value::String(idl_typedef_str.to_string()),
                        breadcrumbs,
                    )?
                },
            },
        )
    }

    fn try_parse_defined(
        idl_typedef_defined: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypedef, ToolboxIdlError> {
        let idl_typedef_name =
            idl_value_as_str_or_object_with_name_as_str_or_else(
                idl_typedef_defined,
                &breadcrumbs.as_idl("defined"),
            )?;
        Ok(ToolboxIdlProgramTypedef::Defined {
            name: idl_typedef_name.to_string(),
        })
    }

    fn try_parse_option(
        idl_typedef_option: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypedef, ToolboxIdlError> {
        Ok(ToolboxIdlProgramTypedef::Option {
            content: Box::new(ToolboxIdlProgramTypedef::try_parse(
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
            items: Box::new(ToolboxIdlProgramTypedef::try_parse(
                idl_typedef_vec,
                &breadcrumbs.with_idl("vec"),
            )?),
        })
    }

    fn try_parse_struct_fields(
        idl_typedef_fields: &[Value],
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypedef, ToolboxIdlError> {
        let mut program_typedef_struct_fields = vec![];
        for (index, idl_typedef_field) in idl_typedef_fields.iter().enumerate()
        {
            let context = &breadcrumbs.as_idl(&format!("fields[{}]", index));
            let idl_typedef_field_object =
                idl_as_object_or_else(idl_typedef_field, context)?;
            let idl_typedef_field_name = idl_object_get_key_as_str_or_else(
                idl_typedef_field_object,
                "name",
                context,
            )?;
            let breadcrumbs = &breadcrumbs.with_idl(idl_typedef_field_name);
            let idl_typedef_field_typedef_value = idl_object_get_key_or_else(
                idl_typedef_field_object,
                "type",
                &breadcrumbs.idl(),
            )?;
            program_typedef_struct_fields.push((
                idl_typedef_field_name.to_string(),
                ToolboxIdlProgramTypedef::try_parse(
                    idl_typedef_field_typedef_value,
                    breadcrumbs,
                )?,
            ));
        }
        Ok(ToolboxIdlProgramTypedef::Struct {
            fields: program_typedef_struct_fields,
        })
    }

    // TODO - support for enums with content ?
    fn try_parse_enum_variants(
        idl_typedef_variants: &[Value],
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramTypedef, ToolboxIdlError> {
        let mut program_typedef_enum_variants = vec![];
        for (index, idl_typedef_variant) in
            idl_typedef_variants.iter().enumerate()
        {
            let idl_typedef_variant_name =
                idl_value_as_str_or_object_with_name_as_str_or_else(
                    idl_typedef_variant,
                    &breadcrumbs.as_idl(&format!("variants[{}]", index)),
                )?;
            program_typedef_enum_variants
                .push(idl_typedef_variant_name.to_string());
        }
        Ok(ToolboxIdlProgramTypedef::Enum {
            variants: program_typedef_enum_variants,
        })
    }
}
