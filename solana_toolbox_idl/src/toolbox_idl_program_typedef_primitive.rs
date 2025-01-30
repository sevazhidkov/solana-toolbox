#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlProgramTypedefPrimitive {
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

impl ToolboxIdlProgramTypedefPrimitive {
    pub fn try_parse(name: &str) -> Option<ToolboxIdlProgramTypedefPrimitive> {
        match name {
            "u8" => Some(ToolboxIdlProgramTypedefPrimitive::U8),
            "u16" => Some(ToolboxIdlProgramTypedefPrimitive::U16),
            "u32" => Some(ToolboxIdlProgramTypedefPrimitive::U32),
            "u64" => Some(ToolboxIdlProgramTypedefPrimitive::U64),
            "u128" => Some(ToolboxIdlProgramTypedefPrimitive::U128),
            "i8" => Some(ToolboxIdlProgramTypedefPrimitive::I8),
            "i16" => Some(ToolboxIdlProgramTypedefPrimitive::I16),
            "i32" => Some(ToolboxIdlProgramTypedefPrimitive::I32),
            "i64" => Some(ToolboxIdlProgramTypedefPrimitive::I64),
            "i128" => Some(ToolboxIdlProgramTypedefPrimitive::I128),
            "f32" => Some(ToolboxIdlProgramTypedefPrimitive::F32),
            "f64" => Some(ToolboxIdlProgramTypedefPrimitive::F64),
            "bytes" => Some(ToolboxIdlProgramTypedefPrimitive::Bytes),
            "bool" => Some(ToolboxIdlProgramTypedefPrimitive::Boolean),
            "string" => Some(ToolboxIdlProgramTypedefPrimitive::String),
            "pubkey" => Some(ToolboxIdlProgramTypedefPrimitive::PublicKey),
            "publicKey" => Some(ToolboxIdlProgramTypedefPrimitive::PublicKey),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ToolboxIdlProgramTypedefPrimitive::U8 => "u8",
            ToolboxIdlProgramTypedefPrimitive::U16 => "u16",
            ToolboxIdlProgramTypedefPrimitive::U32 => "u32",
            ToolboxIdlProgramTypedefPrimitive::U64 => "u64",
            ToolboxIdlProgramTypedefPrimitive::U128 => "u128",
            ToolboxIdlProgramTypedefPrimitive::I8 => "i8",
            ToolboxIdlProgramTypedefPrimitive::I16 => "i16",
            ToolboxIdlProgramTypedefPrimitive::I32 => "i32",
            ToolboxIdlProgramTypedefPrimitive::I64 => "i64",
            ToolboxIdlProgramTypedefPrimitive::I128 => "i128",
            ToolboxIdlProgramTypedefPrimitive::F32 => "f32",
            ToolboxIdlProgramTypedefPrimitive::F64 => "f64",
            ToolboxIdlProgramTypedefPrimitive::Bytes => "bytes",
            ToolboxIdlProgramTypedefPrimitive::Boolean => "bool",
            ToolboxIdlProgramTypedefPrimitive::String => "string",
            ToolboxIdlProgramTypedefPrimitive::PublicKey => "pubkey",
        }
    }
}
