use serde_json::Map;
use serde_json::Number;
use serde_json::Value;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_as_u128_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_i128_from_bytes_at;
use crate::toolbox_idl_utils::idl_i16_from_bytes_at;
use crate::toolbox_idl_utils::idl_i32_from_bytes_at;
use crate::toolbox_idl_utils::idl_i64_from_bytes_at;
use crate::toolbox_idl_utils::idl_i8_from_bytes_at;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_array_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;
use crate::toolbox_idl_utils::idl_pubkey_from_bytes_at;
use crate::toolbox_idl_utils::idl_slice_from_bytes;
use crate::toolbox_idl_utils::idl_u128_from_bytes_at;
use crate::toolbox_idl_utils::idl_u16_from_bytes_at;
use crate::toolbox_idl_utils::idl_u32_from_bytes_at;
use crate::toolbox_idl_utils::idl_u64_from_bytes_at;
use crate::toolbox_idl_utils::idl_u8_from_bytes_at;

impl ToolboxIdl {
    pub fn type_reader(
        &self,
        idl_type: &Value,
        data: &[u8],
        data_offset: usize,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        idl_type_reader(&self.types, idl_type, data, data_offset)
    }
}

fn idl_type_reader(
    idl_types: &Map<String, Value>,
    idl_type: &Value,
    data: &[u8],
    data_offset: usize,
) -> Result<(usize, Value), ToolboxIdlError> {
    if let Some(idl_type_object) = idl_type.as_object() {
        if let Some(idl_type_defined) = idl_type_object.get("defined") {
            return idl_type_reader_defined(
                idl_types,
                idl_type_defined,
                data,
                data_offset,
            );
        }
        if let Some(idl_type_option) = idl_type_object.get("option") {
            return idl_type_reader_option(
                idl_types,
                idl_type_option,
                data,
                data_offset,
            );
        }
        if let Some(idl_type_kind) =
            idl_object_get_key_as_str(idl_type_object, "kind")
        {
            if idl_type_kind == "struct" {
                return idl_type_reader_struct(
                    idl_types,
                    idl_type_object,
                    data,
                    data_offset,
                );
            }
            if idl_type_kind == "enum" {
                return idl_type_reader_enum(
                    idl_type_object,
                    data,
                    data_offset,
                );
            }
        }
        if let Some(idl_type_array) =
            idl_object_get_key_as_array(idl_type_object, "array")
        {
            return idl_type_reader_array(
                idl_types,
                idl_type_array,
                data,
                data_offset,
            );
        }
        if let Some(idl_type_vec) = idl_type_object.get("vec") {
            return idl_type_reader_vec(
                idl_types,
                idl_type_vec,
                data,
                data_offset,
            );
        }
        return idl_err(&format!(
            "type object is unknown: {:?}",
            idl_type_object
        ));
    }
    if let Some(idl_type_str) = idl_type.as_str() {
        return idl_type_reader_leaf(idl_type_str, data, data_offset);
    }
    idl_err(&format!("type is unsupported: {:?}", idl_type))
}

fn idl_type_reader_defined(
    idl_types: &Map<String, Value>,
    idl_type_defined: &Value,
    data: &[u8],
    data_offset: usize,
) -> Result<(usize, Value), ToolboxIdlError> {
    let idl_type_name = match idl_type_defined.as_str() {
        Some(idl_type_name) => idl_type_name,
        None => {
            let idl_type_defined_object =
                idl_as_object_or_else(idl_type_defined, "type defined")?;
            idl_object_get_key_as_str_or_else(
                idl_type_defined_object,
                "name",
                "type defined name",
            )?
        },
    };
    let idl_type = idl_object_get_key_or_else(
        idl_types,
        idl_type_name,
        "type definitions",
    )?;
    return idl_type_reader(idl_types, idl_type, data, data_offset);
}

