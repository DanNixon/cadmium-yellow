#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
