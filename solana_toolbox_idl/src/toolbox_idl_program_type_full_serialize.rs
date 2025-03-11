use std::str::FromStr;

use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_type_full::ToolboxIdlProgramTypeFull;
use crate::toolbox_idl_program_type_full::ToolboxIdlProgramTypeFullFields;
use crate::toolbox_idl_program_type_primitive::ToolboxIdlProgramTypePrimitive;
use crate::toolbox_idl_utils::idl_as_array_or_else;
use crate::toolbox_idl_utils::idl_as_bool_or_else;
use crate::toolbox_idl_utils::idl_as_bytes_or_else;
use crate::toolbox_idl_utils::idl_as_f64_or_else;
use crate::toolbox_idl_utils::idl_as_i128_or_else;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_as_str_or_else;
use crate::toolbox_idl_utils::idl_as_u128_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_iter_get_scoped_values;
use crate::toolbox_idl_utils::idl_map_err_invalid_integer;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;

impl ToolboxIdlProgramTypeFull {
    pub fn try_serialize(
        &self,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        match self {
            ToolboxIdlProgramTypeFull::Option { content } => {
                ToolboxIdlProgramTypeFull::try_serialize_option(
                    content,
                    value,
                    data,
                    deserializable,
                    &breadcrumbs.with_idl("option"),
                )
            },
            ToolboxIdlProgramTypeFull::Vec { items } => {
                ToolboxIdlProgramTypeFull::try_serialize_vec(
                    items,
                    value,
                    data,
                    deserializable,
                    &breadcrumbs.with_idl("vec"),
                )
            },
            ToolboxIdlProgramTypeFull::Array { items, length } => {
                ToolboxIdlProgramTypeFull::try_serialize_array(
                    items,
                    *length,
                    value,
                    data,
                    deserializable,
                    &breadcrumbs.with_idl("array"),
                )
            },
            ToolboxIdlProgramTypeFull::Struct { fields } => {
                ToolboxIdlProgramTypeFull::try_serialize_struct(
                    fields,
                    value,
                    data,
                    deserializable,
                    &breadcrumbs.with_idl("struct"),
                )
            },
            ToolboxIdlProgramTypeFull::Enum { variants } => {
                ToolboxIdlProgramTypeFull::try_serialize_enum(
                    variants,
                    value,
                    data,
                    deserializable,
                    &breadcrumbs.with_idl("enum"),
                )
            },
            ToolboxIdlProgramTypeFull::Const { literal } => idl_err(
                &format!("Can't use a const literal directly: {:?}", literal),
                &breadcrumbs.idl(),
            ),
            ToolboxIdlProgramTypeFull::Primitive { primitive } => {
                ToolboxIdlProgramTypeFull::try_serialize_primitive(
                    primitive,
                    value,
                    data,
                    deserializable,
                    breadcrumbs,
                )
            },
        }
    }

    fn try_serialize_option(
        option_content: &ToolboxIdlProgramTypeFull,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        if value.is_null() {
            data.push(0);
            Ok(())
        } else {
            data.push(1);
            option_content.try_serialize(
                value,
                data,
                deserializable,
                breadcrumbs,
            )
        }
    }

    fn try_serialize_vec(
        vec_items: &ToolboxIdlProgramTypeFull,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        let values = idl_as_array_or_else(value, &breadcrumbs.as_val("vec"))?;
        if deserializable {
            let value_length = u32::try_from(values.len()).unwrap();
            data.extend_from_slice(bytemuck::bytes_of::<u32>(&value_length));
        }
        for (_, value_item, breadcrumbs) in
            idl_iter_get_scoped_values(values, breadcrumbs)?
        {
            vec_items.try_serialize(
                value_item,
                data,
                deserializable,
                &breadcrumbs,
            )?;
        }
        Ok(())
    }

    fn try_serialize_array(
        array_items: &ToolboxIdlProgramTypeFull,
        array_length: usize,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        let values = idl_as_array_or_else(value, &breadcrumbs.as_val("array"))?;
        if values.len() != array_length {
            return idl_err(
                &format!(
                    "value array is not the correct size: expected {} items, found {} items",
                    array_length,
                    values.len()
                ),
                &breadcrumbs.as_idl("value array"),
            );
        }
        for (_, value_item, breadcrumbs) in
            idl_iter_get_scoped_values(values, breadcrumbs)?
        {
            array_items.try_serialize(
                value_item,
                data,
                deserializable,
                &breadcrumbs,
            )?;
        }
        Ok(())
    }

    fn try_serialize_struct(
        struct_fields: &ToolboxIdlProgramTypeFullFields,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        ToolboxIdlProgramTypeFullFields::try_serialize(
            struct_fields,
            value,
            data,
            deserializable,
            breadcrumbs,
        )
    }

    fn try_serialize_enum(
        enum_variants: &[(String, ToolboxIdlProgramTypeFullFields)],
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        let (value_enum, value_fields) =
            if let Some(value_string) = value.as_str() {
                (value_string, &Value::Null)
            } else {
                let values = idl_as_array_or_else(value, &breadcrumbs.val())?;
                if values.len() != 2 {
                    return idl_err(
                        "Expected an array of 2 item [{enum}, {fields}]",
                        &breadcrumbs.val(),
                    );
                }
                let value_string =
                    idl_as_str_or_else(&values[0], &breadcrumbs.val())?;
                (value_string, &values[1])
            };
        for (enum_variant_index, enum_variant, breadcrumbs) in
            idl_iter_get_scoped_values(enum_variants, breadcrumbs)?
        {
            if enum_variant.0 == value_enum {
                data.push(u8::try_from(enum_variant_index).unwrap());
                return ToolboxIdlProgramTypeFullFields::try_serialize(
                    &enum_variant.1,
                    value_fields,
                    data,
                    deserializable,
                    &breadcrumbs,
                );
            }
        }
        idl_err(
            "could not find matching enum",
            &breadcrumbs.as_val(value_enum),
        )
    }