fn idl_type_reader_option(
    idl_types: &Map<String, Value>,
    idl_type_option: &Value,
    data: &[u8],
    data_offset: usize,
) -> Result<(usize, Value), ToolboxIdlError> {
    let data_flag = idl_u8_from_bytes_at(data, data_offset)?;
    let mut data_size = size_of_val(&data_flag);
    if data_flag > 0 {
        let (data_content_size, data_content_value) =
            idl_type_reader(idl_types, idl_type_option, data, data_offset + 1)?;
        data_size += data_content_size;
        Ok((data_size, data_content_value))
    } else {
        Ok((data_size, Value::Null))
    }
}

fn idl_type_reader_struct(
    idl_types: &Map<String, Value>,
    idl_type_struct: &Map<String, Value>,
    data: &[u8],
    data_offset: usize,
) -> Result<(usize, Value), ToolboxIdlError> {
    let idl_type_fields = idl_object_get_key_as_array_or_else(
        idl_type_struct,
        "fields",
        "type 'struct'",
    )?;
    let mut data_size = 0;
    let mut data_fields = Map::new();
    for idl_field in idl_type_fields {
        let idl_field_object =
            idl_as_object_or_else(idl_field, "type 'struct' field")?;
        let idl_field_name = idl_object_get_key_as_str_or_else(
            idl_field_object,
            "name",
            "type 'struct' field",
        )?;
        let idl_field_type = idl_object_get_key_or_else(
            idl_field_object,
            "type",
            "type 'struct' field",
        )?;
        let (data_field_size, data_field_value) = idl_type_reader(
            idl_types,
            idl_field_type,
            data,
            data_offset + data_size,
        )?;
        data_size += data_field_size;
        data_fields.insert(idl_field_name.to_string(), data_field_value);
    }
    return Ok((data_size, Value::Object(data_fields)));
}

fn idl_type_reader_enum(
    idl_type_enum: &Map<String, Value>,
    data: &[u8],
    data_offset: usize,
) -> Result<(usize, Value), ToolboxIdlError> {
    let idl_type_variants = idl_object_get_key_as_array_or_else(
        idl_type_enum,
        "variants",
        "enum variants",
    )?;
    let data_enum = idl_u8_from_bytes_at(data, data_offset)?;
    let data_index = usize::from(data_enum);
    if data_index >= idl_type_variants.len() {
        return idl_err(&format!(
            "Invalid enum value: {}: {:?}",
            data_index, idl_type_variants
        ));
    }
    let idl_type_variant_object = idl_as_object_or_else(
        idl_type_variants.get(data_index).unwrap(),
        "enum variant",
    )?;
    let idl_type_variant_name = idl_object_get_key_as_str_or_else(
        idl_type_variant_object,
        "name",
        "enum variant name",
    )?;
    Ok((size_of_val(&data_enum), Value::String(idl_type_variant_name.into())))
}

fn idl_type_reader_array(
    idl_types: &Map<String, Value>,
    idl_type_array: &Vec<Value>,
    data: &[u8],
    data_offset: usize,
) -> Result<(usize, Value), ToolboxIdlError> {
    if idl_type_array.len() != 2 {
        return idl_err(&format!(
            "type array is malformed: {:?}",
            idl_type_array
        ));
    }
    let idl_item_type = &idl_type_array[0];
    let idl_item_length =
        idl_as_u128_or_else(&idl_type_array[1], "type array length")?;
    let mut data_size = 0;
    let mut data_items = vec![];
    for _index in 0..idl_item_length {
        let (data_item_size, data_item_value) = idl_type_reader(
            idl_types,
            idl_item_type,
            data,
            data_offset + data_size,
        )?;
        data_size += data_item_size;
        data_items.push(data_item_value);
    }
    return Ok((data_size, Value::Array(data_items)));
}

