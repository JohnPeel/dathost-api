use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize)]
pub struct CreateMatch {
    /// Time until match is canceled if not everyone has joined
    pub connect_time: Option<u32>,

    /// Set to "true" to enable this knife round plugin: <https://github.com/dathost/CSGO-Knife-Round>
    pub enable_knife_round: Option<bool>,

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

    /// Map.
    pub map: Option<String>,

    /// URL to send a webhook to when the match is done.
    pub match_end_webhook_url: Option<String>,

    /// Prefix of in game chat messages from the match bot
    pub message_prefix: Option<String>,

    /// URL to send a webhook to when the PlayWin match analysis is done.
    pub playwin_result_webhook_url: Option<String>,

    /// If enable_ready is true, the game will wait until this number of players in each team have written !ready.
    pub ready_min_players: Option<u32>,

    /// URL to send a webhook to after each round.
    pub round_end_webhook_url: Option<String>,

    /// Comma separated list of Steam-IDs of spectators, if any.
    pub spectator_steam_ids: Option<String>,

    /// Steam-ID of an optional coach in team 1
    pub team1_coach_steam_id: Option<String>,

    /// Flag of team 1 (ISO 3166-1 alpha-2)
    pub team1_flag: Option<String>,

    /// Name of team 1
    pub team1_name: Option<String>,

    /// Comma separated list of Steam-IDs in team 1 which starts as Terrorists.
    pub team1_steam_ids: Option<String>,

    /// Steam-ID of an optional coach in team 2
    pub team2_coach_steam_id: Option<String>,

    /// Flag of team 2 (ISO 3166-1 alpha-2)
    pub team2_flag: Option<String>,

    /// Name of team 2
    pub team2_name: Option<String>,

    /// Comma separated list of Steam-IDs in team 2 which starts as Counter Terrorists.
    pub team2_steam_ids: Option<String>,

    /// Amount of players in each team.
    pub team_size: Option<u32>,

    /// Set to "false" to start the game when all players has connected even if there are missing coaches
    pub wait_for_coaches: Option<bool>,

    /// Wait for GOTV delay to catch up before stopping the GOTV demo recording.
    pub wait_for_gotv_before_nextmap: Option<bool>,

    /// Set to "false" to start the game when all players has connected even if there are missing spectators.
    pub wait_for_spectators: Option<bool>,

    /// Warmup time after everyone have joined
    pub warmup_time: Option<u32>,

    /// If set, this value will be set as an Authorization HTTP header on all webhook requests.
    pub webhook_authorization_header: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MatchPlayerStats {
    pub assists: Option<i32>,

    pub deaths: Option<i32>,

    pub kills: Option<i32>,

    pub steam_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MatchTeamStats {
    pub score: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Match {
    pub cancel_reason: Option<String>,

    pub connect_time: Option<i32>,

    pub enable_knife_round: Option<bool>,

    pub enable_pause: Option<bool>,

    pub enable_playwin: Option<bool>,

    pub enable_ready: Option<bool>,

    pub enable_tech_pause: Option<bool>,

    pub finished: Option<bool>,

    pub game_server_id: Option<String>,

    pub id: Option<String>,

    pub map: Option<String>,

    pub match_end_webhook_url: Option<String>,

    pub match_series_id: Option<String>,

    pub message_prefix: Option<String>,

    pub player_stats: Option<Vec<MatchPlayerStats>>,

    /// See <https://anticheat.playwin.me/csgo/webhook/>
    pub playwin_result: Option<Value>,

    pub playwin_result_webhook_url: Option<String>,

    pub ready_min_players: Option<i32>,

    pub round_end_webhook_url: Option<String>,

    pub rounds_played: Option<i32>,

    pub spectator_steam_ids: Option<Vec<String>>,

    pub started: Option<bool>,

    pub team1_coach_steam_id: Option<String>,

    pub team1_flag: Option<String>,

    pub team1_name: Option<String>,

    pub team1_start_ct: Option<bool>,

    pub team1_stats: Option<MatchTeamStats>,

    pub team1_steam_ids: Option<Vec<String>>,

    pub team2_coach_steam_id: Option<String>,

    pub team2_flag: Option<String>,

    pub team2_name: Option<String>,

    pub team2_stats: Option<MatchTeamStats>,

    pub team2_steam_ids: Option<Vec<String>>,

    pub team_size: Option<i32>,

    pub wait_for_coaches: Option<bool>,

    pub wait_for_gotv_before_nextmap: Option<bool>,

    pub wait_for_spectators: Option<bool>,

    pub warmup_time: Option<i32>,
}
