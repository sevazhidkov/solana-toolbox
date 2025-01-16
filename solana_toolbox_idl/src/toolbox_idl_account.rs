use serde_json::Map;
use serde_json::Number;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

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
    pub async fn get_account(
        &self,
        endpoint: &mut ToolboxEndpoint,
        account_type: &str,
        account_address: &Pubkey,
    ) -> Result<Option<(usize, Value)>, ToolboxIdlError> {
        let account_data = match endpoint.get_account(account_address).await? {
            Some(account) => account.data,
            None => return Ok(None),
        };
        Ok(Some(self.read_account(account_type, &account_data)?))
    }

    pub fn read_account(
        &self,
        account_type: &str,
        account_data: &[u8],
    ) -> Result<(usize, Value), ToolboxIdlError> {
        let idl_type = match self.account_types.get(account_type) {
            Some(idl_account_type) => idl_account_type,
            None => {
                idl_object_get_key_or_else(&self.types, account_type, "types")?
            },
        };
        let data_discriminator =
            *idl_type_from_bytes_at::<u64>(account_data, 0)?;
        let expected_discriminator =
            ToolboxIdl::compute_account_discriminator(account_type);
        if data_discriminator != expected_discriminator {
            return idl_err(&format!(
                "invalid discriminator: found {:016X}, expected {:016X}",
                data_discriminator, expected_discriminator
            ));
        }
        let data_offset = size_of_val(&data_discriminator);
        let (data_length, data_json) = idl_type_data_read_value(
            &account_data[data_offset..],
            idl_type,
            &self.types,
        )?;
        Ok((data_offset + data_length, data_json))
    }
}

fn idl_type_data_read_value(
    data: &[u8],
    idl_type: &Value,
    idl_types: &Map<String, Value>,
) -> Result<(usize, Value), ToolboxIdlError> {
    if let Some(idl_type_object) = idl_type.as_object() {
        if let Some(idl_type_defined) = idl_type_object.get("defined") {
            return idl_type_data_read_value_defined(
                data,
                idl_type_defined,
                idl_types,
            );
        }
        if let Some(idl_type_option) = idl_type_object.get("option") {
            return idl_type_data_read_value_option(
                data,
                idl_type_option,
                idl_types,
            );
        }
        if let Some(idl_type_kind) =
            idl_object_get_key_as_str(idl_type_object, "kind")
        {
            if idl_type_kind == "struct" {
                return idl_type_data_read_value_struct(
                    data,
                    idl_type_object,
                    idl_types,
                );
            }
        }
        if let Some(idl_type_array) =
            idl_object_get_key_as_array(idl_type_object, "array")
        {
            return idl_type_data_read_value_array(
                data,
                idl_type_array,
                idl_types,
            );
        }
        if let Some(idl_type_vec) = idl_type_object.get("vec") {
            return idl_type_data_read_value_vec(data, idl_type_vec, idl_types);
        }
        return idl_err(&format!(
            "type object is unknown: {:?}",
            idl_type_object
        ));
    }
    if let Some(idl_type_str) = idl_type.as_str() {
        return idl_type_data_read_value_leaf(data, idl_type_str);
    }
    idl_err(&format!("type is unsupported: {:?}", idl_type))
}

fn idl_type_data_read_value_defined(
    data: &[u8],
    idl_type_defined: &Value,
    idl_types: &Map<String, Value>,
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
    return idl_type_data_read_value(data, idl_type, idl_types);
}

fn idl_type_data_read_value_option(
    data: &[u8],
    idl_type_option: &Value,
    idl_types: &Map<String, Value>,
) -> Result<(usize, Value), ToolboxIdlError> {
    let flag = *idl_type_from_bytes_at::<u8>(data, 0)?;
    if flag > 0 {
        let (data_content_length, data_content_value) =
            idl_type_data_read_value(&data[1..], idl_type_option, idl_types)?;
        Ok((1 + data_content_length, data_content_value))
    } else {
        Ok((1, Value::Null))
    }
}

