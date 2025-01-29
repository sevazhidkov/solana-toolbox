use serde_json::Map;
use serde_json::Number;
use serde_json::Value;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_typedef::ToolboxIdlProgramTypedef;
use crate::toolbox_idl_program_typedef_primitive::ToolboxIdlProgramTypedefPrimitiveKind;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_f32_from_bytes_at;
use crate::toolbox_idl_utils::idl_f64_from_bytes_at;
use crate::toolbox_idl_utils::idl_i128_from_bytes_at;
use crate::toolbox_idl_utils::idl_i16_from_bytes_at;
use crate::toolbox_idl_utils::idl_i32_from_bytes_at;
use crate::toolbox_idl_utils::idl_i64_from_bytes_at;
use crate::toolbox_idl_utils::idl_i8_from_bytes_at;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;
use crate::toolbox_idl_utils::idl_pubkey_from_bytes_at;
use crate::toolbox_idl_utils::idl_slice_from_bytes;
use crate::toolbox_idl_utils::idl_u128_from_bytes_at;
use crate::toolbox_idl_utils::idl_u16_from_bytes_at;
use crate::toolbox_idl_utils::idl_u32_from_bytes_at;
use crate::toolbox_idl_utils::idl_u64_from_bytes_at;
use crate::toolbox_idl_utils::idl_u8_from_bytes_at;

impl ToolboxIdlProgramTypedef {
    pub(crate) fn try_deserialize(
        &self,
        idl: &ToolboxIdl,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        match self {
            ToolboxIdlProgramTypedef::Defined { name } => {
                ToolboxIdlProgramTypedef::try_deserialize_defined(
                    idl,
                    name,
                    data,
                    data_offset,
                    &breadcrumbs.with_idl(name),
                )
            },
            ToolboxIdlProgramTypedef::Option { content } => {
                ToolboxIdlProgramTypedef::try_deserialize_option(
                    idl,
                    content,
                    data,
                    data_offset,
                    &breadcrumbs.with_idl("option"),
                )
            },
            ToolboxIdlProgramTypedef::Vec { items } => {
                ToolboxIdlProgramTypedef::try_deserialize_vec(
                    idl,
                    items,
                    data,
                    data_offset,
                    &breadcrumbs.with_idl("vec"),
                )
            },
            ToolboxIdlProgramTypedef::Array { length, items } => {
                ToolboxIdlProgramTypedef::try_deserialize_array(
                    idl,
                    *length,
                    items,
                    data,
                    data_offset,
                    &breadcrumbs.with_idl("array"),
                )
            },
            ToolboxIdlProgramTypedef::Struct { fields } => {
                ToolboxIdlProgramTypedef::try_deserialize_struct(
                    idl,
                    fields,
                    data,
                    data_offset,
                    &breadcrumbs.with_idl("struct"),
                )
            },
            ToolboxIdlProgramTypedef::Enum { variants } => {
                ToolboxIdlProgramTypedef::try_deserialize_enum(
                    variants,
                    data,
                    data_offset,
                    &breadcrumbs.with_idl("enum"),
                )
            },
            ToolboxIdlProgramTypedef::Primitive { kind } => {
                ToolboxIdlProgramTypedef::try_deserialize_primitive(
                    kind,
                    data,
                    data_offset,
                    breadcrumbs,
                )
            },
        }
    }

    fn try_deserialize_defined(
        idl: &ToolboxIdl,
        program_typdef_defined_name: &str,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let program_typedef = idl_map_get_key_or_else(
            &idl.program_typedefs,
            program_typdef_defined_name,
            &breadcrumbs.as_idl("$program_typedefs"),
        )?;
        program_typedef.try_deserialize(idl, data, data_offset, breadcrumbs)
    }

