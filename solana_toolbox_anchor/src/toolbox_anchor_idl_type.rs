use serde_json::Map;
use serde_json::Number;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_anchor_endpoint::ToolboxAnchorEndpoint;
use crate::toolbox_anchor_error::ToolboxAnchorError;
use crate::toolbox_anchor_idl::ToolboxAnchorIdl;

impl ToolboxAnchorEndpoint {
    pub async fn get_account_data_anchor_idl_type_deserialized(
        &mut self,
        idl: &ToolboxAnchorIdl,
        address: &Pubkey,
        account_type: &str,
    ) -> Result<Option<(usize, Value)>, ToolboxAnchorError> {
        let idl_accounts = json_object_get_key_as_array(&idl.json, "accounts")
            .ok_or_else(|| {
                ToolboxAnchorError::Custom("IDL doesn't have accounts".into())
            })?;
        let idl_types = json_object_get_key_as_array(&idl.json, "types")
            .ok_or_else(|| {
                ToolboxAnchorError::Custom("IDL doesn't have types".into())
            })?;
        let idl_type =
            json_array_find_object_type_with_name(idl_accounts, account_type)
                .or(json_array_find_object_type_with_name(
                    idl_types,
                    account_type,
                ))
                .ok_or_else(|| {
                    ToolboxAnchorError::Custom(format!(
                        "IDL doesn't contain information about type: {}",
                        account_type
                    ))
                })?;
        let data_bytes =
            if let Some(account) = self.get_account(&address).await? {
                account.data
            } else {
                return Ok(None);
            };
        let expected_discriminator =
            self.compute_anchor_account_discriminator(account_type);
        let data_offset_discriminator = size_of_val(&expected_discriminator);
        let data_discriminator = u64::from_le_bytes(
            data_bytes[..data_offset_discriminator]
                .try_into()
                .map_err(ToolboxAnchorError::TryFromSlice)?,
        );
        if data_discriminator != expected_discriminator {
            return Err(ToolboxAnchorError::Custom(format!(
                "IDL discriminator doesnt match the account: found:{:X} expected:{:X}",
                data_discriminator, expected_discriminator
            )));
        }
        let (data_length, data_json) = idl_type_data_read_into_json(
            &data_bytes[data_offset_discriminator..],
            idl_type,
            idl_types,
        )?;
        Ok(Some((data_length + data_offset_discriminator, data_json)))
    }
}

fn json_object_get_key_as_array<'a>(
    object: &'a Map<String, Value>,
    key: &str,
) -> Option<&'a Vec<Value>> {
    object.get(key).map(|value| value.as_array()).flatten()
}

fn json_object_get_key_as_object<'a>(
    object: &'a Map<String, Value>,
    key: &str,
) -> Option<&'a Map<String, Value>> {
    object.get(key).map(|value| value.as_object()).flatten()
}

fn json_object_get_key_as_str<'a>(
    object: &'a Map<String, Value>,
    key: &str,
) -> Option<&'a str> {
    object.get(key).map(|value| value.as_str()).flatten()
}

fn json_array_find_object_type_with_name<'a>(
    array: &'a Vec<Value>,
    name: &str,
) -> Option<&'a Value> {
    for item in array.iter() {
        if let Some(item_object) = item.as_object() {
            if let Some(item_name) =
                json_object_get_key_as_str(item_object, "name")
            {
                if item_name == name {
                    return item_object.get("type");
                }
            }
        }
    }
    None
}