fn idl_type_data_read_value_struct(
    data: &[u8],
    idl_type_struct: &Map<String, Value>,
    idl_types: &Map<String, Value>,
) -> Result<(usize, Value), ToolboxIdlError> {
    let mut data_length = 0;
    let mut data_fields = Map::new();
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
        let (data_field_length, data_field_value) = idl_type_data_read_value(
            &data[data_length..],
            idl_field_type,
            idl_types,
        )?;
        data_length += data_field_length;
        data_fields.insert(idl_field_name.to_string(), data_field_value);
    }
    return Ok((data_length, Value::Object(data_fields)));
}

fn idl_type_data_read_value_array(
    data: &[u8],
    idl_type_array: &Vec<Value>,
    idl_types: &Map<String, Value>,
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
    let mut data_length = 0;
    let mut data_items = vec![];
    for _index in 0..idl_item_length {
        let (data_item_length, data_item_value) = idl_type_data_read_value(
            &data[data_length..],
            idl_item_type,
            idl_types,
        )?;
        data_length += data_item_length;
        data_items.push(data_item_value);
    }
    return Ok((data_length, Value::Array(data_items)));
}

// TODO - this needs to be tested on on-chain accounts
fn idl_type_data_read_value_vec(
    data: &[u8],
    idl_type_vec: &Value,
    idl_types: &Map<String, Value>,
) -> Result<(usize, Value), ToolboxIdlError> {
    let data_count = *idl_type_from_bytes_at::<u32>(data, 0)?;
    let mut data_length = 4;
    let mut data_items = vec![];
    for _index in
        0..usize::try_from(data_count).map_err(ToolboxIdlError::TryFromInt)?
    {
        let (data_item_length, data_item_value) = idl_type_data_read_value(
            &data[data_length..],
            idl_type_vec,
            idl_types,
        )?;
        data_length += data_item_length;
        data_items.push(data_item_value);
    }
    return Ok((data_length, Value::Array(data_items)));
}

fn idl_type_data_read_value_leaf(
    data: &[u8],
    idl_type_str: &str,
) -> Result<(usize, Value), ToolboxIdlError> {
    macro_rules! number_from_data_as_integer {
        ($type:ident) => {{
            let integer = *idl_type_from_bytes_at::<$type>(data, 0)?;
            Ok((size_of::<$type>(), Value::Number(Number::from(integer))))
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
            let integer = *idl_type_from_bytes_at::<$type>(data, 0)?;
            Ok((
                size_of::<$type>(),
                Value::Number($conversion(integer).ok_or_else(|| {
                    ToolboxIdlError::Custom(format!(
                        "JSON Invalid number: {}",
                        integer
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
        let flag = *idl_type_from_bytes_at::<u8>(data, 0)?;
        return Ok((
            size_of_val(&flag),
            Value::Bool(if flag == 0 { false } else { true }),
        ));
    }
    // TODO - this needs to be tested with on-chain accounts
    if idl_type_str == "string" {
        let length = *idl_type_from_bytes_at::<u32>(data, 0)?;
        let utf8_encoded = idl_slice_from_bytes(
            data,
            4,
            usize::try_from(length).map_err(ToolboxIdlError::TryFromInt)?,
        )?;
        return Ok((
            size_of_val(&length) + size_of_val(utf8_encoded),
            Value::String(
                String::from_utf8(utf8_encoded.to_vec())
                    .map_err(ToolboxIdlError::FromUtf8)?,
            ),
        ));
    }
    if idl_type_str == "pubkey" || idl_type_str == "publicKey" {
        let pubkey = *idl_type_from_bytes_at::<Pubkey>(data, 0)?;
        return Ok((size_of_val(&pubkey), Value::String(pubkey.to_string())));
    }
    return idl_err(&format!(
        "type 'string': unknown type descriptor: {}",
        idl_type_str,
    ));
}
