use std::str::FromStr;

use serde_json::Map;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_as_array_or_else;
use crate::toolbox_idl_utils::idl_as_bool_or_else;
use crate::toolbox_idl_utils::idl_as_f64_or_else;
use crate::toolbox_idl_utils::idl_as_i128_or_else;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_as_str_or_else;
use crate::toolbox_idl_utils::idl_as_u128_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_array_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_object_array_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;
use crate::toolbox_idl_utils::idl_value_as_str_or_object_with_name_as_str_or_else;

impl ToolboxIdl {
    pub(crate) fn type_serialize(
        &self,
        idl_type: &Value,
        value: &Value,
        data: &mut Vec<u8>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<(), ToolboxIdlError> {
        idl_type_serialize(&self.types, idl_type, value, data, breadcrumbs)
    }
}

fn idl_type_serialize(
    idl_types: &Map<String, Value>,
    idl_type: &Value,
    value: &Value,
    data: &mut Vec<u8>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(), ToolboxIdlError> {
    if let Some(idl_type_object) = idl_type.as_object() {
        return idl_type_serialize_node(
            idl_types,
            idl_type_object,
            value,
            data,
            breadcrumbs,
        );
    }
    if let Some(idl_type_str) = idl_type.as_str() {
        return idl_type_serialize_leaf(
            idl_type_str,
            value,
            data,
            &breadcrumbs.with_idl(idl_type_str),
        );
    }
    idl_err("Expected object or string", &breadcrumbs.as_idl("typedef"))
}

fn idl_type_serialize_node(
    idl_types: &Map<String, Value>,
    idl_type_object: &Map<String, Value>,
    value: &Value,
    data: &mut Vec<u8>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(), ToolboxIdlError> {
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
            &breadcrumbs.with_idl("Option"),
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
                &breadcrumbs.with_idl("Enum"),
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
            &breadcrumbs.with_idl("Array"),
        );
    }
    if let Some(idl_type_vec) = idl_type_object.get("vec") {
        return idl_type_serialize_vec(
            idl_types,
            idl_type_vec,
            value,
            data,
            &breadcrumbs.with_idl("Vec"),
        );
    }
    idl_err(
        "Missing key: defined/option/kind/array/vec",
        &breadcrumbs.as_idl("typedef(object)"),
    )
}

fn idl_type_serialize_defined(
    idl_types: &Map<String, Value>,
    idl_type_defined: &Value,
    value: &Value,
    data: &mut Vec<u8>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(), ToolboxIdlError> {
    let idl_type_name = idl_value_as_str_or_object_with_name_as_str_or_else(
        idl_type_defined,
        &breadcrumbs.as_idl("defined"),
    )?;
    let idl_type = idl_object_get_key_or_else(
        idl_types,
        idl_type_name,
        &breadcrumbs.as_idl("$idl_types"),
    )?;
    idl_type_serialize(
        idl_types,
        idl_type,
        value,
        data,
        &breadcrumbs.with_idl(idl_type_name),
    )
}

fn idl_type_serialize_option(
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
        idl_type_serialize(idl_types, idl_type_option, value, data, breadcrumbs)
    }
}

fn idl_type_serialize_struct(
    idl_types: &Map<String, Value>,
    idl_type_struct: &Map<String, Value>,
    value: &Value,
    data: &mut Vec<u8>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(), ToolboxIdlError> {
    let value_object =
        idl_as_object_or_else(value, &breadcrumbs.as_val("struct"))?;
    let idl_type_fields_objects = idl_object_get_key_as_object_array_or_else(
        idl_type_struct,
        "fields",
        &breadcrumbs.as_idl("fields"),
    )?;
    for index in 0..idl_type_fields_objects.len() {
        let idl_field_object = idl_type_fields_objects.get(index).unwrap();
        let idl_field_name = idl_object_get_key_as_str_or_else(
            idl_field_object,
            "name",
            &breadcrumbs.as_idl(&format!("fields[{}]", index)),
        )?;
        let idl_field_type = idl_object_get_key_or_else(
            idl_field_object,
            "type",
            &breadcrumbs.as_idl(idl_field_name),
        )?;
        let value_field = idl_object_get_key_or_else(
            value_object,
            idl_field_name,
            &breadcrumbs.as_val("&"),
        )?;
        idl_type_serialize(
            idl_types,
            idl_field_type,
            value_field,
            data,
            &breadcrumbs.with_val(idl_field_name),
        )?;
    }
    Ok(())
}

// TODO - support for enums with content ?
fn idl_type_serialize_enum(
    idl_type_enum: &Map<String, Value>,
    value: &Value,
    data: &mut Vec<u8>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(), ToolboxIdlError> {
    let value_string = idl_as_str_or_else(value, &breadcrumbs.as_val("enum"))?;
    let idl_type_variants = idl_object_get_key_as_array_or_else(
        idl_type_enum,
        "variants",
        &breadcrumbs.as_idl("enum"),
    )?;
    for index in 0..idl_type_variants.len() {
        let idl_variant = idl_type_variants.get(index).unwrap();
        let idl_variant_name =
            idl_value_as_str_or_object_with_name_as_str_or_else(
                idl_variant,
                &breadcrumbs.as_idl(&format!("variants[{}]", index)),
            )?;
        if idl_variant_name == value_string {
            let data_enum = u8::try_from(index).unwrap();
            data.extend_from_slice(bytemuck::bytes_of::<u8>(&data_enum));
            return Ok(());
        }
    }
    idl_err("could not find matching enum", &breadcrumbs.as_val(value_string))
}

fn idl_type_serialize_array(
    idl_types: &Map<String, Value>,
    idl_type_array: &[Value],
    value: &Value,
    data: &mut Vec<u8>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(), ToolboxIdlError> {
    let value_array =
        idl_as_array_or_else(value, &breadcrumbs.as_val("array"))?;
    if idl_type_array.len() != 2 {
        return idl_err(
            "expected 2 items: type and length",
            &breadcrumbs.as_idl("[]"),
        );
    }
    let idl_item_type = &idl_type_array[0];
    let idl_item_length =
        idl_as_u128_or_else(&idl_type_array[1], &breadcrumbs.as_idl("length"))?;
    let idl_item_length = usize::try_from(idl_item_length).map_err(|err| {
        ToolboxIdlError::InvalidInteger {
            conversion: err,
            context: breadcrumbs.as_idl("length"),
        }
    })?;
    if value_array.len() != idl_item_length {
        return idl_err(
            &format!(
            "value array is not the correct size: expected {} items, found {} items",
            idl_item_length,
            value_array.len()
        ),
            &breadcrumbs.as_idl("value array"),
        );
    }
    for index in 0..value_array.len() {
        let value_item = value_array.get(index).unwrap();
        idl_type_serialize(
            idl_types,
            idl_item_type,
            value_item,
            data,
            &breadcrumbs.with_val(&format!("[{}]", index)),
        )?;
    }
    Ok(())
}

fn idl_type_serialize_vec(
    idl_types: &Map<String, Value>,
    idl_type_vec: &Value,
    value: &Value,
    data: &mut Vec<u8>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(), ToolboxIdlError> {
    let value_array = idl_as_array_or_else(value, &breadcrumbs.as_val("vec"))?;
    let value_length = u32::try_from(value_array.len()).unwrap();
    data.extend_from_slice(bytemuck::bytes_of::<u32>(&value_length));
    for index in 0..value_array.len() {
        let value_item = value_array.get(index).unwrap();
        idl_type_serialize(
            idl_types,
            idl_type_vec,
            value_item,
            data,
            &breadcrumbs.with_val(&format!("[{}]", index)),
        )?;
    }
    Ok(())
}

fn idl_type_serialize_leaf(
    idl_type_str: &str,
    value: &Value,
    data: &mut Vec<u8>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<(), ToolboxIdlError> {
    let context = &breadcrumbs.as_val("_");
    macro_rules! write_data_using_u_number {
        ($type:ident) => {
            let value_integer = idl_as_u128_or_else(value, context)?;
            let value_typed =
                $type::try_from(value_integer).map_err(|err| {
                    ToolboxIdlError::InvalidInteger {
                        conversion: err,
                        context: context.clone(),
                    }
                })?;
            data.extend_from_slice(bytemuck::bytes_of::<$type>(&value_typed));
        };
    }
    macro_rules! write_data_using_i_number {
        ($type:ident) => {
            let value_integer = idl_as_i128_or_else(value, context)?;
            let value_typed =
                $type::try_from(value_integer).map_err(|err| {
                    ToolboxIdlError::InvalidInteger {
                        conversion: err,
                        context: context.clone(),
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
        let value_integer = idl_as_u128_or_else(value, context)?;
        data.extend_from_slice(bytemuck::bytes_of::<u128>(&value_integer));
        return Ok(());
    }
    if idl_type_str == "i128" {
        let value_integer = idl_as_i128_or_else(value, context)?;
        data.extend_from_slice(bytemuck::bytes_of::<i128>(&value_integer));
        return Ok(());
    }
    if idl_type_str == "f32" {
        let value_floating = idl_as_f64_or_else(value, context)? as f32;
        data.extend_from_slice(bytemuck::bytes_of::<f32>(&value_floating));
        return Ok(());
    }
    if idl_type_str == "i128" {
        let value_floating = idl_as_f64_or_else(value, context)?;
        data.extend_from_slice(bytemuck::bytes_of::<f64>(&value_floating));
        return Ok(());
    }
    if idl_type_str == "bool" {
        let value_flag =
            if idl_as_bool_or_else(value, context)? { 1 } else { 0 };
        data.extend_from_slice(bytemuck::bytes_of::<u8>(&value_flag));
        return Ok(());
    }
    if idl_type_str == "string" {
        let value_str = idl_as_str_or_else(value, context)?;
        let value_length = u32::try_from(value_str.len()).unwrap();
        data.extend_from_slice(bytemuck::bytes_of::<u32>(&value_length));
        data.extend_from_slice(value_str.as_bytes());
        return Ok(());
    }
    if idl_type_str == "pubkey" || idl_type_str == "publicKey" {
        let value_str = idl_as_str_or_else(value, context)?;
        let value_pubkey = Pubkey::from_str(value_str).map_err(|err| {
            ToolboxIdlError::InvalidPubkey {
                parsing: err,
                context: context.clone(),
            }
        })?;
        data.extend_from_slice(bytemuck::bytes_of::<Pubkey>(&value_pubkey));
        return Ok(());
    }
    Err(ToolboxIdlError::InvalidTypeLeaf { context: context.clone() })
}
