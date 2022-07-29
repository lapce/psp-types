use std::path::PathBuf;

pub use lsp_types::notification::Notification;
pub use lsp_types::request::Request;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub enum DownloadFileRequest {}

impl Request for DownloadFileRequest {
    type Params = DownloadFileRequestParams;
    type Result = ();
    const METHOD: &'static str = "host/download_file";
}

#[derive(Deserialize, Serialize)]
pub struct DownloadFileRequestParams {
    pub path: PathBuf,
    pub url: String,
    pub overwrite: bool,
    pub exec: bool,
}

pub enum StartLspServerNotification {}

impl Notification for StartLspServerNotification {
    type Params = StartLspServerNotificationParams;
    const METHOD: &'static str = "host/start_lsp_server";
}

#[derive(Deserialize, Serialize)]
pub struct StartLspServerNotificationParams {
    pub exec_path: PathBuf,
    pub language_id: String,
    pub options: Option<Value>,
}