// TODO - this needs to be tested on on-chain accounts
fn idl_type_reader_vec(
    idl_types: &Map<String, Value>,
    idl_type_vec: &Value,
    data: &[u8],
    data_offset: usize,
) -> Result<(usize, Value), ToolboxIdlError> {
    let data_length = idl_u32_from_bytes_at(data, data_offset)?;
    let mut data_size = size_of_val(&data_length);
    let mut data_items = vec![];
    for _index in
        0..usize::try_from(data_length).map_err(ToolboxIdlError::TryFromInt)?
    {
        let (data_item_size, data_item_value) = idl_type_reader(
            idl_types,
            idl_type_vec,
            data,
            data_offset + data_size,
        )?;
        data_size += data_item_size;
        data_items.push(data_item_value);
    }
    return Ok((data_size, Value::Array(data_items)));
}

fn idl_type_reader_leaf(
    idl_type_str: &str,
    data: &[u8],
    data_offset: usize,
) -> Result<(usize, Value), ToolboxIdlError> {
    if idl_type_str == "u8" {
        let int = idl_u8_from_bytes_at(data, data_offset)?;
        return Ok((size_of_val(&int), Value::Number(Number::from(int))));
    }
    if idl_type_str == "i8" {
        let int = idl_i8_from_bytes_at(data, data_offset)?;
        return Ok((size_of_val(&int), Value::Number(Number::from(int))));
    }
    if idl_type_str == "u16" {
        let int = idl_u16_from_bytes_at(data, data_offset)?;
        return Ok((size_of_val(&int), Value::Number(Number::from(int))));
    }
    if idl_type_str == "i16" {
        let int = idl_i16_from_bytes_at(data, data_offset)?;
        return Ok((size_of_val(&int), Value::Number(Number::from(int))));
    }
    if idl_type_str == "u32" {
        let int = idl_u32_from_bytes_at(data, data_offset)?;
        return Ok((size_of_val(&int), Value::Number(Number::from(int))));
    }
    if idl_type_str == "i32" {
        let int = idl_i32_from_bytes_at(data, data_offset)?;
        return Ok((size_of_val(&int), Value::Number(Number::from(int))));
    }
    if idl_type_str == "u64" {
        let int = idl_u64_from_bytes_at(data, data_offset)?;
        return Ok((size_of_val(&int), Value::Number(Number::from(int))));
    }
    if idl_type_str == "i64" {
        let int = idl_i64_from_bytes_at(data, data_offset)?;
        return Ok((size_of_val(&int), Value::Number(Number::from(int))));
    }
    if idl_type_str == "u128" {
        let int = idl_u128_from_bytes_at(data, data_offset)?;
        return Ok((
            size_of_val(&int),
            Value::Number(Number::from_u128(int).unwrap_or(Number::from(0))),
        ));
    }
    if idl_type_str == "i128" {
        let int = idl_i128_from_bytes_at(data, data_offset)?;
        return Ok((
            size_of_val(&int),
            Value::Number(Number::from_i128(int).unwrap_or(Number::from(0))),
        ));
    }
    if idl_type_str == "bool" {
        let data_flag = idl_u8_from_bytes_at(data, data_offset)?;
        let data_size = size_of_val(&data_flag);
        return Ok((
            data_size,
            Value::Bool(if data_flag == 0 { false } else { true }),
        ));
    }
    // TODO - this needs to be tested with on-chain accounts
    if idl_type_str == "string" {
        let data_length = idl_u32_from_bytes_at(data, data_offset)?;
        let mut data_size = size_of_val(&data_length);
        let data_string = String::from_utf8(
            idl_slice_from_bytes(
                data,
                data_offset + data_size,
                usize::try_from(data_length)
                    .map_err(ToolboxIdlError::TryFromInt)?,
            )?
            .to_vec(),
        )
        .map_err(ToolboxIdlError::FromUtf8)?;
        data_size += data_string.len();
        return Ok((data_size, Value::String(data_string)));
    }
    if idl_type_str == "pubkey" || idl_type_str == "publicKey" {
        let data_pubkey = idl_pubkey_from_bytes_at(data, data_offset)?;
        let data_size = size_of_val(&data_pubkey);
        return Ok((data_size, Value::String(data_pubkey.to_string())));
    }
    return idl_err(&format!(
        "type 'string': unknown type descriptor: {}",
        idl_type_str,
    ));
}
