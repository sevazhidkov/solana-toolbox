use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_def_primitive::ToolboxIdlProgramDefPrimitive;
use crate::toolbox_idl_utils::idl_array_get_scoped_named_object_array_or_else;
use crate::toolbox_idl_utils::idl_array_get_scoped_object_array_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_map_err_invalid_integer;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;
use crate::toolbox_idl_utils::idl_value_as_object_get_key;
use crate::toolbox_idl_utils::idl_value_as_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_value_as_str_or_object_with_name_as_str_or_else;

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlProgramDef {
    Defined {
        name: String,
        generics: Vec<ToolboxIdlProgramDef>,
    },
    Option {
        content: Box<ToolboxIdlProgramDef>,
    },
    Vec {
        items: Box<ToolboxIdlProgramDef>,
    },
    Array {
        items: Box<ToolboxIdlProgramDef>,
        length: Box<ToolboxIdlProgramDef>,
    },
    Struct {
        fields: Vec<(String, ToolboxIdlProgramDef)>,
    },
    Enum {
        variants: Vec<(String, Vec<ToolboxIdlProgramDef>)>,
    },
    Generic {
        symbol: String,
    },
    Const {
        literal: usize, // TODO - what other kind of consts can be supported ?
    },
    Primitive {
        primitive: ToolboxIdlProgramDefPrimitive,
    },
}

