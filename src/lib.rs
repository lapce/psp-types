pub use lsp_types;
pub use lsp_types::notification::Notification;
pub use lsp_types::request::Request;
use lsp_types::{DocumentSelector, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub enum StartLspServer {}

impl Notification for StartLspServer {
    type Params = StartLspServerParams;
    const METHOD: &'static str = "host/startLspServer";
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartLspServerParams {
    pub server_uri: Url,
    pub server_args: Vec<String>,
    pub document_selector: DocumentSelector,
    pub options: Option<Value>,
}

pub enum ExecuteProcess {}

impl Request for ExecuteProcess {
    type Params = ExecuteProcessParams;
    type Result = ExecuteProcessResult;
    const METHOD: &'static str = "host/executeProcess";
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteProcessParams {
    pub program: String,
    pub args: Vec<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteProcessResult {
    pub success: bool,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}
