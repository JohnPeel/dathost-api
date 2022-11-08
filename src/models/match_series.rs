use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MatchStartCt {
    Team1,
    Team2,
    Knife,
}

#[derive(Clone, Copy, Debug)]
pub enum MapCount {
    Two,
    Three,
    Five,
}

impl Serialize for MapCount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            MapCount::Two => serializer.serialize_u32(2),
            MapCount::Three => serializer.serialize_u32(3),
            MapCount::Five => serializer.serialize_u32(5),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct CreateMatchSeries {
    /// Time until match is canceled if not everyone has joined.
    pub connect_time: Option<u32>,

    /// Set to "true" to enable the !pause in-game chat command.
    pub enable_pause: Option<bool>,

    /// Set to "true" to enable PlayWin cheat detection for this match.
    pub enable_playwin: Option<bool>,

    /// Set to "true" to enable the !ready in-game chat command.
    pub enable_ready: Option<bool>,

    /// Set to "true" to enable the !tech in-game chat command.
    pub enable_tech_pause: Option<bool>,

    /// Id of the game server to run the match on.
    pub game_server_id: String,

    /// Map 1.
    pub map1: Option<String>,

    /// Team that starts on the CT side on map 1.
    pub map1_start_ct: Option<MatchStartCt>,

    /// Map 2.
    pub map2: Option<String>,

    /// Team that starts on the CT side on map 2.
    pub map2_start_ct: Option<MatchStartCt>,

    /// Map 3.
    pub map3: Option<String>,

    /// Team that starts on the CT side on map 3.
    pub map3_start_ct: Option<MatchStartCt>,

    /// Map 4.
    pub map4: Option<String>,

    /// Team that starts on the CT side on map 4.
    pub map4_start_ct: Option<MatchStartCt>,

    /// Map 5.
    pub map5: Option<String>,

    /// Team that starts on the CT side on map 5.
    pub map5_start_ct: Option<MatchStartCt>,

    /// URL to send a webhook to when the match is done.
    pub match_end_webhook_url: Option<String>,

    /// URL to send a webhook to when the match series is done.
    pub match_series_end_webhook_url: Option<String>,

    /// Prefix of in game chat messages from the match bot.
    pub message_prefix: Option<String>,

    /// Number of maps to play in the maps series.
    pub number_of_maps: Option<MapCount>,

    /// URL to send a webhook to when the PlayWin match analysis is done.
    pub playwin_result_webhook_url: Option<String>,

    /// If enable_ready is true, the game will wait until this number of players in each team have written !ready.
    pub ready_min_players: Option<u32>,

    /// URL to send a webhook to after each round.
    pub round_end_webhook_url: Option<String>,

    /// Comma separated list of Steam-IDs of spectators, if any.
    pub spectator_steam_ids: Option<String>,

    /// Steam-ID of an optional coach in team 1.
    pub team1_coach_steam_id: Option<String>,

    /// Flag of team 1 (ISO 3166-1 alpha-2).
    pub team1_flag: Option<String>,

    /// Name of team 1.
    pub team1_name: Option<String>,

    /// Comma separated list of Steam-IDs in team 1 which starts as Terrorists.
    pub team1_steam_ids: Option<String>,

    /// Steam-ID of an optional coach in team 2.
    pub team2_coach_steam_id: Option<String>,

    /// Flag of team 2 (ISO 3166-1 alpha-2).
    pub team2_flag: Option<String>,

    /// Name of team 2.
    pub team2_name: Option<String>,

    /// Comma separated list of Steam-IDs in team 2 which starts as Counter Terrorists.
    pub team2_steam_ids: Option<String>,

    /// Amount of players in each team.
    pub team_size: Option<u32>,

    /// Set to "false" to start the game when all players has connected even if there are missing coaches.
    pub wait_for_coaches: Option<bool>,

    /// Wait for GOTV delay to catch up before stopping the GOTV demo recording.
    pub wait_for_gotv_before_nextmap: Option<bool>,

    /// Set to "false" to start the game when all players has connected even if there are missing spectators.
    pub wait_for_spectators: Option<bool>,

    /// Warmup time after everyone have joined.
    pub warmup_time: Option<u32>,

    /// If set, this value will be set as an Authorization HTTP header on all webhook requests.
    pub webhook_authorization_header: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct MatchSeriesTeamStats {
    pub matches_won: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MatchSeries {
    pub finished: Option<bool>,
    pub id: Option<String>,
    pub match_series_end_webhook_url: Option<String>,
    pub matches: Option<Vec<crate::Match>>,
    pub team1_stats: Option<MatchSeriesTeamStats>,
    pub team2_stats: Option<MatchSeriesTeamStats>,
}
