use std::cmp::max;

use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
use crate::toolbox_idl_type_primitive::ToolboxIdlTypePrimitive;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_f32_from_bytes_at;
use crate::toolbox_idl_utils::idl_f64_from_bytes_at;
use crate::toolbox_idl_utils::idl_i128_from_bytes_at;
use crate::toolbox_idl_utils::idl_i16_from_bytes_at;
use crate::toolbox_idl_utils::idl_i32_from_bytes_at;
use crate::toolbox_idl_utils::idl_i64_from_bytes_at;
use crate::toolbox_idl_utils::idl_i8_from_bytes_at;
use crate::toolbox_idl_utils::idl_iter_get_scoped_values;
use crate::toolbox_idl_utils::idl_map_err_invalid_integer;
use crate::toolbox_idl_utils::idl_pubkey_from_bytes_at;
use crate::toolbox_idl_utils::idl_slice_from_bytes;
use crate::toolbox_idl_utils::idl_u128_from_bytes_at;
use crate::toolbox_idl_utils::idl_u16_from_bytes_at;
use crate::toolbox_idl_utils::idl_u32_from_bytes_at;
use crate::toolbox_idl_utils::idl_u64_from_bytes_at;
use crate::toolbox_idl_utils::idl_u8_from_bytes_at;

impl ToolboxIdlTypeFull {
    pub fn try_deserialize(
        &self,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        match self {
            ToolboxIdlTypeFull::Option {
                prefix_bytes,
                content,
            } => ToolboxIdlTypeFull::try_deserialize_option(
                prefix_bytes,
                content,
                data,
                data_offset,
                &breadcrumbs.with_idl("option"),
            ),
            ToolboxIdlTypeFull::Vec { items } => {
                ToolboxIdlTypeFull::try_deserialize_vec(
                    items,
                    data,
                    data_offset,
                    &breadcrumbs.with_idl("vec"),
                )
            },
            ToolboxIdlTypeFull::Array { items, length } => {
                ToolboxIdlTypeFull::try_deserialize_array(
                    items,
                    length,
                    data,
                    data_offset,
                    &breadcrumbs.with_idl("array"),
                )
            },
            ToolboxIdlTypeFull::Struct { fields } => {
                ToolboxIdlTypeFull::try_deserialize_struct(
                    fields,
                    data,
                    data_offset,
                    &breadcrumbs.with_idl("struct"),
                )
            },
            ToolboxIdlTypeFull::Enum { variants } => {
                ToolboxIdlTypeFull::try_deserialize_enum(
                    variants,
                    data,
                    data_offset,
                    &breadcrumbs.with_idl("enum"),
                )
            },
            ToolboxIdlTypeFull::Padded {
                size_bytes,
                content,
            } => ToolboxIdlTypeFull::try_deserialize_padded(
                size_bytes,
                content,
                data,
                data_offset,
                &breadcrumbs.with_idl("padded"),
            ),
            ToolboxIdlTypeFull::Const { literal } => idl_err(
                &format!("Can't use a const literal directly: {:?}", literal),
                &breadcrumbs.idl(),
            ),
            ToolboxIdlTypeFull::Primitive { primitive } => {
                ToolboxIdlTypeFull::try_deserialize_primitive(
                    primitive,
                    data,
                    data_offset,
                    breadcrumbs,
                )
            },
        }
    }

    fn try_deserialize_option(
        option_prefix_bytes: &u8,
        option_content: &ToolboxIdlTypeFull,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let data_flag = idl_u8_from_bytes_at(
            data,
            data_offset,
            &breadcrumbs.as_val("flag"),
        )?;
        let mut data_size = usize::from(*option_prefix_bytes);
        if data_flag > 0 {
            let (data_content_size, data_content) = option_content
                .try_deserialize(data, data_offset + data_size, breadcrumbs)?;
            data_size += data_content_size;
            Ok((data_size, data_content))
        } else {
            Ok((data_size, Value::Null))
        }
    }

