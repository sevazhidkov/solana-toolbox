use std::str::FromStr;

use serde_json::Map;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_as_array_or_else;
use crate::toolbox_idl_utils::idl_as_bool_or_else;
use crate::toolbox_idl_utils::idl_as_i128_or_else;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_as_str_or_else;
use crate::toolbox_idl_utils::idl_as_u128_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_array_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;

impl ToolboxIdl {
    pub fn type_serialize(
        &self,
        idl_type: &Value,
        value: &Value,
        data: &mut Vec<u8>,
    ) -> Result<(), ToolboxIdlError> {
        idl_type_serialize(&self.types, idl_type, value, data)
    }
}

pub fn idl_type_serialize(
    idl_types: &Map<String, Value>,
    idl_type: &Value,
    value: &Value,
    data: &mut Vec<u8>,
) -> Result<(), ToolboxIdlError> {
    if let Some(idl_type_object) = idl_type.as_object() {
        if let Some(idl_type_defined) = idl_type_object.get("defined") {
            return idl_type_serialize_defined(
                idl_types,
                idl_type_defined,
                value,
                data,
            );
        }
        if let Some(idl_type_option) = idl_type_object.get("option") {
            return idl_type_serialize_option(
                idl_types,
                idl_type_option,
                value,
                data,
            );
        }
        if let Some(idl_type_kind) =
            idl_object_get_key_as_str(idl_type_object, "kind")
        {
            if idl_type_kind == "struct" {
                return idl_type_serialize_struct(
                    idl_types,
                    idl_type_object,
                    value,
                    data,
                );
            }
        }
        if let Some(idl_type_array) =
            idl_object_get_key_as_array(idl_type_object, "array")
        {
            return idl_type_serialize_array(
                idl_types,
                idl_type_array,
                value,
                data,
            );
        }
        if let Some(idl_type_vec) = idl_type_object.get("vec") {
            return idl_type_serialize_vec(
                idl_types,
                idl_type_vec,
                value,
                data,
            );
        }
        return idl_err(&format!(
            "type object is unknown: {:?}",
            idl_type_object
        ));
    }
    if let Some(idl_type_str) = idl_type.as_str() {
        return idl_type_serialize_leaf(idl_type_str, value, data);
    }
    idl_err(&format!("type is unsupported: {:?}", idl_type))
}

pub fn idl_type_serialize_defined(
    idl_types: &Map<String, Value>,
    idl_type_defined: &Value,
    value: &Value,
    data: &mut Vec<u8>,
) -> Result<(), ToolboxIdlError> {
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
    return idl_type_serialize(idl_types, idl_type, value, data);
}

pub fn idl_type_serialize_option(
    idl_types: &Map<String, Value>,
    idl_type_option: &Value,
    value: &Value,
    data: &mut Vec<u8>,
) -> Result<(), ToolboxIdlError> {
    if value.is_null() {
        data.extend_from_slice(bytemuck::bytes_of::<u8>(&0));
        Ok(())
    } else {
        data.extend_from_slice(bytemuck::bytes_of::<u8>(&1));
        idl_type_serialize(idl_types, idl_type_option, value, data)
    }
}

pub fn idl_type_serialize_struct(
    idl_types: &Map<String, Value>,
    idl_type_struct: &Map<String, Value>,
    value: &Value,
    data: &mut Vec<u8>,
) -> Result<(), ToolboxIdlError> {
    let value_object = idl_as_object_or_else(value, "type 'struct'")?; // TODO - better context string recursive handling
    let idl_type_fields = idl_object_get_key_as_array_or_else(
        idl_type_struct,
        "fields",
        "type 'struct'",
    )?;
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
        let value_field = idl_object_get_key_or_else(
            value_object,
            idl_field_name,
            "value", // TODO - better error message
        )?;
        idl_type_serialize(idl_types, idl_field_type, value_field, data)?;
    }
    Ok(())
}

pub fn idl_type_serialize_array(
    idl_types: &Map<String, Value>,
    idl_type_array: &Vec<Value>,
    value: &Value,
    data: &mut Vec<u8>,
) -> Result<(), ToolboxIdlError> {
    let value_array = idl_as_array_or_else(value, "value?")?; // TODO - better context string recursive handling
    if idl_type_array.len() != 2 {
        return idl_err(&format!(
            "type array is malformed: {:?}",
            idl_type_array
        ));
    }
    let idl_item_type = &idl_type_array[0];
    let idl_item_length =
        idl_as_u128_or_else(&idl_type_array[1], "type array length")?;
    let idl_item_length = usize::try_from(idl_item_length)
        .map_err(ToolboxIdlError::TryFromInt)?;
    if value_array.len() != idl_item_length {
        return idl_err(&format!(
            "array is not the correct size: expected {} items, found {} items",
            idl_item_length,
            value_array.len()
        ));
    }
    for index in 0..value_array.len() {
        let value_item = value_array.get(index).unwrap();
        idl_type_serialize(idl_types, idl_item_type, value_item, data)?;
    }
    Ok(())
}

