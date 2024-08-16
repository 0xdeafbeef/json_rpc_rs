use std::borrow::Cow;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use reqwest::{Client as ReqwestClient, Url};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::error::{Error, JsonRpcError};

const JSON_RPC_VERSION: &str = "2.0";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);
const DEFAULT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Clone, Debug)]
pub struct Client {
    inner: ReqwestClient,
    url: Url,
    id: Arc<AtomicU64>,
}

impl Client {
    pub fn new(url: Url, timeout: Option<Duration>, connection_timeout: Option<Duration>) -> Self {
        let client = ReqwestClient::builder()
            .connect_timeout(connection_timeout.unwrap_or(DEFAULT_CONNECTION_TIMEOUT))
            .timeout(timeout.unwrap_or(DEFAULT_TIMEOUT))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            inner: client,
            url,
            id: Arc::new(AtomicU64::new(0)),
        }
    }

    pub async fn request<M, P, R>(&self, method: M, params: P) -> Result<R, Error>
    where
        M: AsRef<str> + Send,
        P: Serialize,
        R: DeserializeOwned,
    {
        let id = self.id.fetch_add(1, Ordering::SeqCst);

        let payload = serde_json::json!({
            "jsonrpc": JSON_RPC_VERSION,
            "method": method.as_ref(),
            "params": params,
            "id": id
        });

        let response = self
            .inner
            .post(self.url.clone())
            .json(&payload)
            .send()
            .await?
            .json()
            .await?;

        Self::parse_response(response)
    }

    pub fn parse_response<R: DeserializeOwned>(response: serde_json::Value) -> Result<R, Error> {
        let parsed: JsonRpcResponse = serde_json::from_value(response)?;

        match parsed.result {
            JsonRpcAnswer::Result(d) => serde_json::from_value(d).map_err(Error::Serialization),
            JsonRpcAnswer::Error(e) => Err(Error::JsonRpc(e)),
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new(
            Url::parse("http://localhost:8545").expect("Invalid default URL"),
            None,
            None,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
/// A JSON-RPC response.
pub struct JsonRpcResponse {
    /// Request content.
    pub result: JsonRpcAnswer,
    /// The request ID.
    pub id: Id,
}

/// An identifier established by the Client that MUST contain a String, Number,
/// or NULL value if included. If it is not included it is assumed to be a notification.
/// The value SHOULD normally not be Null and Numbers SHOULD NOT contain fractional parts
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Hash)]
#[serde(untagged)]
pub enum Id {
    Num(i64),
    Str(String),
    None(()),
}

#[derive(Serialize, Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
/// `JsonRpc` [response object](https://www.jsonrpc.org/specification#response_object)
pub enum JsonRpcAnswer {
    Result(serde_json::Value),
    Error(JsonRpcError),
}

impl<'de> Deserialize<'de> for JsonRpcResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        #[derive(Deserialize)]
        struct Helper<'a> {
            #[serde(borrow)]
            jsonrpc: Cow<'a, str>,
            #[serde(flatten)]
            result: JsonRpcAnswer,
            id: Id,
        }

        let helper = Helper::deserialize(deserializer)?;
        if helper.jsonrpc == JSON_RPC_VERSION {
            Ok(Self {
                result: helper.result,
                id: helper.id,
            })
        } else {
            Err(D::Error::custom("Unknown jsonrpc version"))
        }
    }
}