    fn try_deserialize_vec(
        vec_items: &ToolboxIdlTypeFull,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let data_length = idl_u32_from_bytes_at(
            data,
            data_offset,
            &breadcrumbs.as_val("length"),
        )?;
        let mut data_size = std::mem::size_of_val(&data_length);
        let mut data_items = vec![];
        for (_, _, breadcrumbs) in idl_iter_get_scoped_values(
            &(0..data_length).collect::<Vec<u32>>(),
            breadcrumbs,
        )? {
            let (data_item_size, data_item) = vec_items.try_deserialize(
                data,
                data_offset + data_size,
                &breadcrumbs,
            )?;
            data_size += data_item_size;
            data_items.push(data_item);
        }
        Ok((data_size, json!(data_items)))
    }

    fn try_deserialize_array(
        array_items: &ToolboxIdlTypeFull,
        array_length: &u64,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let array_length = usize::try_from(*array_length).unwrap();
        let mut data_size = 0;
        let mut data_items = vec![];
        for (_, _, breadcrumbs) in idl_iter_get_scoped_values(
            &(0..array_length).collect::<Vec<usize>>(),
            breadcrumbs,
        )? {
            let (data_item_size, data_item) = array_items.try_deserialize(
                data,
                data_offset + data_size,
                &breadcrumbs,
            )?;
            data_size += data_item_size;
            data_items.push(data_item);
        }
        Ok((data_size, json!(data_items)))
    }

    fn try_deserialize_struct(
        struct_fields: &ToolboxIdlTypeFullFields,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        struct_fields.try_deserialize(data, data_offset, breadcrumbs)
    }

    fn try_deserialize_enum(
        enum_variants: &[(String, ToolboxIdlTypeFullFields)],
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let data_code = usize::from(idl_u8_from_bytes_at(
            data,
            data_offset,
            &breadcrumbs.as_val("enum"),
        )?);
        if data_code >= enum_variants.len() {
            return idl_err(
                &format!("Invalid enum value: {}", data_code),
                &breadcrumbs.as_idl("variants"),
            );
        }
        let mut data_size = std::mem::size_of::<u8>();
        let enum_variant = &enum_variants[data_code];
        let (enum_variant_name, enum_variant_fields) = enum_variant;
        let (data_fields_size, data_fields) = enum_variant_fields
            .try_deserialize(data, data_offset + data_size, breadcrumbs)?;
        data_size += data_fields_size;
        if data_fields.is_null() {
            Ok((data_size, json!(enum_variant_name)))
        } else {
            Ok((data_size, json!({ enum_variant_name: data_fields })))
        }
    }

    fn try_deserialize_padded(
        padded_size_bytes: &u64,
        padded_content: &ToolboxIdlTypeFull,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let padded_size_bytes = usize::try_from(*padded_size_bytes).unwrap();
        let (data_content_size, data_content) =
            padded_content.try_deserialize(data, data_offset, breadcrumbs)?;
        Ok((max(data_content_size, padded_size_bytes), data_content))
    }

