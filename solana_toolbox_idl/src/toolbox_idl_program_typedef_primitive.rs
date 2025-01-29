#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlProgramTypedefPrimitiveKind {
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

impl ToolboxIdlProgramTypedefPrimitiveKind {
    pub fn from_str(
        kind: &str
    ) -> Option<ToolboxIdlProgramTypedefPrimitiveKind> {
        match kind {
            "u8" => Some(ToolboxIdlProgramTypedefPrimitiveKind::U8),
            "u16" => Some(ToolboxIdlProgramTypedefPrimitiveKind::U16),
            "u32" => Some(ToolboxIdlProgramTypedefPrimitiveKind::U32),
            "u64" => Some(ToolboxIdlProgramTypedefPrimitiveKind::U64),
            "u128" => Some(ToolboxIdlProgramTypedefPrimitiveKind::U128),
            "i8" => Some(ToolboxIdlProgramTypedefPrimitiveKind::I8),
            "i16" => Some(ToolboxIdlProgramTypedefPrimitiveKind::I16),
            "i32" => Some(ToolboxIdlProgramTypedefPrimitiveKind::I32),
            "i64" => Some(ToolboxIdlProgramTypedefPrimitiveKind::I64),
            "i128" => Some(ToolboxIdlProgramTypedefPrimitiveKind::I128),
            "f32" => Some(ToolboxIdlProgramTypedefPrimitiveKind::F32),
            "f64" => Some(ToolboxIdlProgramTypedefPrimitiveKind::F64),
            "bool" => Some(ToolboxIdlProgramTypedefPrimitiveKind::Boolean),
            "string" => Some(ToolboxIdlProgramTypedefPrimitiveKind::String),
            "pubkey" => Some(ToolboxIdlProgramTypedefPrimitiveKind::PublicKey),
            "publicKey" => {
                Some(ToolboxIdlProgramTypedefPrimitiveKind::PublicKey)
            },
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ToolboxIdlProgramTypedefPrimitiveKind::U8 => "u8",
            ToolboxIdlProgramTypedefPrimitiveKind::U16 => "u16",
            ToolboxIdlProgramTypedefPrimitiveKind::U32 => "u32",
            ToolboxIdlProgramTypedefPrimitiveKind::U64 => "u64",
            ToolboxIdlProgramTypedefPrimitiveKind::U128 => "u128",
            ToolboxIdlProgramTypedefPrimitiveKind::I8 => "i8",
            ToolboxIdlProgramTypedefPrimitiveKind::I16 => "i16",
            ToolboxIdlProgramTypedefPrimitiveKind::I32 => "i32",
            ToolboxIdlProgramTypedefPrimitiveKind::I64 => "i64",
            ToolboxIdlProgramTypedefPrimitiveKind::I128 => "i128",
            ToolboxIdlProgramTypedefPrimitiveKind::F32 => "f32",
            ToolboxIdlProgramTypedefPrimitiveKind::F64 => "f64",
            ToolboxIdlProgramTypedefPrimitiveKind::Boolean => "boolean",
            ToolboxIdlProgramTypedefPrimitiveKind::String => "string",
            ToolboxIdlProgramTypedefPrimitiveKind::PublicKey => "publickey",
        }
    }
}
