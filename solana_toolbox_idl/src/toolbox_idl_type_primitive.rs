#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlTypePrimitive {
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
    Bytes, // TODO (MEDIUM) - what's the usecase for this ?
    // TODO (MEDIUM) - should this be supporting something like a base58, base64 ?
    // TODO (MEDIUM) - should support like a "Rest" type which returns byte[]
    Boolean,
    String,
    PublicKey,
}

impl ToolboxIdlTypePrimitive {
    pub fn try_parse(name: &str) -> Option<ToolboxIdlTypePrimitive> {
        match name {
            "u8" => Some(ToolboxIdlTypePrimitive::U8),
            "u16" => Some(ToolboxIdlTypePrimitive::U16),
            "u32" => Some(ToolboxIdlTypePrimitive::U32),
            "u64" => Some(ToolboxIdlTypePrimitive::U64),
            "u128" => Some(ToolboxIdlTypePrimitive::U128),
            "i8" => Some(ToolboxIdlTypePrimitive::I8),
            "i16" => Some(ToolboxIdlTypePrimitive::I16),
            "i32" => Some(ToolboxIdlTypePrimitive::I32),
            "i64" => Some(ToolboxIdlTypePrimitive::I64),
            "i128" => Some(ToolboxIdlTypePrimitive::I128),
            "f32" => Some(ToolboxIdlTypePrimitive::F32),
            "f64" => Some(ToolboxIdlTypePrimitive::F64),
            "bytes" => Some(ToolboxIdlTypePrimitive::Bytes),
            "bool" => Some(ToolboxIdlTypePrimitive::Boolean),
            "string" => Some(ToolboxIdlTypePrimitive::String),
            "pubkey" => Some(ToolboxIdlTypePrimitive::PublicKey),
            "publicKey" => Some(ToolboxIdlTypePrimitive::PublicKey),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ToolboxIdlTypePrimitive::U8 => "u8",
            ToolboxIdlTypePrimitive::U16 => "u16",
            ToolboxIdlTypePrimitive::U32 => "u32",
            ToolboxIdlTypePrimitive::U64 => "u64",
            ToolboxIdlTypePrimitive::U128 => "u128",
            ToolboxIdlTypePrimitive::I8 => "i8",
            ToolboxIdlTypePrimitive::I16 => "i16",
            ToolboxIdlTypePrimitive::I32 => "i32",
            ToolboxIdlTypePrimitive::I64 => "i64",
            ToolboxIdlTypePrimitive::I128 => "i128",
            ToolboxIdlTypePrimitive::F32 => "f32",
            ToolboxIdlTypePrimitive::F64 => "f64",
            ToolboxIdlTypePrimitive::Bytes => "bytes",
            ToolboxIdlTypePrimitive::Boolean => "bool",
            ToolboxIdlTypePrimitive::String => "string",
            ToolboxIdlTypePrimitive::PublicKey => "pubkey",
        }
    }
}