    fn try_serialize_primitive(
        primitive: &ToolboxIdlProgramTypePrimitive,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        let context = &breadcrumbs.val();
        macro_rules! write_data_using_u_number {
            ($type:ident) => {
                let value_integer = idl_as_u128_or_else(value, context)?;
                let value_typed = idl_map_err_invalid_integer(
                    $type::try_from(value_integer),
                    context,
                )?;
                data.extend_from_slice(bytemuck::bytes_of::<$type>(
                    &value_typed,
                ));
            };
        }
        macro_rules! write_data_using_i_number {
            ($type:ident) => {
                let value_integer = idl_as_i128_or_else(value, context)?;
                let value_typed = idl_map_err_invalid_integer(
                    $type::try_from(value_integer),
                    context,
                )?;
                data.extend_from_slice(bytemuck::bytes_of::<$type>(
                    &value_typed,
                ));
            };
        }
        match primitive {
            ToolboxIdlProgramTypePrimitive::U8 => {
                write_data_using_u_number!(u8);
            },
            ToolboxIdlProgramTypePrimitive::U16 => {
                write_data_using_u_number!(u16);
            },
            ToolboxIdlProgramTypePrimitive::U32 => {
                write_data_using_u_number!(u32);
            },
            ToolboxIdlProgramTypePrimitive::U64 => {
                write_data_using_u_number!(u64);
            },
            ToolboxIdlProgramTypePrimitive::U128 => {
                let value_integer = idl_as_u128_or_else(value, context)?;
                data.extend_from_slice(bytemuck::bytes_of::<u128>(
                    &value_integer,
                ));
            },
            ToolboxIdlProgramTypePrimitive::I8 => {
                write_data_using_i_number!(i8);
            },
            ToolboxIdlProgramTypePrimitive::I16 => {
                write_data_using_i_number!(i16);
            },
            ToolboxIdlProgramTypePrimitive::I32 => {
                write_data_using_i_number!(i32);
            },
            ToolboxIdlProgramTypePrimitive::I64 => {
                write_data_using_i_number!(i64);
            },
            ToolboxIdlProgramTypePrimitive::I128 => {
                let value_integer = idl_as_i128_or_else(value, context)?;
                data.extend_from_slice(bytemuck::bytes_of::<i128>(
                    &value_integer,
                ));
            },
            ToolboxIdlProgramTypePrimitive::F32 => {
                let value_floating = idl_as_f64_or_else(value, context)? as f32;
                data.extend_from_slice(bytemuck::bytes_of::<f32>(
                    &value_floating,
                ));
            },
            ToolboxIdlProgramTypePrimitive::F64 => {
                let value_floating = idl_as_f64_or_else(value, context)?;
                data.extend_from_slice(bytemuck::bytes_of::<f64>(
                    &value_floating,
                ));
            },
            ToolboxIdlProgramTypePrimitive::Bytes => {
                let values = idl_as_array_or_else(value, context)?;
                let value_bytes = idl_as_bytes_or_else(values, context)?;
                if deserializable {
                    data.extend_from_slice(bytemuck::bytes_of::<u32>(
                        &u32::try_from(value_bytes.len()).unwrap(),
                    ));
                }
                data.extend_from_slice(&value_bytes);
            },
            ToolboxIdlProgramTypePrimitive::Boolean => {
                data.push(if idl_as_bool_or_else(value, context)? {
                    1
                } else {
                    0
                });
            },
            ToolboxIdlProgramTypePrimitive::String => {
                let value_str = idl_as_str_or_else(value, context)?;
                if deserializable {
                    data.extend_from_slice(bytemuck::bytes_of::<u32>(
                        &u32::try_from(value_str.len()).unwrap(),
                    ));
                }
                data.extend_from_slice(value_str.as_bytes());
            },
            ToolboxIdlProgramTypePrimitive::PublicKey => {
                let value_str = idl_as_str_or_else(value, context)?;
                data.extend_from_slice(bytemuck::bytes_of::<Pubkey>(
                    &Pubkey::from_str(value_str).map_err(|err| {
                        ToolboxIdlError::InvalidPubkey {
                            parsing: err,
                            context: context.clone(),
                        }
                    })?,
                ));
            },
        };
        Ok(())
    }
}

impl ToolboxIdlProgramTypeFullFields {
    fn try_serialize(
        &self,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        match self {
            ToolboxIdlProgramTypeFullFields::Named(fields) => {
                let value = idl_as_object_or_else(value, &breadcrumbs.val())?;
                for (name, field) in fields {
                    let value_field = idl_object_get_key_or_else(
                        value,
                        name,
                        &breadcrumbs.val(),
                    )?;
                    field.try_serialize(
                        value_field,
                        data,
                        deserializable,
                        &breadcrumbs.with_val(name),
                    )?;
                }
            },
            ToolboxIdlProgramTypeFullFields::Unamed(fields) => {
                let values = idl_as_array_or_else(value, &breadcrumbs.val())?;
                if values.len() != fields.len() {
                    return idl_err(
                        "Wrong number of unamed fields",
                        &breadcrumbs.val(),
                    );
                }
                for (field_index, field, breadcrumbs) in
                    idl_iter_get_scoped_values(fields, breadcrumbs)?
                {
                    field.try_serialize(
                        &values[field_index],
                        data,
                        deserializable,
                        &breadcrumbs,
                    )?;
                }
            },
            ToolboxIdlProgramTypeFullFields::None => {},
        }
        Ok(())
    }
}