impl ToolboxIdlProgramDef {
    pub(crate) fn try_parse(
        idl_def: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramDef, ToolboxIdlError> {
        if let Some(idl_def_object) = idl_def.as_object() {
            return ToolboxIdlProgramDef::try_parse_object(
                idl_def_object,
                breadcrumbs,
            );
        }
        if let Some(idl_def_array) = idl_def.as_array() {
            return ToolboxIdlProgramDef::try_parse_array(
                idl_def_array,
                breadcrumbs,
            );
        }
        if let Some(idl_def_str) = idl_def.as_str() {
            return ToolboxIdlProgramDef::try_parse_str(
                idl_def_str,
                breadcrumbs,
            );
        }
        if let Some(idl_def_u64) = idl_def.as_u64() {
            return ToolboxIdlProgramDef::try_parse_u64(
                idl_def_u64,
                breadcrumbs,
            );
        }
        idl_err(
            "Expected type value: object, array, string or number",
            &breadcrumbs.as_idl("def"),
        )
    }

    fn try_parse_object(
        idl_def_object: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramDef, ToolboxIdlError> {
        if let Some(idl_def_defined) = idl_def_object.get("defined") {
            return ToolboxIdlProgramDef::try_parse_defined(
                idl_def_defined,
                breadcrumbs,
            );
        }
        if let Some(idl_def_option) = idl_def_object.get("option") {
            return ToolboxIdlProgramDef::try_parse_option(
                idl_def_option,
                breadcrumbs,
            );
        }
        if let Some(idl_def_vec) = idl_def_object.get("vec") {
            return ToolboxIdlProgramDef::try_parse_vec(
                idl_def_vec,
                breadcrumbs,
            );
        }
        if let Some(idl_def_array) =
            idl_object_get_key_as_array(idl_def_object, "array")
        {
            return ToolboxIdlProgramDef::try_parse_array(
                idl_def_array,
                breadcrumbs,
            );
        }
        if let Some(idl_def_struct_fields) =
            idl_object_get_key_as_array(idl_def_object, "fields")
        {
            return ToolboxIdlProgramDef::try_parse_struct_fields(
                idl_def_struct_fields,
                breadcrumbs,
            );
        }
        if let Some(idl_def_enum_variants) =
            idl_object_get_key_as_array(idl_def_object, "variants")
        {
            return ToolboxIdlProgramDef::try_parse_enum_variants(
                idl_def_enum_variants,
                breadcrumbs,
            );
        }
        if let Some(idl_def_generic_symbol) =
            idl_object_get_key_as_str(idl_def_object, "generic")
        {
            return ToolboxIdlProgramDef::try_parse_generic_symbol(
                idl_def_generic_symbol,
                breadcrumbs,
            );
        }
        idl_err(
            "Missing type object key: defined/option/fields/variants/array/vec/generic",
            &breadcrumbs.as_idl("def(object)"),
        )
    }

    fn try_parse_array(
        idl_def_array: &[Value],
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramDef, ToolboxIdlError> {
        if idl_def_array.len() == 1 {
            return ToolboxIdlProgramDef::try_parse_vec(
                &idl_def_array[0],
                breadcrumbs,
            );
        }
        if idl_def_array.len() == 2 {
            return Ok(ToolboxIdlProgramDef::Array {
                items: Box::new(ToolboxIdlProgramDef::try_parse(
                    &idl_def_array[0],
                    &breadcrumbs.with_idl("values"),
                )?),
                length: Box::new(ToolboxIdlProgramDef::try_parse(
                    &idl_def_array[1],
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
        idl_def_str: &str,
        _breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramDef, ToolboxIdlError> {
        Ok(match ToolboxIdlProgramDefPrimitive::try_parse(idl_def_str) {
            Some(program_def_primitive) => ToolboxIdlProgramDef::Primitive {
                primitive: program_def_primitive,
            },
            None => ToolboxIdlProgramDef::Defined {
                name: idl_def_str.to_string(),
                generics: vec![],
            },
        })
    }

    fn try_parse_u64(
        idl_def_u64: u64,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramDef, ToolboxIdlError> {
        Ok(ToolboxIdlProgramDef::Const {
            literal: idl_map_err_invalid_integer(
                usize::try_from(idl_def_u64),
                &breadcrumbs.idl(),
            )?,
        })
    }

    fn try_parse_defined(
        idl_def_defined: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramDef, ToolboxIdlError> {
        let program_def_defined_name =
            idl_value_as_str_or_object_with_name_as_str_or_else(
                idl_def_defined,
                &breadcrumbs.as_idl("defined"),
            )?;
        let mut program_def_defined_generics = vec![];
        if let Some(idl_def_defined_generics) =
            idl_value_as_object_get_key_as_array(idl_def_defined, "generics")
        {
            for (idl_def_defined_generic, breadcrumbs) in
                idl_array_get_scoped_object_array_or_else(
                    idl_def_defined_generics,
                    &breadcrumbs.with_idl("generics"),
                )?
            {
                program_def_defined_generics.push(
                    ToolboxIdlProgramDef::try_parse_defined_generic(
                        idl_def_defined_generic,
                        &breadcrumbs,
                    )?,
                );
            }
        }
        Ok(ToolboxIdlProgramDef::Defined {
            name: program_def_defined_name.to_string(),
            generics: program_def_defined_generics,
        })
    }

    fn try_parse_defined_generic(
        idl_def_defined_generic: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramDef, ToolboxIdlError> {
        match idl_object_get_key_as_str_or_else(
            idl_def_defined_generic,
            "kind",
            &breadcrumbs.idl(),
        )? {
            "type" => ToolboxIdlProgramDef::try_parse(
                idl_object_get_key_or_else(
                    idl_def_defined_generic,
                    "type",
                    &breadcrumbs.idl(),
                )?,
                &breadcrumbs,
            ),
            "const" => Ok(ToolboxIdlProgramDef::Const {
                literal: idl_object_get_key_as_str_or_else(
                    idl_def_defined_generic,
                    "value",
                    &breadcrumbs.idl(),
                )?
                .parse()
                .map_err(|err| {
                    ToolboxIdlError::InvalidConstLiteral {
                        parsing: err,
                        context: breadcrumbs.as_idl("value"),
                    }
                })?,
            }),
            _ => idl_err("Unknown generic kind", &breadcrumbs.idl()),
        }
    }

    fn try_parse_option(
        idl_def_option: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramDef, ToolboxIdlError> {
        Ok(ToolboxIdlProgramDef::Option {
            content: Box::new(ToolboxIdlProgramDef::try_parse(
                idl_def_option,
                &breadcrumbs.with_idl("option"),
            )?),
        })
    }

    fn try_parse_vec(
        idl_def_vec: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramDef, ToolboxIdlError> {
        Ok(ToolboxIdlProgramDef::Vec {
            items: Box::new(ToolboxIdlProgramDef::try_parse(
                idl_def_vec,
                &breadcrumbs.with_idl("vec"),
            )?),
        })
    }

    fn try_parse_struct_fields(
        idl_def_struct_fields: &[Value],
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramDef, ToolboxIdlError> {
        let mut program_def_struct_fields = vec![];
        for (idl_def_struct_field_name, idl_def_struct_field, breadcrumbs) in
            idl_array_get_scoped_named_object_array_or_else(
                idl_def_struct_fields,
                breadcrumbs,
            )?
        {
            let idl_def_struct_field_type = idl_object_get_key_or_else(
                idl_def_struct_field,
                "type",
                &breadcrumbs.idl(),
            )?;
            program_def_struct_fields.push((
                idl_def_struct_field_name.to_string(),
                ToolboxIdlProgramDef::try_parse(
                    idl_def_struct_field_type,
                    &breadcrumbs,
                )?,
            ));
        }
        Ok(ToolboxIdlProgramDef::Struct { fields: program_def_struct_fields })
    }

    fn try_parse_enum_variants(
        idl_def_enum_variants: &[Value],
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramDef, ToolboxIdlError> {
        let mut program_def_enum_variants = vec![];
        for (index, idl_def_enum_variant) in
            idl_def_enum_variants.iter().enumerate()
        {
            let idl_def_enum_variant_name =
                idl_value_as_str_or_object_with_name_as_str_or_else(
                    idl_def_enum_variant,
                    &breadcrumbs.as_idl(&format!("variants[{}]", index)),
                )?;
            let mut program_def_enum_variant_fields = vec![];
            if let Some(idl_def_enum_variant_fields) =
                idl_value_as_object_get_key_as_array(
                    idl_def_enum_variant,
                    "fields",
                )
            {
                for (index, idl_def_enum_variant_field) in
                    idl_def_enum_variant_fields.iter().enumerate()
                {
                    program_def_enum_variant_fields.push(
                        ToolboxIdlProgramDef::try_parse_enum_variant_field(
                            idl_def_enum_variant_field,
                            &breadcrumbs.with_idl(&format!("[{}]", index)),
                        )?,
                    );
                }
            }
            program_def_enum_variants.push((
                idl_def_enum_variant_name.to_string(),
                program_def_enum_variant_fields,
            ));
        }
        Ok(ToolboxIdlProgramDef::Enum { variants: program_def_enum_variants })
    }

    fn try_parse_enum_variant_field(
        idl_def_enum_variant_field: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramDef, ToolboxIdlError> {
        if let Some(idl_def_enum_variant_field_type) =
            idl_value_as_object_get_key(idl_def_enum_variant_field, "type")
        {
            return ToolboxIdlProgramDef::try_parse(
                idl_def_enum_variant_field_type,
                breadcrumbs,
            );
        }
        ToolboxIdlProgramDef::try_parse(idl_def_enum_variant_field, breadcrumbs)
    }

    fn try_parse_generic_symbol(
        idl_def_generic_symbol: &str,
        _breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramDef, ToolboxIdlError> {
        Ok(ToolboxIdlProgramDef::Generic {
            symbol: idl_def_generic_symbol.to_string(),
        })
    }
}
