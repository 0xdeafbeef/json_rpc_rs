use reqwest::{Client as ClientR, Url};
use serde_json::{json, Value, from_value};
use tokio::time::Duration;
pub mod error;
pub mod params;
use crate::error::{JsonRpcError};
use anyhow::Error;
use params::Params;
use serde::Deserialize;
pub use serde_json::Value as JsonValue;

use tokio::sync::Mutex;
use serde::de::DeserializeOwned;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Client {
    inner: ClientR,
    url: Url,
    id: Arc<Mutex<u64>>,
}

impl Client {
    pub fn new(url: &Url, timeout: Option<Duration>, connection_timeout: Option<Duration>) -> Self {
        let builder = ClientR::builder();
        let client = builder
            .connect_timeout(connection_timeout.unwrap_or_else(|| Duration::from_secs(10)))
            .timeout(timeout.unwrap_or_else(|| Duration::from_secs(5)))
            .build()
            .expect("Shouldn't happen on this set of options");
        //todo check correctness

        Client {
            inner: client,
            url: url.clone(),
            id: Arc::new(Mutex::new(0)),
        }
    }
    pub async fn request<M, Ret>(&self, method: M, params: Params) -> Result<Ret, Error>
    where
        M: AsRef<str>,
          Ret: DeserializeOwned,
    {
        #[derive(Deserialize, Debug)]
        struct JsonRpcData {
            result: Option<Value>,
            error: Option<Value>,
        }

        let client = &self.inner;
        let id = {
            let mut lock = self.id.lock().await;
            *lock += 1;
            *lock
        };

        let json_payload = json!({
            "jsonrpc": 2.0,
            "method": method.as_ref(),
            "params": params.to_value(),
            "id": id
        });

        let data: JsonRpcData = client
            .post(self.url.clone())
            .json(&json_payload)
            .send()
            .await?
            .json()
            .await?;
        match data.error {
            Some(a) =>  Err(parse_error(a)?.into()),
            None => match data.result {
                Some(a) => Ok( from_value(a)?),
                None => Err(Error::msg("Bad server  answer")),
            },
        }
    }
}

fn parse_error(valie: Value) -> Result<JsonRpcError, Error> {
    #[derive(Deserialize)]
    struct ErrorObj {
        code: i32,
        message: String,
        data: Option<String>,
    }
    let error_obj: ErrorObj = serde_json::from_value(valie)?;

    Ok(JsonRpcError {
        code: error_obj.code,
        data: error_obj.data,
        message: error_obj.message,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
