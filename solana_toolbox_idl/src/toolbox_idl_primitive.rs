#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlPrimitive {
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
    Bytes,
    Boolean,
    String,
    PublicKey,
}

impl ToolboxIdlPrimitive {
    pub fn try_parse(name: &str) -> Option<ToolboxIdlPrimitive> {
        match name {
            "u8" => Some(ToolboxIdlPrimitive::U8),
            "u16" => Some(ToolboxIdlPrimitive::U16),
            "u32" => Some(ToolboxIdlPrimitive::U32),
            "u64" => Some(ToolboxIdlPrimitive::U64),
            "u128" => Some(ToolboxIdlPrimitive::U128),
            "i8" => Some(ToolboxIdlPrimitive::I8),
            "i16" => Some(ToolboxIdlPrimitive::I16),
            "i32" => Some(ToolboxIdlPrimitive::I32),
            "i64" => Some(ToolboxIdlPrimitive::I64),
            "i128" => Some(ToolboxIdlPrimitive::I128),
            "f32" => Some(ToolboxIdlPrimitive::F32),
            "f64" => Some(ToolboxIdlPrimitive::F64),
            "bytes" => Some(ToolboxIdlPrimitive::Bytes),
            "bool" => Some(ToolboxIdlPrimitive::Boolean),
            "string" => Some(ToolboxIdlPrimitive::String),
            "pubkey" => Some(ToolboxIdlPrimitive::PublicKey),
            "publicKey" => Some(ToolboxIdlPrimitive::PublicKey),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ToolboxIdlPrimitive::U8 => "u8",
            ToolboxIdlPrimitive::U16 => "u16",
            ToolboxIdlPrimitive::U32 => "u32",
            ToolboxIdlPrimitive::U64 => "u64",
            ToolboxIdlPrimitive::U128 => "u128",
            ToolboxIdlPrimitive::I8 => "i8",
            ToolboxIdlPrimitive::I16 => "i16",
            ToolboxIdlPrimitive::I32 => "i32",
            ToolboxIdlPrimitive::I64 => "i64",
            ToolboxIdlPrimitive::I128 => "i128",
            ToolboxIdlPrimitive::F32 => "f32",
            ToolboxIdlPrimitive::F64 => "f64",
            ToolboxIdlPrimitive::Bytes => "bytes",
            ToolboxIdlPrimitive::Boolean => "bool",
            ToolboxIdlPrimitive::String => "string",
            ToolboxIdlPrimitive::PublicKey => "pubkey",
        }
    }
}
