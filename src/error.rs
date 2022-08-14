use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;
use std::{
    error,
    fmt::{self, Display},
    result,
};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Http(ReqwestError),
    Json(SerdeJsonError),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Http(e) => e.fmt(f),
            Error::Json(e) => e.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Http(e) => e.source(),
            Error::Json(e) => e.source(),
        }
    }
}

impl From<ReqwestError> for Error {
    fn from(error: ReqwestError) -> Error {
        Error::Http(error)
    }
}

impl From<SerdeJsonError> for Error {
    fn from(e: SerdeJsonError) -> Error {
        Error::Json(e)
    }
}
