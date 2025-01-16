use std::str::FromStr;

use serde_json::Map;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_as_array_or_else;
use crate::toolbox_idl_utils::idl_as_i128_or_else;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_as_str_or_else;
use crate::toolbox_idl_utils::idl_as_u128_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_array_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_object;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;
use crate::toolbox_idl_utils::idl_ok_or_else;

impl ToolboxIdl {
    pub fn generate_instruction_data(
        &self,
        instruction_name: &str,
        instruction_args: &Map<String, Value>,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        let mut data: Vec<u8> = vec![];
        data.extend_from_slice(
            &ToolboxIdl::compute_instruction_discriminator(instruction_name)
                .to_le_bytes(),
        );
        let idl_instruction_args = idl_object_get_key_as_array_or_else(
            &self.instructions_args,
            instruction_name,
            "instructions args",
        )?;
        for idl_instruction_arg in idl_instruction_args {
            println!("idl_instruction_arg:{:#?}", idl_instruction_arg);
            let idl_instruction_arg_object =
                idl_as_object_or_else(idl_instruction_arg, "instruction arg")?;
            let idl_instruction_arg_name = idl_object_get_key_as_str_or_else(
                idl_instruction_arg_object,
                "name",
                "instruction arg",
            )?;
            let idl_instruction_arg_type = idl_object_get_key_or_else(
                idl_instruction_arg_object,
                "type",
                "instruction arg",
            )?;
            let instruction_arg = idl_object_get_key_or_else(
                instruction_args,
                idl_instruction_arg_name,
                "instruction params",
            )?;
            idl_type_value_write_data(
                &mut data,
                instruction_arg,
                idl_instruction_arg_type,
                &self.types,
            )?;
        }
        Ok(data)
    }
}

fn idl_type_value_write_data(
    data: &mut Vec<u8>,
    value: &Value,
    idl_type: &Value,
    idl_types: &Map<String, Value>,
) -> Result<(), ToolboxIdlError> {
    if let Some(idl_type_object) = idl_type.as_object() {
        if let Some(idl_type_defined) =
            idl_object_get_key_as_object(idl_type_object, "defined")
        {
            return idl_type_value_write_data_defined(
                data,
                value,
                idl_type_defined,
                idl_types,
            );
        }
        if let Some(idl_type_kind) =
            idl_object_get_key_as_str(idl_type_object, "kind")
        {
            if idl_type_kind == "struct" {
                return idl_type_value_write_data_struct(
                    data,
                    value,
                    idl_type_object,
                    idl_types,
                );
            }
        }
        if let Some(idl_type_array) =
            idl_object_get_key_as_array(idl_type_object, "array")
        {
            return idl_type_value_write_data_array(
                data,
                value,
                idl_type_array,
                idl_types,
            );
        }
        return idl_err(&format!(
            "type object is unknown: {:?}",
            idl_type_object
        ));
    }
    if let Some(idl_type_str) = idl_type.as_str() {
        return idl_type_value_write_data_leaf(data, value, idl_type_str);
    }
    idl_err(&format!("type is unsupported: {:?}", idl_type))
}

fn idl_type_value_write_data_defined(
    data: &mut Vec<u8>,
    value: &Value,
    idl_type_defined: &Map<String, Value>,
    idl_types: &Map<String, Value>,
) -> Result<(), ToolboxIdlError> {
    let idl_type_name = idl_object_get_key_as_str_or_else(
        idl_type_defined,
        "name",
        "type reference as 'defined'",
    )?;
    let idl_type = idl_object_get_key_or_else(
        idl_types,
        idl_type_name,
        "type definitions",
    )?;
    return idl_type_value_write_data(data, value, idl_type, idl_types);
}

fn idl_type_value_write_data_struct(
    data: &mut Vec<u8>,
    value: &Value,
    idl_type_struct: &Map<String, Value>,
    idl_types: &Map<String, Value>,
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
        idl_type_value_write_data(
            data,
            value_field,
            idl_field_type,
            idl_types,
        )?;
    }
    return Ok(());
}

fn idl_type_value_write_data_array(
    data: &mut Vec<u8>,
    value: &Value,
    idl_type_array: &Vec<Value>,
    idl_types: &Map<String, Value>,
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
    for index in 0..idl_item_length {
        let value_item = idl_ok_or_else(
            value_array.get(index),
            "value array",
            "has no item at index",
            &index.to_string(),
            value_array, // TODO - better error message
        )?;
        idl_type_value_write_data(data, value_item, idl_item_type, idl_types)?;
    }
    return Ok(());
}

fn idl_type_value_write_data_leaf(
    data: &mut Vec<u8>,
    value: &Value,
    idl_type_str: &str,
) -> Result<(), ToolboxIdlError> {
    macro_rules! return_with_unsigned_integer {
        ($type:ident) => {
            let value_u128 = idl_as_u128_or_else(value, "value")?; // TODO - better error
            let value_casted = $type::try_from(value_u128)
                .map_err(ToolboxIdlError::TryFromInt)?;
            data.extend_from_slice(bytemuck::bytes_of(&value_casted));
            return Ok(());
        }
    }
    if idl_type_str == "u8" {
        return_with_unsigned_integer!(u8);
    }
    if idl_type_str == "u16" {
        return_with_unsigned_integer!(u16);
    }
    if idl_type_str == "u32" {
        return_with_unsigned_integer!(u32);
    }
    if idl_type_str == "u64" {
        return_with_unsigned_integer!(u64);
    }
    if idl_type_str == "u128" {
        let value_u128 = idl_as_u128_or_else(value, "value")?; // TODO - better error
        data.extend_from_slice(bytemuck::bytes_of(&value_u128));
    }
    macro_rules! return_with_signed_integer {
        ($type:ident) => {
            let value_i128 = idl_as_i128_or_else(value, "value")?; // TODO - better error
            let value_casted = $type::try_from(value_i128)
                .map_err(ToolboxIdlError::TryFromInt)?;
            data.extend_from_slice(bytemuck::bytes_of(&value_casted));
            return Ok(());
        }
    }
    if idl_type_str == "i8" {
        return_with_signed_integer!(i8);
    }
    if idl_type_str == "i16" {
        return_with_signed_integer!(i16);
    }
    if idl_type_str == "i32" {
        return_with_signed_integer!(i32);
    }
    if idl_type_str == "i64" {
        return_with_signed_integer!(i64);
    }
    if idl_type_str == "i128" {
        let value_i128 = idl_as_i128_or_else(value, "value")?; // TODO - better error
        data.extend_from_slice(bytemuck::bytes_of(&value_i128));
        return Ok(());
    }
    if idl_type_str == "pubkey" || idl_type_str == "publicKey" {
        let value_str = idl_as_str_or_else(value, "value")?; // TODO - better error handling
        let value_pubkey = Pubkey::from_str(value_str)
            .map_err(ToolboxIdlError::ParsePubkey)?;
        data.extend_from_slice(bytemuck::bytes_of(&value_pubkey));
        return Ok(());
    }
    return idl_err(&format!(
        "type 'string': unknown type descriptor: {}",
        idl_type_str,
    ));
}
