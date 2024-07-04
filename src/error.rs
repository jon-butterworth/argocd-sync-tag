use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Request error: {0}")]
    Reqwest(#[from] ReqwestError),
    #[error("Serde error: {0}")]
    Serde(#[from] SerdeError),
    #[error("Other error: {0}")]
    Other(String),
}

