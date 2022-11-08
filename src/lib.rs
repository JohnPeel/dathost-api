mod models;
mod util;

use api_client::{api, Api};
use reqwest::{Client, RequestBuilder};

pub use models::*;

use crate::util::part_from_file_path;

const BASE_URL: &str = "https://dathost.com/api/v1";

#[derive(Clone)]
pub struct DathostApi {
    client: Client,
    username: String,
    password: String,
}

impl DathostApi {
    #[inline]
    #[must_use]
    pub fn new_with_client(client: Client, username: String, password: String) -> DathostApi {
        DathostApi {
            client,
            username,
            password,
        }
    }

    #[inline]
    #[must_use]
    pub fn new(username: String, password: String) -> DathostApi {
        DathostApi::new_with_client(Client::new(), username, password)
    }
}

impl Api for DathostApi {
    #[inline]
    fn client(&self) -> &Client {
        &self.client
    }

    #[inline]
    fn pre_request(&self, request: RequestBuilder) -> reqwest::Result<RequestBuilder> {
        Ok(request.basic_auth(&self.username, Some(&self.password)))
    }
}

impl DathostApi {
    pub async fn game_server_upload_file(
        &self,
        request: &UploadFile,
        server_id: &str,
        path: &str,
    ) -> Result<reqwest::StatusCode, Box<dyn std::error::Error + Send + Sync + 'static>> {
        use reqwest::multipart::Form;

        let part = part_from_file_path(request.file.as_path()).await?;
        Ok(self
            .request::<()>(
                reqwest::Method::POST,
                format!("{BASE_URL}/game-servers/{server_id}/files/{path}").as_str(),
                api_client::Body::Multipart(Form::new().part("file", part)),
            )
            .await
            .map(|res| res.status())?)
    }

    api! {
        pub fn account() -> Json<Account> {
            GET "{BASE_URL}/account"
        }

        pub fn custom_domains() -> Json<Vec<CustomDomain>> {
            GET "{BASE_URL}/custom-domains"
        }

        pub fn game_servers() -> Json<Vec<GameServer>> {
            GET "{BASE_URL}/game-servers"
        }

        pub fn game_server_create(request: Form<CreateGameServer>) -> Json<GameServer> {
            POST "{BASE_URL}/game-servers"
        }

        pub fn game_server_delete(server_id: &str) -> StatusCode {
            DELETE "{BASE_URL}/game-servers/{server_id}"
        }

        pub fn game_server(server_id: &str) -> Json<GameServer> {
            GET "{BASE_URL}/game-servers/{server_id}"
        }

        pub fn game_server_update(request: Form<UpdateGameServer>, server_id: &str) -> Json<GameServer> {
            PUT "{BASE_URL}/game-servers/{server_id}"
        }

        pub fn game_server_backups(server_id: &str) -> Json<Vec<Backup>> {
            GET "{BASE_URL}/game-servers/{server_id}/backups"
        }

        pub fn game_server_restore(server_id: &str, backup_name: &str) -> StatusCode {
            POST "{BASE_URL}/game-servers/{server_id}/backups/{backup_name}/restore"
        }

        pub fn game_server_console(server_id: &str) -> Json<Console> {
            GET "{BASE_URL}/game-servers/{server_id}/console"
        }

        pub fn game_server_console_send(request: Form<SendConsole>, server_id: &str) -> StatusCode {
            POST "{BASE_URL}/game-servers/{server_id}/console"
        }

        pub fn game_server_duplicate(server_id: &str) -> Json<GameServer> {
            POST "{BASE_URL}/game-servers/{server_id}/duplicate"
        }

        pub fn game_server_files(server_id: &str, hide_default_files: bool, include_deleted_files: bool, path: &str, with_filesizes: bool) -> Json<Files> {
            GET "{BASE_URL}/game-servers/{server_id}/files?hide_default_files={hide_default_files}&include_deleted_files={include_deleted_files}&path={path}&with_filesizes={with_filesizes}"
        }

        pub fn game_server_remove_file(server_id: &str, path: &str) -> StatusCode {
            DELETE "{BASE_URL}/game-servers/{server_id}/files/{path}"
        }

        pub fn game_server_download_file(server_id: &str, path: &str) -> Bytes {
            GET "{BASE_URL}/game-servers/{server_id}/files/{path}"
        }

        pub fn game_server_download_file_text(server_id: &str, path: &str) -> String {
            GET "{BASE_URL}/game-servers/{server_id}/files/{path}?as_text=true"
        }

        pub fn game_server_move_file(request: Form<Destination>, server_id: &str, path: &str) -> StatusCode {
            PUT "{BASE_URL}/game-servers/{server_id}/files/{path}"
        }

        pub fn game_server_metrics(server_id: &str) -> Json<Metrics> {
            GET "{BASE_URL}/game-servers/{server_id}/metrics"
        }

        pub fn game_server_regenerate_ftp_password(server_id: &str) -> StatusCode {
            POST "{BASE_URL}/game-servers/{server_id}/regenerate-ftp-password"
        }

        pub fn game_server_restart(server_id: &str) -> StatusCode {
            POST "{BASE_URL}/game-servers/{server_id}/restart"
        }

        pub fn game_server_start(request: Form<StartServer>, server_id: &str) -> StatusCode {
            POST "{BASE_URL}/game-servers/{server_id}/start"
        }

        pub fn game_server_stop(server_id: &str) -> StatusCode {
            POST "{BASE_URL}/game-servers/{server_id}/stop"
        }

        pub fn game_server_sync_files(server_id: &str) -> StatusCode {
            POST "{BASE_URL}/game-servers/{server_id}/sync-files"
        }

        pub fn game_server_unzip_file(request: Form<Destination>, server_id: &str, path: &str) -> StatusCode {
            POST "{BASE_URL}/game-servers/{server_id}/unzip-file/{path}"
        }

        pub fn match_series_create(request: Form<CreateMatchSeries>) -> Json<MatchSeries> {
            POST "{BASE_URL}/match-series"
        }

        pub fn match_series(series_id: &str) -> Json<MatchSeries> {
            GET "{BASE_URL}/match-series/{series_id}"
        }

        pub fn match_create(request: Form<CreateMatch>) -> Json<Match> {
            POST "{BASE_URL}/matches"
        }

        pub fn match_get(match_id: &str) -> Json<Match> {
            GET "{BASE_URL}/matches/{match_id}"
        }
    }
}