pub fn idl_type_serialize_vec(
    idl_types: &Map<String, Value>,
    idl_type_vec: &Value,
    value: &Value,
    data: &mut Vec<u8>,
) -> Result<(), ToolboxIdlError> {
    let value_array = idl_as_array_or_else(value, "value?")?; // TODO - better context string recursive handling
    let value_count = u32::try_from(value_array.len())
        .map_err(ToolboxIdlError::TryFromInt)?;
    data.extend_from_slice(bytemuck::bytes_of::<u32>(&value_count));
    for index in 0..value_array.len() {
        let value_item = value_array.get(index).unwrap();
        idl_type_serialize(idl_types, idl_type_vec, value_item, data)?;
    }
    return Ok(());
}

pub fn idl_type_serialize_leaf(
    idl_type_str: &str,
    value: &Value,
    data: &mut Vec<u8>,
) -> Result<(), ToolboxIdlError> {
    macro_rules! write_data_using_u_number {
        ($type:ident) => {
            let value_integer = idl_as_u128_or_else(value, "value")?; // TODO - better error
            let value_typed = $type::try_from(value_integer)
                .map_err(ToolboxIdlError::TryFromInt)?;
            data.extend_from_slice(bytemuck::bytes_of::<$type>(&value_typed));
        }
    }
    macro_rules! write_data_using_i_number {
        ($type:ident) => {
            let value_integer = idl_as_i128_or_else(value, "value")?; // TODO - better error
            let value_typed = $type::try_from(value_integer)
                .map_err(ToolboxIdlError::TryFromInt)?;
            data.extend_from_slice(bytemuck::bytes_of::<$type>(&value_typed));
        }
    }
    if idl_type_str == "u8" {
        write_data_using_u_number!(u8);
        return Ok(());
    }
    if idl_type_str == "i8" {
        write_data_using_i_number!(i8);
        return Ok(());
    }
    if idl_type_str == "u16" {
        write_data_using_u_number!(u16);
        return Ok(());
    }
    if idl_type_str == "i16" {
        write_data_using_i_number!(i16);
        return Ok(());
    }
    if idl_type_str == "u32" {
        write_data_using_u_number!(u32);
        return Ok(());
    }
    if idl_type_str == "i32" {
        write_data_using_i_number!(i32);
        return Ok(());
    }
    if idl_type_str == "u64" {
        write_data_using_u_number!(u64);
        return Ok(());
    }
    if idl_type_str == "i64" {
        write_data_using_i_number!(i64);
        return Ok(());
    }
    if idl_type_str == "u128" {
        let value_integer = idl_as_u128_or_else(value, "value")?; // TODO - better error
        data.extend_from_slice(bytemuck::bytes_of::<u128>(&value_integer));
        return Ok(());
    }
    if idl_type_str == "i128" {
        let value_integer = idl_as_i128_or_else(value, "value")?; // TODO - better error
        data.extend_from_slice(bytemuck::bytes_of::<i128>(&value_integer));
        return Ok(());
    }
    if idl_type_str == "bool" {
        // TODO - better error
        let value_flag =
            if idl_as_bool_or_else(value, "value")? { 1 } else { 0 };
        data.extend_from_slice(bytemuck::bytes_of::<u8>(&value_flag));
        return Ok(());
    }
    if idl_type_str == "string" {
        let value_str = idl_as_str_or_else(value, "value")?; // TODO - better error
        let value_length = u32::try_from(value_str.len())
            .map_err(ToolboxIdlError::TryFromInt)?;
        data.extend_from_slice(bytemuck::bytes_of::<u32>(&value_length));
        data.extend_from_slice(value_str.as_bytes());
        return Ok(());
    }
    if idl_type_str == "pubkey" || idl_type_str == "publicKey" {
        let value_str = idl_as_str_or_else(value, "value")?; // TODO - better error handling
        let value_pubkey = Pubkey::from_str(value_str)
            .map_err(ToolboxIdlError::ParsePubkey)?;
        data.extend_from_slice(bytemuck::bytes_of::<Pubkey>(&value_pubkey));
        return Ok(());
    }
    return idl_err(&format!(
        "type 'string': unknown type descriptor: {}",
        idl_type_str,
    ));
}
