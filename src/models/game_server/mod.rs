
mod backup;
mod console;
mod files;
mod metrics;

pub use backup::*;
pub use console::*;
pub use files::*;
pub use metrics::*;

use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct StartServer {
    pub allow_host_reassignment: bool,
}

// TODO: Unfinished models
pub type GameServer = serde_json::Value;
pub type CreateGameServer = serde_json::Value;
pub type UpdateGameServer = serde_json::Value;
