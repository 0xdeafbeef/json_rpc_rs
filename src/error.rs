use derive_more::Display;
use thiserror::Error;

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
            -32700 => Self::ParseError,
            -32600 => Self::InvalidRequest,
            -32601 => Self::MethodNotFound,
            -32602 => Self::InvalidParams,
            -32603 => Self::InternalError,
            a => Self::ServerError(a),
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
