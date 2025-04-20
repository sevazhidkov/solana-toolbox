use anyhow::Result;

use crate::toolbox_idl_utils::idl_u16_from_bytes_at;
use crate::toolbox_idl_utils::idl_u32_from_bytes_at;
use crate::toolbox_idl_utils::idl_u64_from_bytes_at;
use crate::toolbox_idl_utils::idl_u8_from_bytes_at;

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlTypePrefix {
    U8,
    U16,
    U32,
    U64,
}

impl ToolboxIdlTypePrefix {
    pub fn from_bytes_at(&self, bytes: &[u8], offset: usize) -> Result<u64> {
        match self {
            ToolboxIdlTypePrefix::U8 => {
                Ok(idl_u8_from_bytes_at(bytes, offset)?.into())
            },
            ToolboxIdlTypePrefix::U16 => {
                Ok(idl_u16_from_bytes_at(bytes, offset)?.into())
            },
            ToolboxIdlTypePrefix::U32 => {
                Ok(idl_u32_from_bytes_at(bytes, offset)?.into())
            },
            ToolboxIdlTypePrefix::U64 => {
                Ok(idl_u64_from_bytes_at(bytes, offset)?)
            },
        }
    }

    pub fn write(&self, value: u64, data: &mut Vec<u8>) -> Result<()> {
        match self {
            ToolboxIdlTypePrefix::U8 => {
                data.extend_from_slice(&u8::try_from(value)?.to_le_bytes())
            },
            ToolboxIdlTypePrefix::U16 => {
                data.extend_from_slice(&u16::try_from(value)?.to_le_bytes())
            },
            ToolboxIdlTypePrefix::U32 => {
                data.extend_from_slice(&u32::try_from(value)?.to_le_bytes())
            },
            ToolboxIdlTypePrefix::U64 => {
                data.extend_from_slice(&u64::try_from(value)?.to_le_bytes())
            },
        }
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        match self {
            ToolboxIdlTypePrefix::U8 => std::mem::size_of::<u8>(),
            ToolboxIdlTypePrefix::U16 => std::mem::size_of::<u16>(),
            ToolboxIdlTypePrefix::U32 => std::mem::size_of::<u32>(),
            ToolboxIdlTypePrefix::U64 => std::mem::size_of::<u64>(),
        }
    }
}