    fn try_deserialize_primitive(
        primitive: &ToolboxIdlTypePrimitive,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let context = &breadcrumbs.val();
        Ok(match primitive {
            ToolboxIdlTypePrimitive::U8 => {
                let int = idl_u8_from_bytes_at(data, data_offset, context)?;
                (std::mem::size_of_val(&int), json!(int))
            },
            ToolboxIdlTypePrimitive::U16 => {
                let int = idl_u16_from_bytes_at(data, data_offset, context)?;
                (std::mem::size_of_val(&int), json!(int))
            },
            ToolboxIdlTypePrimitive::U32 => {
                let int = idl_u32_from_bytes_at(data, data_offset, context)?;
                (std::mem::size_of_val(&int), json!(int))
            },
            ToolboxIdlTypePrimitive::U64 => {
                let int = idl_u64_from_bytes_at(data, data_offset, context)?;
                (std::mem::size_of_val(&int), json!(int))
            },
            ToolboxIdlTypePrimitive::U128 => {
                let int = idl_u128_from_bytes_at(data, data_offset, context)?;
                (std::mem::size_of_val(&int), json!(int))
            },
            ToolboxIdlTypePrimitive::I8 => {
                let int = idl_i8_from_bytes_at(data, data_offset, context)?;
                (std::mem::size_of_val(&int), json!(int))
            },
            ToolboxIdlTypePrimitive::I16 => {
                let int = idl_i16_from_bytes_at(data, data_offset, context)?;
                (std::mem::size_of_val(&int), json!(int))
            },
            ToolboxIdlTypePrimitive::I32 => {
                let int = idl_i32_from_bytes_at(data, data_offset, context)?;
                (std::mem::size_of_val(&int), json!(int))
            },
            ToolboxIdlTypePrimitive::I64 => {
                let int = idl_i64_from_bytes_at(data, data_offset, context)?;
                (std::mem::size_of_val(&int), json!(int))
            },
            ToolboxIdlTypePrimitive::I128 => {
                let int = idl_i128_from_bytes_at(data, data_offset, context)?;
                (std::mem::size_of_val(&int), json!(int))
            },
            ToolboxIdlTypePrimitive::F32 => {
                let float =
                    idl_f32_from_bytes_at(data, data_offset, context)? as f64;
                (std::mem::size_of_val(&float), json!(float))
            },
            ToolboxIdlTypePrimitive::F64 => {
                let float = idl_f64_from_bytes_at(data, data_offset, context)?;
                (std::mem::size_of_val(&float), json!(float))
            },
            ToolboxIdlTypePrimitive::Boolean => {
                let data_flag =
                    idl_u8_from_bytes_at(data, data_offset, context)?;
                let data_size = std::mem::size_of_val(&data_flag);
                (data_size, json!(data_flag != 0))
            },
            ToolboxIdlTypePrimitive::String => {
                let data_length =
                    idl_u32_from_bytes_at(data, data_offset, context)?;
                let mut data_size = std::mem::size_of_val(&data_length);
                let data_bytes = idl_slice_from_bytes(
                    data,
                    data_offset + data_size,
                    idl_map_err_invalid_integer(
                        usize::try_from(data_length),
                        context,
                    )?,
                    context,
                )?;
                data_size += data_bytes.len();
                let data_string = String::from_utf8(data_bytes.to_vec())
                    .map_err(|err| ToolboxIdlError::InvalidString {
                        parsing: err,
                        context: context.clone(),
                    })?;
                (data_size, json!(data_string))
            },
            ToolboxIdlTypePrimitive::PublicKey => {
                let data_pubkey =
                    idl_pubkey_from_bytes_at(data, data_offset, context)?;
                let data_size = std::mem::size_of_val(&data_pubkey);
                (data_size, json!(data_pubkey.to_string()))
            },
        })
    }
}

impl ToolboxIdlTypeFullFields {
    pub fn try_deserialize(
        &self,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        Ok(match self {
            ToolboxIdlTypeFullFields::Named(fields) => {
                let mut data_size = 0;
                let mut data_fields = Map::new();
                for (field_name, field) in fields {
                    let breadcrumbs = &breadcrumbs.with_idl(field_name);
                    let (data_field_size, data_field) = field.try_deserialize(
                        data,
                        data_offset + data_size,
                        &breadcrumbs.with_val(field_name),
                    )?;
                    data_size += data_field_size;
                    data_fields.insert(field_name.to_string(), data_field);
                }
                (data_size, json!(data_fields))
            },
            ToolboxIdlTypeFullFields::Unnamed(fields) => {
                let mut data_size = 0;
                let mut data_fields = vec![];
                for (_, field, breadcrumbs) in
                    idl_iter_get_scoped_values(fields, breadcrumbs)?
                {
                    let (data_field_size, data_field) = field.try_deserialize(
                        data,
                        data_offset + data_size,
                        &breadcrumbs,
                    )?;
                    data_size += data_field_size;
                    data_fields.push(data_field);
                }
                (data_size, json!(data_fields))
            },
            ToolboxIdlTypeFullFields::None => (0, Value::Null),
        })
    }
}
