use serde_json::Map;
use serde_json::Number;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_anchor_endpoint::ToolboxAnchorEndpoint;
use crate::toolbox_anchor_error::ToolboxAnchorError;
use crate::toolbox_anchor_idl::ToolboxAnchorIdl;
use crate::toolbox_anchor_idl_utils::idl_as_object_or_else;
use crate::toolbox_anchor_idl_utils::idl_as_u64_or_else;
use crate::toolbox_anchor_idl_utils::idl_err;
use crate::toolbox_anchor_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_anchor_idl_utils::idl_object_get_key_as_array_or_else;
use crate::toolbox_anchor_idl_utils::idl_object_get_key_as_object;
use crate::toolbox_anchor_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_anchor_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_anchor_idl_utils::idl_object_get_key_or_else;
use crate::toolbox_anchor_idl_utils::idl_ok_or_else;

impl ToolboxIdl {
    pub async fn deserialize_account(
        &self,
        account_data: &[u8],
        account_type: &str,
    ) -> Result<Option<(usize, Value)>, ToolboxAnchorError> {
        let idl_type = idl_ok_or_else(
            self.accounts
                .get(account_type)
                .or_else(|| self.types.get(account_type)),
            "account type",
            "is unknown",
            account_type,
            &self.types,
        )?;
        let data_bytes =
            if let Some(account) = endpoint.get_account(&address).await? {
                account.data
            }
            else {
                return Ok(None);
            };
        let expected_discriminator =
            self.compute_account_discriminator(account_type);
        let data_offset_discriminator = size_of_val(&expected_discriminator);
        let data_discriminator = u64::from_le_bytes(
            data_bytes[..data_offset_discriminator]
                .try_into()
                .map_err(ToolboxAnchorError::TryFromSlice)?,
        );
        if data_discriminator != expected_discriminator {
            return idl_err(
                "discriminator not as expected",
                &format!("{:16X}", data_discriminator),
                &format!("{:16X}", expected_discriminator),
            );
        }
        let (data_length, data_json) = idl_type_data_read(
            &data_bytes[data_offset_discriminator..],
            idl_type,
            &self.types,
        )?;
        Ok(Some((data_length + data_offset_discriminator, data_json)))
    }
}

fn idl_type_data_read(
    data_bytes: &[u8],
    idl_type: &Value,
    idl_types: &Map<String, Value>,
) -> Result<(usize, Value), ToolboxAnchorError> {
    if let Some(idl_type_object) = idl_type.as_object() {
        if let Some(idl_type_kind) =
            idl_object_get_key_as_str(idl_type_object, "kind")
        {
            if idl_type_kind == "struct" {
                let mut data_length = 0;
                let mut data_fields = Map::new();
                let idl_type_fields = idl_object_get_key_as_array_or_else(
                    idl_type_object,
                    "fields",
                    "type 'struct'",
                )?;
                for idl_field in idl_type_fields {
                    let idl_field_object = idl_as_object_or_else(
                        idl_field,
                        "type 'struct' field",
                    )?;
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
                    let (data_field_length, data_field_value) =
                        idl_type_data_read(
                            &data_bytes[data_length..],
                            idl_field_type,
                            idl_types,
                        )?;
                    data_length += data_field_length;
                    data_fields
                        .insert(idl_field_name.to_string(), data_field_value);
                }
                return Ok((data_length, Value::Object(data_fields)));
            }
        }
        if let Some(idl_type_array) =
            idl_object_get_key_as_array(idl_type_object, "array")
        {
            if idl_type_array.len() != 2 {
                return idl_err("type array", "is malformed", idl_type_array);
            }
            let idl_item_type = &idl_type_array[0];
            let idl_item_length =
                idl_as_u64_or_else(&idl_type_array[1], "type array length")?;
            let mut data_length = 0;
            let mut data_items = vec![];
            for _ in 0..idl_item_length {
                let (data_item_length, data_item_value) = idl_type_data_read(
                    &data_bytes[data_length..],
                    idl_item_type,
                    idl_types,
                )?;
                data_length += data_item_length;
                data_items.push(data_item_value);
            }
            return Ok((data_length, Value::Array(data_items)));
        }
        if let Some(idl_type_defined) =
            idl_object_get_key_as_object(idl_type_object, "defined")
        {
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
            return idl_type_data_read(data_bytes, idl_type, idl_types);
        }
        return idl_err("type object", "is unknown", idl_type_object);
    }
    if let Some(idl_type_str) = idl_type.as_str() {
        macro_rules! return_from_integer_bytes {
            ($data_bytes:expr, $type:ident) => {
                let data_length = size_of::<$type>();
                let data_integer = $type::from_le_bytes(
                    $data_bytes[..data_length]
                        .try_into()
                        .map_err(ToolboxAnchorError::TryFromSlice)?,
                );
                return Ok((
                    data_length,
                    Value::Number(Number::from(data_integer)),
                ));
            };
        }
        if idl_type_str == "u8" {
            return_from_integer_bytes!(data_bytes, u8);
        }
        if idl_type_str == "i8" {
            return_from_integer_bytes!(data_bytes, i8);
        }
        if idl_type_str == "u16" {
            return_from_integer_bytes!(data_bytes, u16);
        }
        if idl_type_str == "i16" {
            return_from_integer_bytes!(data_bytes, i16);
        }
        if idl_type_str == "u32" {
            return_from_integer_bytes!(data_bytes, u32);
        }
        if idl_type_str == "i32" {
            return_from_integer_bytes!(data_bytes, i32);
        }
        if idl_type_str == "u64" {
            return_from_integer_bytes!(data_bytes, u64);
        }
        if idl_type_str == "i64" {
            return_from_integer_bytes!(data_bytes, i64);
        }
        macro_rules! return_from_converted_bytes {
            ($data_bytes:expr, $type:ident, $conversion:ident) => {
                let data_length = size_of::<$type>();
                let data_integer = $type::from_le_bytes(
                    $data_bytes[..data_length]
                        .try_into()
                        .map_err(ToolboxAnchorError::TryFromSlice)?,
                );
                return Ok((
                    data_length,
                    Value::Number($conversion(data_integer).ok_or_else(
                        || {
                            ToolboxAnchorError::Custom(format!(
                                "JSON Invalid number: {}",
                                data_integer
                            ))
                        },
                    )?),
                ));
            };
        }
        if idl_type_str == "u128" {
            fn number_u128(data_integer: u128) -> Option<Number> {
                Number::from_u128(data_integer)
            }
            return_from_converted_bytes!(data_bytes, u128, number_u128);
        }
        if idl_type_str == "i128" {
            fn number_i128(data_integer: i128) -> Option<Number> {
                Number::from_i128(data_integer)
            }
            return_from_converted_bytes!(data_bytes, i128, number_i128);
        }
        if idl_type_str == "pubkey" || idl_type_str == "publicKey" {
            let data_length = size_of::<Pubkey>();
            let data_pubkey = Pubkey::new_from_array(
                data_bytes[..data_length]
                    .try_into()
                    .map_err(ToolboxAnchorError::TryFromSlice)?,
            );
            return Ok((data_length, Value::String(data_pubkey.to_string())));
        }
        return idl_err(
            "type string",
            "unknown type descriptor",
            &idl_type_str,
        );
    }
    idl_err("type value", "is unknown", idl_type)
}
