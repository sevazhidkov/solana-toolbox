use anyhow::anyhow;
use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlTypePrefix {
    U8,
    U16,
    U32,
    U64,
}

impl ToolboxIdlTypePrefix {
    pub fn from_size(size: usize) -> Result<ToolboxIdlTypePrefix> {
        Ok(match size {
            1 => ToolboxIdlTypePrefix::U8,
            2 => ToolboxIdlTypePrefix::U16,
            4 => ToolboxIdlTypePrefix::U32,
            8 => ToolboxIdlTypePrefix::U64,
            _ => return Err(anyhow!("Prefix size {} is not supported", size)),
        })
    }

    pub fn to_size(&self) -> usize {
        match self {
            ToolboxIdlTypePrefix::U8 => 1,
            ToolboxIdlTypePrefix::U16 => 2,
            ToolboxIdlTypePrefix::U32 => 4,
            ToolboxIdlTypePrefix::U64 => 8,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ToolboxIdlTypePrefix::U8 => "u8",
            ToolboxIdlTypePrefix::U16 => "u16",
            ToolboxIdlTypePrefix::U32 => "u32",
            ToolboxIdlTypePrefix::U64 => "u64",
        }
    }
}
