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
        if let Some(idl_type_defined) = idl_type_object.get("defined") {
            return idl_type_value_write_data_defined(
                data,
                value,
                idl_type_defined,
                idl_types,
            );
        }
        if let Some(idl_type_option) = idl_type_object.get("option") {
            return idl_type_value_write_data_option(
                data,
                value,
                idl_type_option,
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
        if let Some(idl_type_vec) = idl_type_object.get("vec") {
            return idl_type_value_write_data_vec(
                data,
                value,
                idl_type_vec,
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
    idl_type_defined: &Value,
    idl_types: &Map<String, Value>,
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
    return idl_type_value_write_data(data, value, idl_type, idl_types);
}

fn idl_type_value_write_data_option(
    data: &mut Vec<u8>,
    value: &Value,
    idl_type_option: &Value,
    idl_types: &Map<String, Value>,
) -> Result<(), ToolboxIdlError> {
    if value.is_null() {
        data.extend_from_slice(bytemuck::bytes_of::<u8>(&0));
        Ok(())
    } else {
        data.extend_from_slice(bytemuck::bytes_of::<u8>(&1));
        idl_type_value_write_data(data, value, idl_type_option, idl_types)
    }
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
    Ok(())
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
    for index in 0..value_array.len() {
        let value_item = value_array.get(index).unwrap();
        idl_type_value_write_data(data, value_item, idl_item_type, idl_types)?;
    }
    Ok(())
}

fn idl_type_value_write_data_vec(
    data: &mut Vec<u8>,
    value: &Value,
    idl_type_vec: &Value,
    idl_types: &Map<String, Value>,
) -> Result<(), ToolboxIdlError> {
    let value_array = idl_as_array_or_else(value, "value?")?; // TODO - better context string recursive handling
    let value_count = u32::try_from(value_array.len())
        .map_err(ToolboxIdlError::TryFromInt)?;
    data.extend_from_slice(bytemuck::bytes_of::<u32>(&value_count));
    for index in 0..value_array.len() {
        let value_item = value_array.get(index).unwrap();
        idl_type_value_write_data(data, value_item, idl_type_vec, idl_types)?;
    }
    return Ok(());
}

fn idl_type_value_write_data_leaf(
    data: &mut Vec<u8>,
    value: &Value,
    idl_type_str: &str,
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
