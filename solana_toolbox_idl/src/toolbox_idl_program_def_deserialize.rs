use serde_json::Map;
use serde_json::Number;
use serde_json::Value;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_def::ToolboxIdlProgramDef;
use crate::toolbox_idl_program_def_primitive::ToolboxIdlProgramDefPrimitive;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_f32_from_bytes_at;
use crate::toolbox_idl_utils::idl_f64_from_bytes_at;
use crate::toolbox_idl_utils::idl_i128_from_bytes_at;
use crate::toolbox_idl_utils::idl_i16_from_bytes_at;
use crate::toolbox_idl_utils::idl_i32_from_bytes_at;
use crate::toolbox_idl_utils::idl_i64_from_bytes_at;
use crate::toolbox_idl_utils::idl_i8_from_bytes_at;
use crate::toolbox_idl_utils::idl_map_err_invalid_integer;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;
use crate::toolbox_idl_utils::idl_ok_or_else;
use crate::toolbox_idl_utils::idl_pubkey_from_bytes_at;
use crate::toolbox_idl_utils::idl_slice_from_bytes;
use crate::toolbox_idl_utils::idl_u128_from_bytes_at;
use crate::toolbox_idl_utils::idl_u16_from_bytes_at;
use crate::toolbox_idl_utils::idl_u32_from_bytes_at;
use crate::toolbox_idl_utils::idl_u64_from_bytes_at;
use crate::toolbox_idl_utils::idl_u8_from_bytes_at;

impl ToolboxIdlProgramDef {
    pub(crate) fn try_deserialize(
        &self,
        idl: &ToolboxIdl,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        match self {
            ToolboxIdlProgramDef::Defined { name, generics } => {
                ToolboxIdlProgramDef::try_deserialize_defined(
                    idl,
                    name,
                    data,
                    data_offset,
                    &breadcrumbs.with_idl(name),
                )
            },
            ToolboxIdlProgramDef::Option { content } => {
                ToolboxIdlProgramDef::try_deserialize_option(
                    idl,
                    content,
                    data,
                    data_offset,
                    &breadcrumbs.with_idl("option"),
                )
            },
            ToolboxIdlProgramDef::Vec { items } => {
                ToolboxIdlProgramDef::try_deserialize_vec(
                    idl,
                    items,
                    data,
                    data_offset,
                    &breadcrumbs.with_idl("vec"),
                )
            },
            ToolboxIdlProgramDef::Array { length, items } => {
                ToolboxIdlProgramDef::try_deserialize_array(
                    idl,
                    length,
                    items,
                    data,
                    data_offset,
                    &breadcrumbs.with_idl("array"),
                )
            },
            ToolboxIdlProgramDef::Struct { fields } => {
                ToolboxIdlProgramDef::try_deserialize_struct(
                    idl,
                    fields,
                    data,
                    data_offset,
                    &breadcrumbs.with_idl("struct"),
                )
            },
            ToolboxIdlProgramDef::Enum { variants } => {
                ToolboxIdlProgramDef::try_deserialize_enum(
                    variants,
                    data,
                    data_offset,
                    &breadcrumbs.with_idl("enum"),
                )
            },
            ToolboxIdlProgramDef::Primitive { primitive } => {
                ToolboxIdlProgramDef::try_deserialize_primitive(
                    primitive,
                    data,
                    data_offset,
                    breadcrumbs,
                )
            },
            ToolboxIdlProgramDef::Const { literal: value } => todo!(),
            ToolboxIdlProgramDef::Generic { symbol } => todo!(),
        }
    }

    fn try_deserialize_defined(
        idl: &ToolboxIdl,
        program_def_defined_name: &str,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let program_type = idl_map_get_key_or_else(
            &idl.program_types,
            program_def_defined_name,
            &breadcrumbs.as_idl("$program_types"),
        )?;
        program_type.def.try_deserialize(idl, data, data_offset, breadcrumbs)
    }

