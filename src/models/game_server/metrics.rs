use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Player {
    /// How long the player has been online
    pub duration: Option<i32>,
    /// Player nickname
    pub name: Option<String>,
    /// Player score
    pub score: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MapsPlayed {
    /// Map name
    pub map: Option<String>,
    /// Approx. seconds played
    pub seconds: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GraphPoint {
    pub timestamp: Option<i32>,
    pub value: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Metrics {
    pub all_time_players: Option<Vec<Player>>,
    pub maps_played: Option<Vec<MapsPlayed>>,
    pub players_online: Option<Vec<Player>>,
    pub players_online_graph: Option<Vec<GraphPoint>>,
}
