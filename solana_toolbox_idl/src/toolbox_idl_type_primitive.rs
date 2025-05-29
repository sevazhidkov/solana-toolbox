use solana_sdk::pubkey::Pubkey;

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
    Bool,
    Pubkey,
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
            "bool" => Some(ToolboxIdlTypePrimitive::Bool),
            "pubkey" => Some(ToolboxIdlTypePrimitive::Pubkey),
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
            ToolboxIdlTypePrimitive::Bool => "bool",
            ToolboxIdlTypePrimitive::Pubkey => "pubkey",
        }
    }

    pub fn size(&self) -> usize {
        match self {
            ToolboxIdlTypePrimitive::U8 => std::mem::size_of::<u8>(),
            ToolboxIdlTypePrimitive::U16 => std::mem::size_of::<u16>(),
            ToolboxIdlTypePrimitive::U32 => std::mem::size_of::<u32>(),
            ToolboxIdlTypePrimitive::U64 => std::mem::size_of::<u64>(),
            ToolboxIdlTypePrimitive::U128 => std::mem::size_of::<u128>(),
            ToolboxIdlTypePrimitive::I8 => std::mem::size_of::<i8>(),
            ToolboxIdlTypePrimitive::I16 => std::mem::size_of::<i16>(),
            ToolboxIdlTypePrimitive::I32 => std::mem::size_of::<i32>(),
            ToolboxIdlTypePrimitive::I64 => std::mem::size_of::<i64>(),
            ToolboxIdlTypePrimitive::I128 => std::mem::size_of::<i128>(),
            ToolboxIdlTypePrimitive::F32 => std::mem::size_of::<f32>(),
            ToolboxIdlTypePrimitive::F64 => std::mem::size_of::<f64>(),
            ToolboxIdlTypePrimitive::Bool => std::mem::size_of::<bool>(),
            ToolboxIdlTypePrimitive::Pubkey => std::mem::size_of::<Pubkey>(),
        }
    }

    pub fn alignment(&self) -> usize {
        match self {
            ToolboxIdlTypePrimitive::U8 => std::mem::size_of::<u8>(),
            ToolboxIdlTypePrimitive::U16 => std::mem::size_of::<u16>(),
            ToolboxIdlTypePrimitive::U32 => std::mem::size_of::<u32>(),
            ToolboxIdlTypePrimitive::U64 => std::mem::size_of::<u64>(),
            ToolboxIdlTypePrimitive::U128 => std::mem::size_of::<u128>(),
            ToolboxIdlTypePrimitive::I8 => std::mem::size_of::<i8>(),
            ToolboxIdlTypePrimitive::I16 => std::mem::size_of::<i16>(),
            ToolboxIdlTypePrimitive::I32 => std::mem::size_of::<i32>(),
            ToolboxIdlTypePrimitive::I64 => std::mem::size_of::<i64>(),
            ToolboxIdlTypePrimitive::I128 => std::mem::size_of::<i128>(),
            ToolboxIdlTypePrimitive::F32 => std::mem::size_of::<f32>(),
            ToolboxIdlTypePrimitive::F64 => std::mem::size_of::<f64>(),
            ToolboxIdlTypePrimitive::Bool => std::mem::size_of::<bool>(),
            ToolboxIdlTypePrimitive::Pubkey => std::mem::size_of::<u8>(),
        }
    }
}
