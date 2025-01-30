#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlProgramDefPrimitive {
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

impl ToolboxIdlProgramDefPrimitive {
    pub fn try_parse(name: &str) -> Option<ToolboxIdlProgramDefPrimitive> {
        match name {
            "u8" => Some(ToolboxIdlProgramDefPrimitive::U8),
            "u16" => Some(ToolboxIdlProgramDefPrimitive::U16),
            "u32" => Some(ToolboxIdlProgramDefPrimitive::U32),
            "u64" => Some(ToolboxIdlProgramDefPrimitive::U64),
            "u128" => Some(ToolboxIdlProgramDefPrimitive::U128),
            "i8" => Some(ToolboxIdlProgramDefPrimitive::I8),
            "i16" => Some(ToolboxIdlProgramDefPrimitive::I16),
            "i32" => Some(ToolboxIdlProgramDefPrimitive::I32),
            "i64" => Some(ToolboxIdlProgramDefPrimitive::I64),
            "i128" => Some(ToolboxIdlProgramDefPrimitive::I128),
            "f32" => Some(ToolboxIdlProgramDefPrimitive::F32),
            "f64" => Some(ToolboxIdlProgramDefPrimitive::F64),
            "bytes" => Some(ToolboxIdlProgramDefPrimitive::Bytes),
            "bool" => Some(ToolboxIdlProgramDefPrimitive::Boolean),
            "string" => Some(ToolboxIdlProgramDefPrimitive::String),
            "pubkey" => Some(ToolboxIdlProgramDefPrimitive::PublicKey),
            "publicKey" => Some(ToolboxIdlProgramDefPrimitive::PublicKey),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ToolboxIdlProgramDefPrimitive::U8 => "u8",
            ToolboxIdlProgramDefPrimitive::U16 => "u16",
            ToolboxIdlProgramDefPrimitive::U32 => "u32",
            ToolboxIdlProgramDefPrimitive::U64 => "u64",
            ToolboxIdlProgramDefPrimitive::U128 => "u128",
            ToolboxIdlProgramDefPrimitive::I8 => "i8",
            ToolboxIdlProgramDefPrimitive::I16 => "i16",
            ToolboxIdlProgramDefPrimitive::I32 => "i32",
            ToolboxIdlProgramDefPrimitive::I64 => "i64",
            ToolboxIdlProgramDefPrimitive::I128 => "i128",
            ToolboxIdlProgramDefPrimitive::F32 => "f32",
            ToolboxIdlProgramDefPrimitive::F64 => "f64",
            ToolboxIdlProgramDefPrimitive::Bytes => "bytes",
            ToolboxIdlProgramDefPrimitive::Boolean => "bool",
            ToolboxIdlProgramDefPrimitive::String => "string",
            ToolboxIdlProgramDefPrimitive::PublicKey => "pubkey",
        }
    }
}