fn idl_type_data_read_into_json(
    data_bytes: &[u8],
    idl_type: &Value,
    idl_types: &Vec<Value>,
) -> Result<(usize, Value), ToolboxAnchorError> {
    if let Some(idl_type_object) = idl_type.as_object() {
        if let Some(idl_type_kind) =
            json_object_get_key_as_str(idl_type_object, "kind")
        {
            if idl_type_kind == "struct" {
                let mut data_length = 0;
                let mut data_fields = Map::new();
                let idl_type_fields =
                    json_object_get_key_as_array(idl_type_object, "fields")
                        .ok_or_else(|| {
                            ToolboxAnchorError::Custom(
                                "IDL struct doesn't have fields".into(),
                            )
                        })?;
                for idl_field in idl_type_fields {
                    let idl_field_object =
                        idl_field.as_object().ok_or_else(|| {
                            ToolboxAnchorError::Custom(
                                "IDL field is not an object".into(),
                            )
                        })?;
                    let idl_field_name =
                        json_object_get_key_as_str(idl_field_object, "name")
                            .ok_or_else(|| {
                                ToolboxAnchorError::Custom(
                                    "IDL field has no name".into(),
                                )
                            })?;
                    let idl_field_type =
                        idl_field.get("type").ok_or_else(|| {
                            ToolboxAnchorError::Custom(
                                "IDL field has no type".into(),
                            )
                        })?;
                    let (data_field_length, data_field_value) =
                        idl_type_data_read_into_json(
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
            json_object_get_key_as_array(idl_type_object, "array")
        {
            if idl_type_array.len() != 2 {
                return Err(ToolboxAnchorError::Custom(
                    "IDL invalid array type".into(),
                ));
            }
            let idl_item_type = &idl_type_array[0];
            let idl_item_length =
                &idl_type_array[1].as_u64().ok_or_else(|| {
                    ToolboxAnchorError::Custom(
                        "IDL array invalid length".into(),
                    )
                })?;
            let mut data_length = 0;
            let mut data_items = vec![];
            for _ in 0..*idl_item_length {
                let (data_item_length, data_item_value) =
                    idl_type_data_read_into_json(
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
            json_object_get_key_as_object(idl_type_object, "defined")
        {
            let idl_type_name =
                json_object_get_key_as_str(idl_type_defined, "name")
                    .ok_or_else(|| {
                        ToolboxAnchorError::Custom(
                            "IDL type as 'defined' doesnt have a name".into(),
                        )
                    })?;
            let idl_type =
                json_array_find_object_type_with_name(idl_types, idl_type_name)
                    .ok_or_else(|| {
                        ToolboxAnchorError::Custom(format!(
                            "IDL type as 'defined' unknown name: {}",
                            idl_type_name
                        ))
                    })?;
            return idl_type_data_read_into_json(
                data_bytes, idl_type, idl_types,
            );
        }
        return Err(ToolboxAnchorError::Custom(
            "IDL type object unsupported".into(),
        ));
    }
    if let Some(idl_type_str) = idl_type.as_str() {
        if idl_type_str == "u8" {
            let data_length = 1;
            let data_integer = u8::from_le_bytes(
                data_bytes[..data_length]
                    .try_into()
                    .map_err(ToolboxAnchorError::TryFromSlice)?,
            );
            return Ok((
                data_length,
                Value::Number(Number::from(data_integer)),
            ));
        }
        if idl_type_str == "u16" {
            let data_length = 2;
            let data_integer = u16::from_le_bytes(
                data_bytes[..data_length]
                    .try_into()
                    .map_err(ToolboxAnchorError::TryFromSlice)?,
            );
            return Ok((
                data_length,
                Value::Number(Number::from(data_integer)),
            ));
        }
        if idl_type_str == "u32" {
            let data_length = 4;
            let data_integer = u32::from_le_bytes(
                data_bytes[..data_length]
                    .try_into()
                    .map_err(ToolboxAnchorError::TryFromSlice)?,
            );
            return Ok((
                data_length,
                Value::Number(Number::from(data_integer)),
            ));
        }
        if idl_type_str == "u64" {
            let data_length = 8;
            let data_integer = u64::from_le_bytes(
                data_bytes[..data_length]
                    .try_into()
                    .map_err(ToolboxAnchorError::TryFromSlice)?,
            );
            return Ok((
                data_length,
                Value::Number(Number::from(data_integer)),
            ));
        }
        if idl_type_str == "u128" {
            let data_length = 16;
            let data_integer = u128::from_le_bytes(
                data_bytes[..data_length]
                    .try_into()
                    .map_err(ToolboxAnchorError::TryFromSlice)?,
            );
            return Ok((
                data_length,
                Value::Number(Number::from_u128(data_integer).ok_or_else(
                    || {
                        ToolboxAnchorError::Custom(format!(
                            "JSON Invalid number: {}",
                            data_integer
                        ))
                    },
                )?),
            ));
        }
        if idl_type_str == "i8" {
            let data_length = 1;
            let data_integer = i8::from_le_bytes(
                data_bytes[..data_length]
                    .try_into()
                    .map_err(ToolboxAnchorError::TryFromSlice)?,
            );
            return Ok((
                data_length,
                Value::Number(Number::from(data_integer)),
            ));
        }
        if idl_type_str == "i16" {
            let data_length = 2;
            let data_integer = i16::from_le_bytes(
                data_bytes[..data_length]
                    .try_into()
                    .map_err(ToolboxAnchorError::TryFromSlice)?,
            );
            return Ok((
                data_length,
                Value::Number(Number::from(data_integer)),
            ));
        }
        if idl_type_str == "i32" {
            let data_length = 4;
            let data_integer = i32::from_le_bytes(
                data_bytes[..data_length]
                    .try_into()
                    .map_err(ToolboxAnchorError::TryFromSlice)?,
            );
            return Ok((
                data_length,
                Value::Number(Number::from(data_integer)),
            ));
        }
        if idl_type_str == "i64" {
            let data_length = 8;
            let data_integer = i64::from_le_bytes(
                data_bytes[..data_length]
                    .try_into()
                    .map_err(ToolboxAnchorError::TryFromSlice)?,
            );
            return Ok((
                data_length,
                Value::Number(Number::from(data_integer)),
            ));
        }
        if idl_type_str == "i128" {
            let data_length = 16;
            let data_integer = i128::from_le_bytes(
                data_bytes[..data_length]
                    .try_into()
                    .map_err(ToolboxAnchorError::TryFromSlice)?,
            );
            return Ok((
                data_length,
                Value::Number(Number::from_i128(data_integer).ok_or_else(
                    || {
                        ToolboxAnchorError::Custom(format!(
                            "JSON Invalid number: {}",
                            data_integer
                        ))
                    },
                )?),
            ));
        }
        if idl_type_str == "pubkey" || idl_type_str == "publicKey" {
            let data_length = 32;
            let data_pubkey = Pubkey::new_from_array(
                data_bytes[..data_length]
                    .try_into()
                    .map_err(ToolboxAnchorError::TryFromSlice)?,
            );
            return Ok((data_length, Value::String(data_pubkey.to_string())));
        }
        return Err(ToolboxAnchorError::Custom(format!(
            "IDL unknown type code: {}",
            idl_type_str
        )));
    }
    Err(ToolboxAnchorError::Custom(format!(
        "IDL type unknown: {:?}",
        idl_type.to_string()
    )))
}
