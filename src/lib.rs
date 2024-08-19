pub use lsp_types;
pub use lsp_types::notification::Notification;
pub use lsp_types::request::Request;
use lsp_types::{DocumentSelector, Uri};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub enum StartLspServer {}

impl Request for StartLspServer {
    type Params = StartLspServerParams;
    type Result = StartLspServerResult;
    const METHOD: &'static str = "host/startLspServer";
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartLspServerParams {
    pub server_uri: Uri,
    pub server_args: Vec<String>,
    pub document_selector: DocumentSelector,
    pub options: Option<Value>,
}

/// The id of a started Language Server.  
/// This is used to reference the server in future requests.
pub type LspId = u64;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartLspServerResult {
    pub id: LspId,
}

pub struct SendLspNotification {}

impl Notification for SendLspNotification {
    type Params = SendLspNotificationParams;
    const METHOD: &'static str = "host/sendLspNotification";
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendLspNotificationParams {
    /// Id of the LSP server the notification should be sent to
    pub id: LspId,
    pub method: String,
    pub params: Value,
}

/// Send an LSP request to a started LSP server.  
/// This does not include the `id` of the request, because that is handled by the client editor to
/// avoid collisions.
pub struct SendLspRequest {}

impl Request for SendLspRequest {
    type Params = SendLspRequestParams;
    type Result = SendLspRequestResult;
    const METHOD: &'static str = "host/sendLspRequest";
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendLspRequestParams {
    /// Id of the LSP server the notification should be sent to
    pub id: LspId,
    pub method: String,
    pub params: Value,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendLspRequestResult {
    pub result: Value,
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
    pub stdout: Option<Vec<u8>>,
    pub stderr: Option<Vec<u8>>,
}

pub enum RegisterDebuggerType {}

impl Request for RegisterDebuggerType {
    type Params = RegisterDebuggerTypeParams;
    type Result = ();
    const METHOD: &'static str = "host/registerDebuggerType";
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterDebuggerTypeParams {
    pub debugger_type: String,
    pub program: String,
    pub args: Option<Vec<String>>,
}
