use std::str::FromStr;

use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_primitive::ToolboxIdlPrimitive;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
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

impl ToolboxIdlTypeFull {
    pub(crate) fn try_serialize(
        &self,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        match self {
            ToolboxIdlTypeFull::Option { content } => {
                ToolboxIdlTypeFull::try_serialize_option(
                    content,
                    value,
                    data,
                    deserializable,
                    &breadcrumbs.with_idl("option"),
                )
            },
            ToolboxIdlTypeFull::Vec { items } => {
                ToolboxIdlTypeFull::try_serialize_vec(
                    items,
                    value,
                    data,
                    deserializable,
                    &breadcrumbs.with_idl("vec"),
                )
            },
            ToolboxIdlTypeFull::Array { items, length } => {
                ToolboxIdlTypeFull::try_serialize_array(
                    items,
                    *length,
                    value,
                    data,
                    deserializable,
                    &breadcrumbs.with_idl("array"),
                )
            },
            ToolboxIdlTypeFull::Struct { fields } => {
                ToolboxIdlTypeFull::try_serialize_struct(
                    fields,
                    value,
                    data,
                    deserializable,
                    &breadcrumbs.with_idl("struct"),
                )
            },
            ToolboxIdlTypeFull::Enum { variants } => {
                ToolboxIdlTypeFull::try_serialize_enum(
                    variants,
                    value,
                    data,
                    deserializable,
                    &breadcrumbs.with_idl("enum"),
                )
            },
            ToolboxIdlTypeFull::Primitive { primitive } => {
                ToolboxIdlTypeFull::try_serialize_primitive(
                    primitive,
                    value,
                    data,
                    deserializable,
                    breadcrumbs,
                )
            },
            ToolboxIdlTypeFull::Const { literal } => idl_err(
                &format!("Can't use a const literal directly: {:?}", literal),
                &breadcrumbs.idl(),
            ),
        }
    }

    fn try_serialize_option(
        option_content: &ToolboxIdlTypeFull,
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
        vec_items: &ToolboxIdlTypeFull,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        let value_array =
            idl_as_array_or_else(value, &breadcrumbs.as_val("vec"))?;
        if deserializable {
            let value_length = u32::try_from(value_array.len()).unwrap();
            data.extend_from_slice(bytemuck::bytes_of::<u32>(&value_length));
        }
        for (_, value_item, breadcrumbs) in
            idl_iter_get_scoped_values(value_array, breadcrumbs)?
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
        array_items: &ToolboxIdlTypeFull,
        array_length: usize,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        let value_array =
            idl_as_array_or_else(value, &breadcrumbs.as_val("array"))?;
        if value_array.len() != array_length {
            return idl_err(
                &format!(
                    "value array is not the correct size: expected {} items, found {} items",
                    array_length,
                    value_array.len()
                ),
                &breadcrumbs.as_idl("value array"),
            );
        }
        for (_, value_item, breadcrumbs) in
            idl_iter_get_scoped_values(value_array, breadcrumbs)?
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
        struct_fields: &ToolboxIdlTypeFullFields,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        ToolboxIdlTypeFullFields::try_serialize(
            struct_fields,
            value,
            data,
            deserializable,
            breadcrumbs,
        )
    }

    fn try_serialize_enum(
        enum_variants: &[(String, ToolboxIdlTypeFullFields)],
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        let (value_enum, value_fields) = if let Some(value_string) =
            value.as_str()
        {
            (value_string, &Value::Null)
        } else {
            let value_array = idl_as_array_or_else(value, &breadcrumbs.val())?;
            if value_array.len() != 2 {
                return idl_err(
                    "Expected an array of 2 item [{enum}, {fields}]",
                    &breadcrumbs.val(),
                );
            }
            let value_string =
                idl_as_str_or_else(&value_array[0], &breadcrumbs.val())?;
            (value_string, &value_array[1])
        };
        for (enum_variant_index, enum_variant, breadcrumbs) in
            idl_iter_get_scoped_values(enum_variants, breadcrumbs)?
        {
            if enum_variant.0 == value_enum {
                data.push(u8::try_from(enum_variant_index).unwrap());
                return ToolboxIdlTypeFullFields::try_serialize(
                    &enum_variant.1,
                    value_fields,
                    data,
                    deserializable,
                    &breadcrumbs,
                );
            }
        }
        idl_err("could not find matching enum", &breadcrumbs.as_val(value_enum))
    }