    fn try_deserialize_option(
        idl: &ToolboxIdl,
        program_def_option_content_def: &ToolboxIdlProgramDef,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let data_flag = idl_u8_from_bytes_at(
            data,
            data_offset,
            &breadcrumbs.as_val("flag"),
        )?;
        let mut data_size = std::mem::size_of_val(&data_flag);
        if data_flag > 0 {
            let (data_content_size, data_content_value) =
                program_def_option_content_def.try_deserialize(
                    idl,
                    data,
                    data_offset + data_size,
                    breadcrumbs,
                )?;
            data_size += data_content_size;
            Ok((data_size, data_content_value))
        } else {
            Ok((data_size, Value::Null))
        }
    }

    fn try_deserialize_vec(
        idl: &ToolboxIdl,
        program_def_vec_items_def: &ToolboxIdlProgramDef,
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
        for index in 0..data_length {
            let (data_item_size, data_item_value) = program_def_vec_items_def
                .try_deserialize(
                idl,
                data,
                data_offset + data_size,
                &breadcrumbs.with_val(&format!("[{}]", index)),
            )?;
            data_size += data_item_size;
            data_items.push(data_item_value);
        }
        Ok((data_size, Value::Array(data_items)))
    }

    fn try_deserialize_array(
        idl: &ToolboxIdl,
        program_def_array_length_def: &ToolboxIdlProgramDef,
        program_def_array_items_def: &ToolboxIdlProgramDef,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let mut data_size = 0;
        let mut data_items = vec![];
        let program_def_array_length = idl_ok_or_else(
            program_def_array_length_def.as_const_literal(),
            "expected a literal",
            &breadcrumbs.as_idl("length"),
        )?;
        for index in 0..*program_def_array_length {
            let (data_item_size, data_item_value) = program_def_array_items_def
                .try_deserialize(
                    idl,
                    data,
                    data_offset + data_size,
                    &breadcrumbs.with_val(&format!("[{}]", index)),
                )?;
            data_size += data_item_size;
            data_items.push(data_item_value);
        }
        Ok((data_size, Value::Array(data_items)))
    }

    fn try_deserialize_struct(
        idl: &ToolboxIdl,
        program_def_struct_fields: &[(String, ToolboxIdlProgramDef)],
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let mut data_size = 0;
        let mut data_fields = Map::new();
        for (program_def_struct_field_name, program_def_struct_field_def) in
            program_def_struct_fields
        {
            let breadcrumbs =
                &breadcrumbs.with_idl(program_def_struct_field_name);
            let (data_field_size, data_field_value) =
                program_def_struct_field_def.try_deserialize(
                    idl,
                    data,
                    data_offset + data_size,
                    &breadcrumbs.with_val(program_def_struct_field_name),
                )?;
            data_size += data_field_size;
            data_fields.insert(
                program_def_struct_field_name.to_string(),
                data_field_value,
            );
        }
        Ok((data_size, Value::Object(data_fields)))
    }

    fn try_deserialize_enum(
        program_def_enum_variants: &[(String, Vec<ToolboxIdlProgramDef>)],
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let data_enum = idl_u8_from_bytes_at(
            data,
            data_offset,
            &breadcrumbs.as_val("enum"),
        )?;
        let data_index = usize::from(data_enum);
        if data_index >= program_def_enum_variants.len() {
            return idl_err(
                &format!("Invalid enum value: {}", data_index),
                &breadcrumbs.as_idl("variants"),
            );
        }
        let program_def_enum_variant = &program_def_enum_variants[data_index];
        // TODO - support enum variant fields
        Ok((
            std::mem::size_of_val(&data_enum),
            Value::String(program_def_enum_variant.0.to_string()),
        ))
    }

