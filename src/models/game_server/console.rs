use serde::Serialize;

// FIXME: Documentation doesn't have return type. Investigate.
pub type Console = serde_json::Value;

#[derive(Clone, Debug, Serialize)]
pub struct SendConsole {
    pub line: String,
}

impl From<String> for SendConsole {
    fn from(line: String) -> Self {
        SendConsole { line }
    }
}
