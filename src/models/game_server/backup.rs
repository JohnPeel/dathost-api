use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Backup {
    /// Backup name
    pub name: Option<String>,
    /// Creation timestamp
    pub timestamp: Option<String>,
}