    fn try_deserialize_primitive(
        program_def_primitive: &ToolboxIdlProgramDefPrimitive,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let context = &breadcrumbs.val();
        Ok(match program_def_primitive {
            ToolboxIdlProgramDefPrimitive::U8 => {
                let int = idl_u8_from_bytes_at(data, data_offset, context)?;
                (std::mem::size_of_val(&int), Value::Number(Number::from(int)))
            },
            ToolboxIdlProgramDefPrimitive::U16 => {
                let int = idl_u16_from_bytes_at(data, data_offset, context)?;
                (std::mem::size_of_val(&int), Value::Number(Number::from(int)))
            },
            ToolboxIdlProgramDefPrimitive::U32 => {
                let int = idl_u32_from_bytes_at(data, data_offset, context)?;
                (std::mem::size_of_val(&int), Value::Number(Number::from(int)))
            },
            ToolboxIdlProgramDefPrimitive::U64 => {
                let int = idl_u64_from_bytes_at(data, data_offset, context)?;
                (std::mem::size_of_val(&int), Value::Number(Number::from(int)))
            },
            ToolboxIdlProgramDefPrimitive::U128 => {
                let int = idl_u128_from_bytes_at(data, data_offset, context)?;
                (
                    std::mem::size_of_val(&int),
                    Value::Number(
                        Number::from_u128(int).unwrap_or(Number::from(0)),
                    ),
                )
            },
            ToolboxIdlProgramDefPrimitive::I8 => {
                let int = idl_i8_from_bytes_at(data, data_offset, context)?;
                (std::mem::size_of_val(&int), Value::Number(Number::from(int)))
            },
            ToolboxIdlProgramDefPrimitive::I16 => {
                let int = idl_i16_from_bytes_at(data, data_offset, context)?;
                (std::mem::size_of_val(&int), Value::Number(Number::from(int)))
            },
            ToolboxIdlProgramDefPrimitive::I32 => {
                let int = idl_i32_from_bytes_at(data, data_offset, context)?;
                (std::mem::size_of_val(&int), Value::Number(Number::from(int)))
            },
            ToolboxIdlProgramDefPrimitive::I64 => {
                let int = idl_i64_from_bytes_at(data, data_offset, context)?;
                (std::mem::size_of_val(&int), Value::Number(Number::from(int)))
            },
            ToolboxIdlProgramDefPrimitive::I128 => {
                let int = idl_i128_from_bytes_at(data, data_offset, context)?;
                (
                    std::mem::size_of_val(&int),
                    Value::Number(
                        Number::from_i128(int).unwrap_or(Number::from(0)),
                    ),
                )
            },
            ToolboxIdlProgramDefPrimitive::F32 => {
                let float =
                    idl_f32_from_bytes_at(data, data_offset, context)? as f64;
                (
                    std::mem::size_of_val(&float),
                    Value::Number(
                        Number::from_f64(float).unwrap_or(Number::from(0)),
                    ),
                )
            },
            ToolboxIdlProgramDefPrimitive::F64 => {
                let float = idl_f64_from_bytes_at(data, data_offset, context)?;
                (
                    std::mem::size_of_val(&float),
                    Value::Number(
                        Number::from_f64(float).unwrap_or(Number::from(0)),
                    ),
                )
            },
            ToolboxIdlProgramDefPrimitive::Bytes => {
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
                let mut data_array = vec![];
                for data_byte in data_bytes {
                    data_array.push(Value::Number(Number::from(*data_byte)));
                }
                (data_size, Value::Array(data_array))
            },
            ToolboxIdlProgramDefPrimitive::Boolean => {
                let data_flag =
                    idl_u8_from_bytes_at(data, data_offset, context)?;
                let data_size = std::mem::size_of_val(&data_flag);
                (data_size, Value::Bool(data_flag != 0))
            },
            ToolboxIdlProgramDefPrimitive::String => {
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
                (data_size, Value::String(data_string))
            },
            ToolboxIdlProgramDefPrimitive::PublicKey => {
                let data_pubkey =
                    idl_pubkey_from_bytes_at(data, data_offset, context)?;
                let data_size = std::mem::size_of_val(&data_pubkey);
                (data_size, Value::String(data_pubkey.to_string()))
            },
        })
    }
}
