#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlProgramTypePrimitive {
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
    Bytes, // TODO - what's the usecase for this ?
    Boolean,
    String,
    PublicKey,
}

impl ToolboxIdlProgramTypePrimitive {
    pub fn try_parse(name: &str) -> Option<ToolboxIdlProgramTypePrimitive> {
        match name {
            "u8" => Some(ToolboxIdlProgramTypePrimitive::U8),
            "u16" => Some(ToolboxIdlProgramTypePrimitive::U16),
            "u32" => Some(ToolboxIdlProgramTypePrimitive::U32),
            "u64" => Some(ToolboxIdlProgramTypePrimitive::U64),
            "u128" => Some(ToolboxIdlProgramTypePrimitive::U128),
            "i8" => Some(ToolboxIdlProgramTypePrimitive::I8),
            "i16" => Some(ToolboxIdlProgramTypePrimitive::I16),
            "i32" => Some(ToolboxIdlProgramTypePrimitive::I32),
            "i64" => Some(ToolboxIdlProgramTypePrimitive::I64),
            "i128" => Some(ToolboxIdlProgramTypePrimitive::I128),
            "f32" => Some(ToolboxIdlProgramTypePrimitive::F32),
            "f64" => Some(ToolboxIdlProgramTypePrimitive::F64),
            "bytes" => Some(ToolboxIdlProgramTypePrimitive::Bytes),
            "bool" => Some(ToolboxIdlProgramTypePrimitive::Boolean),
            "string" => Some(ToolboxIdlProgramTypePrimitive::String),
            "pubkey" => Some(ToolboxIdlProgramTypePrimitive::PublicKey),
            "publicKey" => Some(ToolboxIdlProgramTypePrimitive::PublicKey),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ToolboxIdlProgramTypePrimitive::U8 => "u8",
            ToolboxIdlProgramTypePrimitive::U16 => "u16",
            ToolboxIdlProgramTypePrimitive::U32 => "u32",
            ToolboxIdlProgramTypePrimitive::U64 => "u64",
            ToolboxIdlProgramTypePrimitive::U128 => "u128",
            ToolboxIdlProgramTypePrimitive::I8 => "i8",
            ToolboxIdlProgramTypePrimitive::I16 => "i16",
            ToolboxIdlProgramTypePrimitive::I32 => "i32",
            ToolboxIdlProgramTypePrimitive::I64 => "i64",
            ToolboxIdlProgramTypePrimitive::I128 => "i128",
            ToolboxIdlProgramTypePrimitive::F32 => "f32",
            ToolboxIdlProgramTypePrimitive::F64 => "f64",
            ToolboxIdlProgramTypePrimitive::Bytes => "bytes",
            ToolboxIdlProgramTypePrimitive::Boolean => "bool",
            ToolboxIdlProgramTypePrimitive::String => "string",
            ToolboxIdlProgramTypePrimitive::PublicKey => "pubkey",
        }
    }
}
