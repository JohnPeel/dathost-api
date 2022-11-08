use std::path::PathBuf;

use serde::Serialize;

// FIXME: Documentation doesn't have the return type. Investigate.
pub type Files = serde_json::Value;

#[derive(Clone, Debug, Serialize)]
pub struct UploadFile {
    pub file: PathBuf,
}

#[derive(Clone, Debug, Serialize)]
pub struct Destination {
    pub destination: String,
}

impl From<String> for Destination {
    fn from(destination: String) -> Self {
        Destination { destination }
    }
}