    fn try_deserialize_option(
        idl: &ToolboxIdl,
        program_typdef_option_content_typedef: &ToolboxIdlProgramTypedef,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let data_flag = idl_u8_from_bytes_at(
            data,
            data_offset,
            &breadcrumbs.as_val("flag"),
        )?;
        let mut data_size = size_of_val(&data_flag);
        if data_flag > 0 {
            let (data_content_size, data_content_value) =
                program_typdef_option_content_typedef.try_deserialize(
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
        program_typedef_vec_items_typedef: &ToolboxIdlProgramTypedef,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let data_length = idl_u32_from_bytes_at(
            data,
            data_offset,
            &breadcrumbs.as_val("length"),
        )?;
        let mut data_size = size_of_val(&data_length);
        let mut data_items = vec![];
        for index in 0..data_length {
            let (data_item_size, data_item_value) =
                program_typedef_vec_items_typedef.try_deserialize(
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
        program_typedef_array_length: u32,
        program_typedef_array_items_typedef: &ToolboxIdlProgramTypedef,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let mut data_size = 0;
        let mut data_items = vec![];
        for index in 0..program_typedef_array_length {
            let (data_item_size, data_item_value) =
                program_typedef_array_items_typedef.try_deserialize(
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
        program_typdef_struct_fields: &[(String, ToolboxIdlProgramTypedef)],
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let mut data_size = 0;
        let mut data_fields = Map::new();
        for (
            program_typedef_struct_field_name,
            program_typedef_struct_field_typedef,
        ) in program_typdef_struct_fields
        {
            let breadcrumbs =
                &breadcrumbs.with_idl(program_typedef_struct_field_name);
            let (data_field_size, data_field_value) =
                program_typedef_struct_field_typedef.try_deserialize(
                    idl,
                    data,
                    data_offset + data_size,
                    &breadcrumbs.with_val(program_typedef_struct_field_name),
                )?;
            data_size += data_field_size;
            data_fields.insert(
                program_typedef_struct_field_name.to_string(),
                data_field_value,
            );
        }
        Ok((data_size, Value::Object(data_fields)))
    }

    fn try_deserialize_enum(
        program_typedef_enum_variants: &[String],
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
        if data_index >= program_typedef_enum_variants.len() {
            return idl_err(
                &format!("Invalid enum value: {}", data_index),
                &breadcrumbs.as_idl("variants"),
            );
        }
        let program_typedef_enum_variant =
            &program_typedef_enum_variants[data_index];
        Ok((
            size_of_val(&data_enum),
            Value::String(program_typedef_enum_variant.to_string()),
        ))
    }

    fn try_deserialize_primitive(
        program_typedef_primitive_kind: &ToolboxIdlProgramTypedefPrimitiveKind,
        data: &[u8],
        data_offset: usize,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let context = &breadcrumbs.val();
        Ok(match program_typedef_primitive_kind {
            ToolboxIdlProgramTypedefPrimitiveKind::U8 => {
                let int = idl_u8_from_bytes_at(data, data_offset, context)?;
                (size_of_val(&int), Value::Number(Number::from(int)))
            },
            ToolboxIdlProgramTypedefPrimitiveKind::U16 => {
                let int = idl_u16_from_bytes_at(data, data_offset, context)?;
                (size_of_val(&int), Value::Number(Number::from(int)))
            },
            ToolboxIdlProgramTypedefPrimitiveKind::U32 => {
                let int = idl_u32_from_bytes_at(data, data_offset, context)?;
                (size_of_val(&int), Value::Number(Number::from(int)))
            },
            ToolboxIdlProgramTypedefPrimitiveKind::U64 => {
                let int = idl_u64_from_bytes_at(data, data_offset, context)?;
                (size_of_val(&int), Value::Number(Number::from(int)))
            },
            ToolboxIdlProgramTypedefPrimitiveKind::U128 => {
                let int = idl_u128_from_bytes_at(data, data_offset, context)?;
                (
                    size_of_val(&int),
                    Value::Number(
                        Number::from_u128(int).unwrap_or(Number::from(0)),
                    ),
                )
            },
            ToolboxIdlProgramTypedefPrimitiveKind::I8 => {
                let int = idl_i8_from_bytes_at(data, data_offset, context)?;
                (size_of_val(&int), Value::Number(Number::from(int)))
            },
            ToolboxIdlProgramTypedefPrimitiveKind::I16 => {
                let int = idl_i16_from_bytes_at(data, data_offset, context)?;
                (size_of_val(&int), Value::Number(Number::from(int)))
            },
            ToolboxIdlProgramTypedefPrimitiveKind::I32 => {
                let int = idl_i32_from_bytes_at(data, data_offset, context)?;
                (size_of_val(&int), Value::Number(Number::from(int)))
            },
            ToolboxIdlProgramTypedefPrimitiveKind::I64 => {
                let int = idl_i64_from_bytes_at(data, data_offset, context)?;
                (size_of_val(&int), Value::Number(Number::from(int)))
            },
            ToolboxIdlProgramTypedefPrimitiveKind::I128 => {
                let int = idl_i128_from_bytes_at(data, data_offset, context)?;
                (
                    size_of_val(&int),
                    Value::Number(
                        Number::from_i128(int).unwrap_or(Number::from(0)),
                    ),
                )
            },
            ToolboxIdlProgramTypedefPrimitiveKind::F32 => {
                let float =
                    idl_f32_from_bytes_at(data, data_offset, context)? as f64;
                (
                    size_of_val(&float),
                    Value::Number(
                        Number::from_f64(float).unwrap_or(Number::from(0)),
                    ),
                )
            },
            ToolboxIdlProgramTypedefPrimitiveKind::F64 => {
                let float = idl_f64_from_bytes_at(data, data_offset, context)?;
                (
                    size_of_val(&float),
                    Value::Number(
                        Number::from_f64(float).unwrap_or(Number::from(0)),
                    ),
                )
            },
            ToolboxIdlProgramTypedefPrimitiveKind::Boolean => {
                let data_flag =
                    idl_u8_from_bytes_at(data, data_offset, context)?;
                let data_size = size_of_val(&data_flag);
                (data_size, Value::Bool(data_flag != 0))
            },
            ToolboxIdlProgramTypedefPrimitiveKind::String => {
                let data_length =
                    idl_u32_from_bytes_at(data, data_offset, context)?;
                let mut data_size = size_of_val(&data_length);
                let data_bytes = idl_slice_from_bytes(
                    data,
                    data_offset + data_size,
                    usize::try_from(data_length).map_err(|err| {
                        ToolboxIdlError::InvalidInteger {
                            conversion: err,
                            context: context.clone(),
                        }
                    })?,
                    context,
                )?;
                data_size += data_bytes.len();
                let data_string = String::from_utf8(data_bytes.to_vec())
                    .map_err(|err| {
                        ToolboxIdlError::InvalidString {
                            parsing: err,
                            context: context.clone(),
                        }
                    })?;
                (data_size, Value::String(data_string))
            },
            ToolboxIdlProgramTypedefPrimitiveKind::PublicKey => {
                let data_pubkey =
                    idl_pubkey_from_bytes_at(data, data_offset, context)?;
                let data_size = size_of_val(&data_pubkey);
                (data_size, Value::String(data_pubkey.to_string()))
            },
        })
    }
}
