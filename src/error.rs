use reqwest::StatusCode;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    /// http error
    #[error("request url:{url} error, statusCode:{status_code}, message:{message}")]
    HttpError {
        url: String,
        status_code: StatusCode,
        message: String,
    },
    /// API error
    #[error("errcode: {code}, errmsg: {message}")]
    ApiError { code: u64, message: String },
    /// 更新时空子段
    #[error("empty fileds when update")]
    EmptyFiledsUpdate,
    #[error("{0}")]
    General(String),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error(transparent)]
    InvalidHeaderName(#[from] reqwest::header::InvalidHeaderName),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    VarError(#[from] std::env::VarError),
}

pub fn new_http_error(url: String, status_code: StatusCode, message: String) -> Error {
    Error::HttpError {
        url,
        status_code,
        message,
    }
}

pub fn new_api_error(code: u64, message: String) -> Error {
    Error::ApiError { code, message }
}
