pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error occurred during HTTP request: {0}")]
    ReqwestError(#[from] reqwest::Error)
}