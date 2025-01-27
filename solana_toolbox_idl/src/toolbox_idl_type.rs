use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_as_u128_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;
use crate::toolbox_idl_utils::idl_value_as_str_or_object_with_name_as_str_or_else;

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlType {
    Defined { name: String, lookup: Box<ToolboxIdlType> },
    Option { content: Box<ToolboxIdlType> },
    Vec { items: Box<ToolboxIdlType> },
    Array { length: u32, items: Box<ToolboxIdlType> },
    Struct { fields: Vec<(String, ToolboxIdlType)> },
    Enum { variants: Vec<String> },
    Primitive { kind: ToolboxIdlTypePrimitiveKind },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlTypePrimitiveKind {
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
    Boolean,
    String,
    PublicKey,
}

impl ToolboxIdl {
    pub fn parse_type(
        &self,
        idl_type_value: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlType, ToolboxIdlError> {
        idl_type_parse_value(self, idl_type_value, breadcrumbs)
    }
}

pub fn idl_type_parse_value(
    idl: &ToolboxIdl,
    idl_type_value: &Value,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<ToolboxIdlType, ToolboxIdlError> {
    if let Some(idl_type_object) = idl_type_value.as_object() {
        return idl_type_parse_object(idl, idl_type_object, breadcrumbs);
    }
    if let Some(idl_type_array) = idl_type_value.as_array() {
        return idl_type_parse_array(idl, idl_type_array, breadcrumbs);
    }
    if let Some(idl_type_str) = idl_type_value.as_str() {
        return idl_type_parse_str(idl, idl_type_str, breadcrumbs);
    }
    idl_err(
        "Expected type object, array or string",
        &breadcrumbs.as_idl("typedef"),
    )
}

pub fn idl_type_parse_object(
    idl: &ToolboxIdl,
    idl_type_object: &Map<String, Value>,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<ToolboxIdlType, ToolboxIdlError> {
    if let Some(idl_type_defined) = idl_type_object.get("defined") {
        return idl_type_parse_defined(idl, idl_type_defined, breadcrumbs);
    }
    if let Some(idl_type_option) = idl_type_object.get("option") {
        return idl_type_parse_option(idl, idl_type_option, breadcrumbs);
    }
    if let Some(idl_type_vec) = idl_type_object.get("vec") {
        return idl_type_parse_vec(idl, idl_type_vec, breadcrumbs);
    }
    if let Some(idl_type_array) =
        idl_object_get_key_as_array(idl_type_object, "array")
    {
        return idl_type_parse_array(idl, idl_type_array, breadcrumbs);
    }
    if let Some(idl_type_fields) =
        idl_object_get_key_as_array(idl_type_object, "fields")
    {
        return idl_type_parse_struct_fields(idl, idl_type_fields, breadcrumbs);
    }
    if let Some(idl_type_variants) =
        idl_object_get_key_as_array(idl_type_object, "variants")
    {
        return idl_type_parse_enum_variants(idl_type_variants, breadcrumbs);
    }
    idl_err(
        "Missing type object key: defined/option/fields/variants/array/vec",
        &breadcrumbs.as_idl("typedef(object)"),
    )
}

pub fn idl_type_parse_array(
    idl: &ToolboxIdl,
    idl_type_array: &[Value],
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<ToolboxIdlType, ToolboxIdlError> {
    if idl_type_array.len() == 1 {
        return idl_type_parse_vec(idl, &idl_type_array[0], breadcrumbs);
    }
    if idl_type_array.len() == 2 {
        return Ok(ToolboxIdlType::Array {
            length: {
                let context = &breadcrumbs.as_idl("array_length");
                u32::try_from(idl_as_u128_or_else(&idl_type_array[1], context)?)
                    .map_err(|err| {
                        ToolboxIdlError::InvalidInteger {
                            conversion: err,
                            context: context.clone(),
                        }
                    })?
            },
            items: Box::new(idl_type_parse_value(
                idl,
                &idl_type_array[0],
                &breadcrumbs.with_idl("array"),
            )?),
        });
    }
    idl_err(
        "Array must be of either [{type}] or [{type}, {length}] format",
        &breadcrumbs.as_idl("typedef(array)"),
    )
}

pub fn idl_type_parse_str(
    idl: &ToolboxIdl,
    idl_type_str: &str,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<ToolboxIdlType, ToolboxIdlError> {
    let idl_type_primitive_kind = match idl_type_str {
        "u8" => Some(ToolboxIdlTypePrimitiveKind::U8),
        "u16" => Some(ToolboxIdlTypePrimitiveKind::U16),
        "u32" => Some(ToolboxIdlTypePrimitiveKind::U32),
        "u64" => Some(ToolboxIdlTypePrimitiveKind::U64),
        "u128" => Some(ToolboxIdlTypePrimitiveKind::U128),
        "i8" => Some(ToolboxIdlTypePrimitiveKind::I8),
        "i16" => Some(ToolboxIdlTypePrimitiveKind::I16),
        "i32" => Some(ToolboxIdlTypePrimitiveKind::I32),
        "i64" => Some(ToolboxIdlTypePrimitiveKind::I64),
        "i128" => Some(ToolboxIdlTypePrimitiveKind::I128),
        "f32" => Some(ToolboxIdlTypePrimitiveKind::F32),
        "f64" => Some(ToolboxIdlTypePrimitiveKind::F64),
        "bool" => Some(ToolboxIdlTypePrimitiveKind::Boolean),
        "string" => Some(ToolboxIdlTypePrimitiveKind::String),
        "pubkey" => Some(ToolboxIdlTypePrimitiveKind::PublicKey),
        "publicKey" => Some(ToolboxIdlTypePrimitiveKind::PublicKey),
        _ => None,
    };
    Ok(match idl_type_primitive_kind {
        Some(primitive_kind) => {
            ToolboxIdlType::Primitive { kind: primitive_kind }
        },
        None => {
            idl_type_parse_defined(
                idl,
                &Value::String(idl_type_str.to_string()),
                breadcrumbs,
            )?
        },
    })
}

pub fn idl_type_parse_defined(
    idl: &ToolboxIdl,
    idl_type_defined: &Value,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<ToolboxIdlType, ToolboxIdlError> {
    let idl_type_name = idl_value_as_str_or_object_with_name_as_str_or_else(
        idl_type_defined,
        &breadcrumbs.as_idl("defined"),
    )?;
    let idl_type_value = idl_object_get_key_or_else(
        &idl.types,
        idl_type_name,
        &breadcrumbs.as_idl("$idl_types"),
    )?;
    Ok(ToolboxIdlType::Defined {
        name: idl_type_name.to_string(),
        lookup:     Box::new(idl_type_parse_value(
            idl,
            idl_type_value,
            &breadcrumbs.with_idl(idl_type_name),
        )?    )
    })
}

pub fn idl_type_parse_option(
    idl: &ToolboxIdl,
    idl_type_option: &Value,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<ToolboxIdlType, ToolboxIdlError> {
    Ok(ToolboxIdlType::Option {
        content: Box::new(idl_type_parse_value(
            idl,
            idl_type_option,
            &breadcrumbs.with_idl("option"),
        )?),
    })
}

pub fn idl_type_parse_vec(
    idl: &ToolboxIdl,
    idl_type_vec: &Value,
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<ToolboxIdlType, ToolboxIdlError> {
    Ok(ToolboxIdlType::Vec {
        items: Box::new(idl_type_parse_value(
            idl,
            idl_type_vec,
            &breadcrumbs.with_idl("vec"),
        )?),
    })
}

pub fn idl_type_parse_struct_fields(
    idl: &ToolboxIdl,
    idl_type_fields: &[Value],
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<ToolboxIdlType, ToolboxIdlError> {
    let mut fields = vec![];
    for (index, idl_type_field) in idl_type_fields.iter().enumerate() {
        let context = &breadcrumbs.as_idl(&format!("fields[{}]", index));
        let idl_type_field_object =
            idl_as_object_or_else(idl_type_field, context)?;
        let idl_type_field_name = idl_object_get_key_as_str_or_else(
            idl_type_field_object,
            "name",
            context,
        )?;
        let breadcrumbs = &breadcrumbs.with_idl(idl_type_field_name);
        let idl_type_field_type = idl_object_get_key_or_else(
            idl_type_field_object,
            "type",
            &breadcrumbs.idl(),
        )?;
        fields.push((
            idl_type_field_name.to_string(),
            idl_type_parse_value(idl, idl_type_field_type, breadcrumbs)?,
        ));
    }
    Ok(ToolboxIdlType::Struct { fields })
}

// TODO - support for enums with content ?
pub fn idl_type_parse_enum_variants(
    idl_type_variants: &[Value],
    breadcrumbs: &ToolboxIdlBreadcrumbs,
) -> Result<ToolboxIdlType, ToolboxIdlError> {
    let mut variants = vec![];
    for (index, idl_type_variant) in idl_type_variants.iter().enumerate() {
        let idl_type_variant_name =
            idl_value_as_str_or_object_with_name_as_str_or_else(
                idl_type_variant,
                &breadcrumbs.as_idl(&format!("variants[{}]", index)),
            )?;
        variants.push(idl_type_variant_name.to_string());
    }
    Ok(ToolboxIdlType::Enum { variants })
}

impl ToolboxIdlType {
    pub fn describe(
        &self,
    ) -> String {
        match self {
            ToolboxIdlType::Defined { name, .. } => {
                name.to_string()
            },
            ToolboxIdlType::Option { content } => {
                format!(
                    "Option<{}>",
                    content.describe()
                )
            },
            ToolboxIdlType::Vec { items } => {
                format!(
                    "Vec<{}>",
                    items.describe()
                )
            },
            ToolboxIdlType::Array { length, items } => {
                format!(
                    "[{}; {}]",
                    items.describe(),
                    length
                )
            },
            ToolboxIdlType::Struct { .. } => "Struct()".to_string(),
            ToolboxIdlType::Enum { .. } => "Enum()".to_string(),
            ToolboxIdlType::Primitive { kind } => kind.as_str().to_string(),
        }
    }
}


impl ToolboxIdlTypePrimitiveKind {
    

    pub fn as_str(&self) -> &str {
        match self {
            ToolboxIdlTypePrimitiveKind::U8 => "u8",
            ToolboxIdlTypePrimitiveKind::U16 => "u16",
            ToolboxIdlTypePrimitiveKind::U32 => "u32",
            ToolboxIdlTypePrimitiveKind::U64 => "u64",
            ToolboxIdlTypePrimitiveKind::U128 => "u128",
            ToolboxIdlTypePrimitiveKind::I8 => "i8",
            ToolboxIdlTypePrimitiveKind::I16 => "i16",
            ToolboxIdlTypePrimitiveKind::I32 => "i32",
            ToolboxIdlTypePrimitiveKind::I64 => "i64",
            ToolboxIdlTypePrimitiveKind::I128 => "i128",
            ToolboxIdlTypePrimitiveKind::F32 => "f32",
            ToolboxIdlTypePrimitiveKind::F64 => "f64",
            ToolboxIdlTypePrimitiveKind::Boolean => "boolean",
            ToolboxIdlTypePrimitiveKind::String => "string",
            ToolboxIdlTypePrimitiveKind::PublicKey => "publickey",
        }
    }
}
