use std::str::FromStr;

use serde_json::Map;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
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
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        idl_type_serialize(&self.types, idl_type, value, data, breadcrumbs)
    }
}

pub fn idl_type_serialize(
    idl_types: &Map<String, Value>,
    idl_type: &Value,
    value: &Value,
    data: &mut Vec<u8>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(), ToolboxIdlError> {
    if let Some(idl_type_object) = idl_type.as_object() {
        if let Some(idl_type_defined) = idl_type_object.get("defined") {
            return idl_type_serialize_defined(
                idl_types,
                idl_type_defined,
                value,
                data,
                breadcrumbs,
            );
        }
        if let Some(idl_type_option) = idl_type_object.get("option") {
            return idl_type_serialize_option(
                idl_types,
                idl_type_option,
                value,
                data,
                breadcrumbs,
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
                    breadcrumbs,
                );
            }
            if idl_type_kind == "enum" {
                return idl_type_serialize_enum(
                    idl_type_object,
                    value,
                    data,
                    breadcrumbs,
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
                breadcrumbs,
            );
        }
        if let Some(idl_type_vec) = idl_type_object.get("vec") {
            return idl_type_serialize_vec(
                idl_types,
                idl_type_vec,
                value,
                data,
                breadcrumbs,
            );
        }
        return idl_err(
            "Missing key: defined/option/kind/array/vec",
            breadcrumbs.context("type object parsing"),
        );
    }
    if let Some(idl_type_str) = idl_type.as_str() {
        return idl_type_serialize_leaf(idl_type_str, value, data, breadcrumbs);
    }
    idl_err("Expected object or string", breadcrumbs.context("type parsing"))
}

pub fn idl_type_serialize_defined(
    idl_types: &Map<String, Value>,
    idl_type_defined: &Value,
    value: &Value,
    data: &mut Vec<u8>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(), ToolboxIdlError> {
    let idl_type_name = match idl_type_defined.as_str() {
        Some(idl_type_name) => idl_type_name,
        None => {
            let idl_type_defined_tag = "defined";
            let idl_type_defined_object = idl_as_object_or_else(
                idl_type_defined,
                breadcrumbs.context(idl_type_defined_tag),
            )?;
            idl_object_get_key_as_str_or_else(
                idl_type_defined_object,
                "name",
                &breadcrumbs.kind(idl_type_defined_tag),
            )?
        },
    };
    let idl_type = idl_object_get_key_or_else(
        idl_types,
        idl_type_name,
        &breadcrumbs.name("$idl_types"),
    )?;
    return idl_type_serialize(
        idl_types,
        idl_type,
        value,
        data,
        &breadcrumbs.kind(idl_type_name),
    );
}

pub fn idl_type_serialize_option(
    idl_types: &Map<String, Value>,
    idl_type_option: &Value,
    value: &Value,
    data: &mut Vec<u8>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(), ToolboxIdlError> {
    if value.is_null() {
        data.extend_from_slice(bytemuck::bytes_of::<u8>(&0));
        Ok(())
    } else {
        data.extend_from_slice(bytemuck::bytes_of::<u8>(&1));
        idl_type_serialize(
            idl_types,
            idl_type_option,
            value,
            data,
            &breadcrumbs.kind("option"),
        )
    }
}

pub fn idl_type_serialize_struct(
    idl_types: &Map<String, Value>,
    idl_type_struct: &Map<String, Value>,
    value: &Value,
    data: &mut Vec<u8>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(), ToolboxIdlError> {
    let value_object =
        idl_as_object_or_else(value, breadcrumbs.context("struct"))?;
    let idl_type_fields = idl_object_get_key_as_array_or_else(
        idl_type_struct,
        "fields",
        breadcrumbs,
    )?;
    for index in 0..idl_type_fields.len() {
        let idl_field = idl_type_fields.get(index).unwrap();
        let idl_field_tag = &format!("fields[{}]", index);
        let idl_field_object = idl_as_object_or_else(
            idl_field,
            breadcrumbs.context(idl_field_tag),
        )?;
        let idl_field_name = idl_object_get_key_as_str_or_else(
            idl_field_object,
            "name",
            &breadcrumbs.kind(idl_field_tag),
        )?;
        let idl_field_type = idl_object_get_key_or_else(
            idl_field_object,
            "type",
            &breadcrumbs.kind(idl_field_tag),
        )?;
        let value_field = idl_object_get_key_or_else(
            value_object,
            idl_field_name,
            breadcrumbs,
        )?;
        idl_type_serialize(
            idl_types,
            idl_field_type,
            value_field,
            data,
            &breadcrumbs.name(idl_field_name),
        )?;
    }
    Ok(())
}

pub fn idl_type_serialize_enum(
    idl_type_enum: &Map<String, Value>,
    value: &Value,
    data: &mut Vec<u8>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(), ToolboxIdlError> {
    let idl_type_variants = idl_object_get_key_as_array_or_else(
        idl_type_enum,
        "variants",
        breadcrumbs,
    )?;
    let value_string = idl_as_str_or_else(value, breadcrumbs.context("enum"))?;
    for index in 0..idl_type_variants.len() {
        let idl_variant = idl_type_variants.get(index).unwrap();
        let idl_variant_tag = &format!("variants[{}]", index);
        let idl_variant_object = idl_as_object_or_else(
            idl_variant,
            breadcrumbs.context(&idl_variant_tag),
        )?;
        let idl_variant_name = idl_object_get_key_as_str_or_else(
            idl_variant_object,
            "name",
            &breadcrumbs.kind(idl_variant_tag),
        )?;
        if idl_variant_name == value_string {
            let data_enum = u8::try_from(index).unwrap();
            data.extend_from_slice(bytemuck::bytes_of::<u8>(&data_enum));
            return Ok(());
        }
    }
    idl_err(
        format!("could not find matching enum"),
        breadcrumbs.context(value_string),
    )
}

pub fn idl_type_serialize_array(
    idl_types: &Map<String, Value>,
    idl_type_array: &Vec<Value>,
    value: &Value,
    data: &mut Vec<u8>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(), ToolboxIdlError> {
    let value_array =
        idl_as_array_or_else(value, breadcrumbs.context("array"))?;
    if idl_type_array.len() != 2 {
        return idl_err(
            format!("expected 2 items: type and length"),
            breadcrumbs.context("[]"),
        );
    }
    let idl_item_type = &idl_type_array[0];
    let idl_item_length =
        idl_as_u128_or_else(&idl_type_array[1], breadcrumbs.context("length"))?;
    let idl_item_length = usize::try_from(idl_item_length)
        .map_err(ToolboxIdlError::TryFromInt)?;
    if value_array.len() != idl_item_length {
        return idl_err(
            format!(
            "value array is not the correct size: expected {} items, found {} items",
            idl_item_length,
            value_array.len()
        ),
            breadcrumbs.context("value array"),
        );
    }
    for index in 0..value_array.len() {
        let value_item = value_array.get(index).unwrap();
        idl_type_serialize(
            idl_types,
            idl_item_type,
            value_item,
            data,
            &breadcrumbs.name(&format!("[{}]", index)),
        )?;
    }
    Ok(())
}

pub fn idl_type_serialize_vec(
    idl_types: &Map<String, Value>,
    idl_type_vec: &Value,
    value: &Value,
    data: &mut Vec<u8>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(), ToolboxIdlError> {
    let value_array = idl_as_array_or_else(value, breadcrumbs.context("vec"))?;
    let value_length = u32::try_from(value_array.len())
        .map_err(ToolboxIdlError::TryFromInt)?;
    data.extend_from_slice(bytemuck::bytes_of::<u32>(&value_length));
    for index in 0..value_array.len() {
        let value_item = value_array.get(index).unwrap();
        idl_type_serialize(
            idl_types,
            idl_type_vec,
            value_item,
            data,
            breadcrumbs,
        )?;
    }
    return Ok(());
}

pub fn idl_type_serialize_leaf(
    idl_type_str: &str,
    value: &Value,
    data: &mut Vec<u8>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(), ToolboxIdlError> {
    macro_rules! write_data_using_u_number {
        ($type:ident) => {
            let value_integer = idl_as_u128_or_else(value, breadcrumbs)?;
            let value_typed =
                $type::try_from(value_integer).map_err(|err| {
                    ToolboxIdlError::InvalidConversionInteger {
                        conversion: err,
                        breadcrumbs: breadcrumbs.clone(),
                    }
                })?;
            data.extend_from_slice(bytemuck::bytes_of::<$type>(&value_typed));
        };
    }
    macro_rules! write_data_using_i_number {
        ($type:ident) => {
            let value_integer = idl_as_i128_or_else(value, breadcrumbs)?;
            let value_typed =
                $type::try_from(value_integer).map_err(|err| {
                    ToolboxIdlError::InvalidConversionInteger {
                        conversion: err,
                        breadcrumbs: breadcrumbs.clone(),
                    }
                })?;
            data.extend_from_slice(bytemuck::bytes_of::<$type>(&value_typed));
        };
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
        let value_integer = idl_as_u128_or_else(value, breadcrumbs)?;
        data.extend_from_slice(bytemuck::bytes_of::<u128>(&value_integer));
        return Ok(());
    }
    if idl_type_str == "i128" {
        let value_integer = idl_as_i128_or_else(value, breadcrumbs)?;
        data.extend_from_slice(bytemuck::bytes_of::<i128>(&value_integer));
        return Ok(());
    }
    if idl_type_str == "bool" {
        let value_flag =
            if idl_as_bool_or_else(value, breadcrumbs)? { 1 } else { 0 };
        data.extend_from_slice(bytemuck::bytes_of::<u8>(&value_flag));
        return Ok(());
    }
    if idl_type_str == "string" {
        let value_str = idl_as_str_or_else(value, breadcrumbs)?;
        let value_length = u32::try_from(value_str.len()).unwrap();
        data.extend_from_slice(bytemuck::bytes_of::<u32>(&value_length));
        data.extend_from_slice(value_str.as_bytes());
        return Ok(());
    }
    if idl_type_str == "pubkey" || idl_type_str == "publicKey" {
        let value_str = idl_as_str_or_else(value, breadcrumbs)?;
        let value_pubkey = Pubkey::from_str(value_str).map_err(|err| {
            ToolboxIdlError::InvalidPubkey {
                parsing: err,
                breadcrumbs: breadcrumbs.clone(),
            }
        })?;
        data.extend_from_slice(bytemuck::bytes_of::<Pubkey>(&value_pubkey));
        return Ok(());
    }
    return Err(ToolboxIdlError::InvalidTypeObject {
        breadcrumbs: breadcrumbs.clone(),
    });
}
