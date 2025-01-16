use serde_json::Map;
use serde_json::Number;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_as_u128_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_array_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;
use crate::toolbox_idl_utils::idl_slice_from_bytes;
use crate::toolbox_idl_utils::idl_type_from_bytes_at;

impl ToolboxIdl {
    pub fn type_deserialize(
        &self,
        idl_type: &Value,
        data: &[u8],
        data_offset: usize,
    ) -> Result<(usize, Value), ToolboxIdlError> {
        idl_type_deserialize(&self.types, idl_type, data, data_offset)
    }
}

fn idl_type_deserialize(
    idl_types: &Map<String, Value>,
    idl_type: &Value,
    data: &[u8],
    data_offset: usize,
) -> Result<(usize, Value), ToolboxIdlError> {
    if let Some(idl_type_object) = idl_type.as_object() {
        if let Some(idl_type_defined) = idl_type_object.get("defined") {
            return idl_type_deserialize_defined(
                idl_types,
                idl_type_defined,
                data,
                data_offset,
            );
        }
        if let Some(idl_type_option) = idl_type_object.get("option") {
            return idl_type_deserialize_option(
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
                return idl_type_deserialize_struct(
                    idl_types,
                    idl_type_object,
                    data,
                    data_offset,
                );
            }
        }
        if let Some(idl_type_array) =
            idl_object_get_key_as_array(idl_type_object, "array")
        {
            return idl_type_deserialize_array(
                idl_types,
                idl_type_array,
                data,
                data_offset,
            );
        }
        if let Some(idl_type_vec) = idl_type_object.get("vec") {
            return idl_type_deserialize_vec(
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
        return idl_type_deserialize_leaf(idl_type_str, data, data_offset);
    }
    idl_err(&format!("type is unsupported: {:?}", idl_type))
}

fn idl_type_deserialize_defined(
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
    return idl_type_deserialize(idl_types, idl_type, data, data_offset);
}

fn idl_type_deserialize_option(
    idl_types: &Map<String, Value>,
    idl_type_option: &Value,
    data: &[u8],
    data_offset: usize,
) -> Result<(usize, Value), ToolboxIdlError> {
    let data_flag = *idl_type_from_bytes_at::<u8>(data, data_offset)?;
    let mut data_size = size_of_val(&data_flag);
    if data_flag > 0 {
        let (data_content_size, data_content_value) = idl_type_deserialize(
            idl_types,
            idl_type_option,
            data,
            data_offset + 1,
        )?;
        data_size += data_content_size;
        Ok((data_size, data_content_value))
    } else {
        Ok((data_size, Value::Null))
    }
}

fn idl_type_deserialize_struct(
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
        let (data_field_size, data_field_value) = idl_type_deserialize(
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

fn idl_type_deserialize_array(
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
        let (data_item_size, data_item_value) = idl_type_deserialize(
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
fn idl_type_deserialize_vec(
    idl_types: &Map<String, Value>,
    idl_type_vec: &Value,
    data: &[u8],
    data_offset: usize,
) -> Result<(usize, Value), ToolboxIdlError> {
    let data_count = *idl_type_from_bytes_at::<u32>(data, data_offset)?;
    let mut data_size = size_of_val(&data_count);
    let mut data_items = vec![];
    for _index in
        0..usize::try_from(data_count).map_err(ToolboxIdlError::TryFromInt)?
    {
        let (data_item_size, data_item_value) = idl_type_deserialize(
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

fn idl_type_deserialize_leaf(
    idl_type_str: &str,
    data: &[u8],
    data_offset: usize,
) -> Result<(usize, Value), ToolboxIdlError> {
    macro_rules! number_from_data_as_integer {
        ($type:ident) => {{
            let data_int = *idl_type_from_bytes_at::<$type>(data, data_offset)?;
            let data_size = size_of_val(&data_int);
            Ok((data_size, Value::Number(Number::from(data_int))))
        }};
    }
    if idl_type_str == "u8" {
        return number_from_data_as_integer!(u8);
    }
    if idl_type_str == "i8" {
        return number_from_data_as_integer!(i8);
    }
    if idl_type_str == "u16" {
        return number_from_data_as_integer!(u16);
    }
    if idl_type_str == "i16" {
        return number_from_data_as_integer!(i16);
    }
    if idl_type_str == "u32" {
        return number_from_data_as_integer!(u32);
    }
    if idl_type_str == "i32" {
        return number_from_data_as_integer!(i32);
    }
    if idl_type_str == "u64" {
        return number_from_data_as_integer!(u64);
    }
    if idl_type_str == "i64" {
        return number_from_data_as_integer!(i64);
    }
    macro_rules! number_from_converted_data_integer {
        ($type:ident, $conversion:ident) => {{
            let data_int = *idl_type_from_bytes_at::<$type>(data, data_offset)?;
            let data_size = size_of_val(&data_int);
            Ok((
                data_size,
                Value::Number($conversion(data_int).ok_or_else(|| {
                    ToolboxIdlError::Custom(format!(
                        "JSON Invalid number: {}",
                        data_int
                    ))
                })?),
            ))
        }};
    }
    if idl_type_str == "u128" {
        fn number_from_u128(integer: u128) -> Option<Number> {
            Number::from_u128(integer)
        }
        return number_from_converted_data_integer!(u128, number_from_u128);
    }
    if idl_type_str == "i128" {
        fn number_from_i128(integer: i128) -> Option<Number> {
            Number::from_i128(integer)
        }
        return number_from_converted_data_integer!(i128, number_from_i128);
    }
    if idl_type_str == "bool" {
        let data_flag = *idl_type_from_bytes_at::<u8>(data, data_offset)?;
        let data_size = size_of_val(&data_flag);
        return Ok((
            data_size,
            Value::Bool(if data_flag == 0 { false } else { true }),
        ));
    }
    // TODO - this needs to be tested with on-chain accounts
    if idl_type_str == "string" {
        let data_length = *idl_type_from_bytes_at::<u32>(data, data_offset)?;
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
        let data_pubkey = *idl_type_from_bytes_at::<Pubkey>(data, data_offset)?;
        let data_size = size_of_val(&data_pubkey);
        return Ok((data_size, Value::String(data_pubkey.to_string())));
    }
    return idl_err(&format!(
        "type 'string': unknown type descriptor: {}",
        idl_type_str,
    ));
}
