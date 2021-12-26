use derive_more::Display;
use thiserror::Error;

use super::constants;

#[derive(Error, Debug, Display)]
pub enum ParamsError {
    InvalidParams(String),
}

#[derive(Debug, Display)]
pub enum JsonRpcErrorReason {
    ParseError,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    InternalError,
    ServerError(i32),
}

impl JsonRpcErrorReason {
    fn new(code: i32) -> Self {
        match code {
            constants::PARSE_ERROR => Self::ParseError,
            constants::INVALID_REQUEST => Self::InvalidRequest,
            constants::METHOD_NOT_FOUND => Self::MethodNotFound,
            constants::INVALID_PARAMS => Self::InvalidParams,
            constants::INTERNAL_ERROR => Self::InternalError,
            other => Self::ServerError(other),
        }
    }
}

#[derive(Debug, Display, Error)]
#[display(fmt = "{} {} {:?}", code, message, data)]
pub struct JsonRpcError {
    pub(crate) code: i32,
    pub(crate) message: String,
    pub(crate) data: Option<String>,
}

impl JsonRpcError {
    pub fn error_reason(&self) -> JsonRpcErrorReason {
        JsonRpcErrorReason::new(self.code)
    }
    pub fn code(&self) -> i32 {
        self.code
    }
}
