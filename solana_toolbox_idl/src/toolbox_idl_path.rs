#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlPath {
    pub parts: Vec<String>,
}

impl ToolboxIdlPath {
    pub fn try_parse(value: &str) -> ToolboxIdlPath {
        let mut parts = vec![];
        for part in value.split(".") {
            parts.push(part.to_string())
        }
        ToolboxIdlPath { parts }
    }

    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }

    pub fn export(&self) -> String {
        self.parts.join(".")
    }

    pub fn split_first(&self) -> Option<(String, ToolboxIdlPath)> {
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
