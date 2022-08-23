use thiserror::Error;

#[derive(Error, Debug)]
pub enum FtxError {
    #[error("reqwest failed: {}", .0)]
    Reqwest(#[from] reqwest::Error),
    #[error("serde_json failed: {}", .0)]
    SerdeJson(#[from] serde_json::Error),
    #[error("ftx response: [ success {success:?} ] {error:?}")]
    Ftx { success: bool, error: String },
}
