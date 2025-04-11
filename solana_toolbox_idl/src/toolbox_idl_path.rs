use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlPath {
    pub parts: Vec<ToolboxIdlPathPart>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlPathPart {
    Key(String),
    Index(usize),
}

// TODO - have tests for path stuff
impl ToolboxIdlPath {
    pub fn try_parse(value: &str) -> Result<ToolboxIdlPath> {
        let mut parts = vec![];
        for part in value.split(".") {
            if part.contains(|c: char| !c.is_ascii_digit()) {
                parts.push(ToolboxIdlPathPart::Index(part.parse()?))
            } else {
                parts.push(ToolboxIdlPathPart::Key(part.to_string()))
            }
        }
        Ok(ToolboxIdlPath { parts })
    }

    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }

    pub fn export(&self) -> String {
        let mut parts = vec![];
        for part in &self.parts {
            parts.push(part.export());
        }
        parts.join(".")
    }

    pub fn split_first(&self) -> Option<(ToolboxIdlPathPart, ToolboxIdlPath)> {
        if let Some((first, rest)) = self.parts.split_first() {
            return Some((
                first.clone(),
                ToolboxIdlPath {
                    parts: rest.to_vec(),
                },
            ));
        }
        None
    }
}

impl ToolboxIdlPathPart {
    pub fn export(&self) -> String {
        match self {
            ToolboxIdlPathPart::Key(key) => key.to_string(),
            ToolboxIdlPathPart::Index(index) => index.to_string(),
        }
    }

    pub fn key(&self) -> Option<&str> {
        match self {
            ToolboxIdlPathPart::Key(key) => Some(key),
            ToolboxIdlPathPart::Index(_) => None,
        }
    }

    pub fn index(&self) -> Option<usize> {
        match self {
            ToolboxIdlPathPart::Key(_) => None,
            ToolboxIdlPathPart::Index(index) => Some(*index),
        }
    }
}