    fn try_serialize_primitive(
        primitive: &ToolboxIdlPrimitive,
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
            ToolboxIdlPrimitive::U8 => {
                write_data_using_u_number!(u8);
            },
            ToolboxIdlPrimitive::U16 => {
                write_data_using_u_number!(u16);
            },
            ToolboxIdlPrimitive::U32 => {
                write_data_using_u_number!(u32);
            },
            ToolboxIdlPrimitive::U64 => {
                write_data_using_u_number!(u64);
            },
            ToolboxIdlPrimitive::U128 => {
                let value_integer = idl_as_u128_or_else(value, context)?;
                data.extend_from_slice(bytemuck::bytes_of::<u128>(
                    &value_integer,
                ));
            },
            ToolboxIdlPrimitive::I8 => {
                write_data_using_i_number!(i8);
            },
            ToolboxIdlPrimitive::I16 => {
                write_data_using_i_number!(i16);
            },
            ToolboxIdlPrimitive::I32 => {
                write_data_using_i_number!(i32);
            },
            ToolboxIdlPrimitive::I64 => {
                write_data_using_i_number!(i64);
            },
            ToolboxIdlPrimitive::I128 => {
                let value_integer = idl_as_i128_or_else(value, context)?;
                data.extend_from_slice(bytemuck::bytes_of::<i128>(
                    &value_integer,
                ));
            },
            ToolboxIdlPrimitive::F32 => {
                let value_floating = idl_as_f64_or_else(value, context)? as f32;
                data.extend_from_slice(bytemuck::bytes_of::<f32>(
                    &value_floating,
                ));
            },
            ToolboxIdlPrimitive::F64 => {
                let value_floating = idl_as_f64_or_else(value, context)?;
                data.extend_from_slice(bytemuck::bytes_of::<f64>(
                    &value_floating,
                ));
            },
            ToolboxIdlPrimitive::Bytes => {
                let value_array = idl_as_array_or_else(value, context)?;
                let value_bytes = idl_as_bytes_or_else(value_array, context)?;
                if deserializable {
                    data.extend_from_slice(bytemuck::bytes_of::<u32>(
                        &u32::try_from(value_bytes.len()).unwrap(),
                    ));
                }
                data.extend_from_slice(&value_bytes);
            },
            ToolboxIdlPrimitive::Boolean => {
                data.push(if idl_as_bool_or_else(value, context)? {
                    1
                } else {
                    0
                });
            },
            ToolboxIdlPrimitive::String => {
                let value_str = idl_as_str_or_else(value, context)?;
                if deserializable {
                    data.extend_from_slice(bytemuck::bytes_of::<u32>(
                        &u32::try_from(value_str.len()).unwrap(),
                    ));
                }
                data.extend_from_slice(value_str.as_bytes());
            },
            ToolboxIdlPrimitive::PublicKey => {
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

impl ToolboxIdlTypeFullFields {
    fn try_serialize(
        &self,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        match self {
            ToolboxIdlTypeFullFields::Named(fields) => {
                let value = idl_as_object_or_else(value, &breadcrumbs.val())?;
                for (name, field) in fields {
                    let value_field = idl_object_get_key_or_else(
                        value,
                        &name,
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
            ToolboxIdlTypeFullFields::Unamed(fields) => {
                let value_array =
                    idl_as_array_or_else(value, &breadcrumbs.val())?;
                if value_array.len() != fields.len() {
                    return idl_err(
                        "Wrong number of unamed fields",
                        &breadcrumbs.val(),
                    );
                }
                for (field_index, field, breadcrumbs) in
                    idl_iter_get_scoped_values(fields, breadcrumbs)?
                {
                    field.try_serialize(
                        &value_array[field_index],
                        data,
                        deserializable,
                        &breadcrumbs,
                    )?;
                }
            },
            ToolboxIdlTypeFullFields::None => {},
        }
        Ok(())
    }
}
