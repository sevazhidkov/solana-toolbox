use std::str::FromStr;

use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
use crate::toolbox_idl_type_primitive::ToolboxIdlTypePrimitive;
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
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_u64;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;

impl ToolboxIdlTypeFull {
    pub fn try_serialize(
        &self,
        value: &Value,
        data: &mut Vec<u8>,
        // Config object for pubkey hashmap
        deserializable: bool,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        match self {
            ToolboxIdlTypeFull::Option {
                prefix_bytes,
                content,
            } => ToolboxIdlTypeFull::try_serialize_option(
                prefix_bytes,
                content,
                value,
                data,
                deserializable,
                &breadcrumbs.with_idl("option"),
            ),
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
                    length,
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
            ToolboxIdlTypeFull::Padded {
                size_bytes,
                content,
            } => ToolboxIdlTypeFull::try_serialize_padded(
                size_bytes,
                content,
                value,
                data,
                deserializable,
                &breadcrumbs.with_idl("padded"),
            ),
            ToolboxIdlTypeFull::Const { literal } => idl_err(
                &format!("Can't use a const literal directly: {:?}", literal),
                &breadcrumbs.idl(),
            ),
            ToolboxIdlTypeFull::Primitive { primitive } => {
                ToolboxIdlTypeFull::try_serialize_primitive(
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
        option_prefix_bytes: &u8,
        option_content: &ToolboxIdlTypeFull,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        if value.is_null() {
            for _ in 0..*option_prefix_bytes {
                data.push(0);
            }
            Ok(())
        } else {
            data.push(1);
            for _ in 1..*option_prefix_bytes {
                data.push(0);
            }
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
        if vec_items
            == (&ToolboxIdlTypeFull::Primitive {
                primitive: ToolboxIdlTypePrimitive::U8,
            })
        {
            let bytes = try_read_value_to_bytes(value, breadcrumbs)?;
            if deserializable {
                data.extend_from_slice(bytemuck::bytes_of::<u32>(
                    &u32::try_from(bytes.len()).unwrap(),
                ));
            }
            data.extend_from_slice(&bytes);
            return Ok(());
        }
        let values = idl_as_array_or_else(value, &breadcrumbs.as_val("vec"))?;
        if deserializable {
            data.extend_from_slice(bytemuck::bytes_of::<u32>(
                &u32::try_from(values.len()).unwrap(),
            ));
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
        array_items: &ToolboxIdlTypeFull,
        array_length: &u64,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        let array_length = usize::try_from(*array_length).unwrap();
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
        struct_fields: &ToolboxIdlTypeFullFields,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        struct_fields.try_serialize(value, data, deserializable, breadcrumbs)
    }

    fn try_serialize_enum(
        enum_variants: &[(String, ToolboxIdlTypeFullFields)],
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
        for (enum_code, enum_variant, breadcrumbs) in
            idl_iter_get_scoped_values(enum_variants, breadcrumbs)?
        {
            if enum_variant.0 == value_enum {
                data.push(u8::try_from(enum_code).unwrap());
                return enum_variant.1.try_serialize(
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

    fn try_serialize_padded(
        padded_size_bytes: &u64,
        padded_content: &ToolboxIdlTypeFull,
        value: &Value,
        data: &mut Vec<u8>,
        deserializable: bool,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        let padded_size_bytes = usize::try_from(*padded_size_bytes).unwrap();
        let data_len_enforced = data.len() + padded_size_bytes;
        padded_content.try_serialize(
            value,
            data,
            deserializable,
            breadcrumbs,
        )?;
        while data.len() < data_len_enforced {
            data.push(0);
        }
        Ok(())
    }

    fn try_serialize_primitive(
        primitive: &ToolboxIdlTypePrimitive,
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
            ToolboxIdlTypePrimitive::U8 => {
                write_data_using_u_number!(u8);
            },
            ToolboxIdlTypePrimitive::U16 => {
                write_data_using_u_number!(u16);
            },
            ToolboxIdlTypePrimitive::U32 => {
                write_data_using_u_number!(u32);
            },
            ToolboxIdlTypePrimitive::U64 => {
                write_data_using_u_number!(u64);
            },
            ToolboxIdlTypePrimitive::U128 => {
                let value_integer = idl_as_u128_or_else(value, context)?;
                data.extend_from_slice(bytemuck::bytes_of::<u128>(
                    &value_integer,
                ));
            },
            ToolboxIdlTypePrimitive::I8 => {
                write_data_using_i_number!(i8);
            },
            ToolboxIdlTypePrimitive::I16 => {
                write_data_using_i_number!(i16);
            },
            ToolboxIdlTypePrimitive::I32 => {
                write_data_using_i_number!(i32);
            },
            ToolboxIdlTypePrimitive::I64 => {
                write_data_using_i_number!(i64);
            },
            ToolboxIdlTypePrimitive::I128 => {
                let value_integer = idl_as_i128_or_else(value, context)?;
                data.extend_from_slice(bytemuck::bytes_of::<i128>(
                    &value_integer,
                ));
            },
            ToolboxIdlTypePrimitive::F32 => {
                let value_floating = idl_as_f64_or_else(value, context)? as f32;
                data.extend_from_slice(bytemuck::bytes_of::<f32>(
                    &value_floating,
                ));
            },
            ToolboxIdlTypePrimitive::F64 => {
                let value_floating = idl_as_f64_or_else(value, context)?;
                data.extend_from_slice(bytemuck::bytes_of::<f64>(
                    &value_floating,
                ));
            },
            ToolboxIdlTypePrimitive::Boolean => {
                data.push(if idl_as_bool_or_else(value, context)? {
                    1
                } else {
                    0
                });
            },
            ToolboxIdlTypePrimitive::String => {
                let value_str = idl_as_str_or_else(value, context)?;
                if deserializable {
                    data.extend_from_slice(bytemuck::bytes_of::<u32>(
                        &u32::try_from(value_str.len()).unwrap(),
                    ));
                }
                data.extend_from_slice(value_str.as_bytes());
            },
            ToolboxIdlTypePrimitive::PublicKey => {
                // TODO (FAR) - support pubkeys as keypair arrays ?
                let value_str = idl_as_str_or_else(value, context)?;
                data.extend_from_slice(bytemuck::bytes_of::<Pubkey>(
                    // TODO (MEDIUM) - use better error handling and use endpoint's util ?
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
    pub fn try_serialize(
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
            ToolboxIdlTypeFullFields::Unamed(fields) => {
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
            ToolboxIdlTypeFullFields::None => {},
        }
        Ok(())
    }
}

fn try_read_value_to_bytes(
    value: &Value,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Vec<u8>, ToolboxIdlError> {
    if let Some(value_array) = value.as_array() {
        return idl_as_bytes_or_else(value_array, &breadcrumbs.val());
    }
    if let Some(value_object) = value.as_object() {
        if let Some(data) = idl_object_get_key_as_str(value_object, "hex") {
            return try_read_hex_to_bytes(data, breadcrumbs);
        }
        if let Some(data) = idl_object_get_key_as_str(value_object, "base58") {
            return Ok(ToolboxEndpoint::sanitize_and_decode_base58(data)?);
        }
        if let Some(data) = idl_object_get_key_as_str(value_object, "base64") {
            return Ok(ToolboxEndpoint::sanitize_and_decode_base64(data)?);
        }
        if let Some(data) = idl_object_get_key_as_str(value_object, "utf8") {
            return Ok(data.as_bytes().to_vec());
        }
        if let Some(data) = idl_object_get_key_as_u64(value_object, "zeroes") {
            return Ok(vec![0; usize::try_from(data).unwrap()]);
        }
    }
    idl_err(
        "Could not read bytes, expected an array/object",
        &breadcrumbs.val(),
    )
}

fn try_read_hex_to_bytes(
    data: &str,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<Vec<u8>, ToolboxIdlError> {
    let hex = data.replace(|c| !char::is_ascii_alphanumeric(&c), "");
    let mut bytes = vec![];
    for byte in 0..(hex.len() / 2) {
        let byte_idx = byte * 2;
        let byte_hex = &hex[byte_idx..byte_idx + 2];
        bytes.push(u8::from_str_radix(byte_hex, 16).map_err(|err| {
            ToolboxIdlError::InvalidNumber {
                parsing: err,
                context: breadcrumbs.as_val(&format!("[{}]", byte_idx)),
            }
        })?);
    }
    Ok(bytes)
}
