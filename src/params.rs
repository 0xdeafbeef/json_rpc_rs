use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Params {
    None,
    Array(Vec<JsonValue>),
    Map(serde_json::Map<String, JsonValue>),
}

impl Params {
    pub fn parse<D: serde::de::DeserializeOwned>(self) -> Result<D, Error> {
        let value: JsonValue = self.into();
        serde_json::from_value(value).map_err(Error::InvalidParams)
    }

    pub fn to_value(&self) -> JsonValue {
        match self {
            Params::None => JsonValue::Null,
            Params::Array(a) => JsonValue::Array(a.clone()),
            Params::Map(a) => JsonValue::Object(a.clone()),
        }
    }
}

impl From<Params> for JsonValue {
    fn from(params: Params) -> Self {
        match params {
            Params::None => JsonValue::Null,
            Params::Array(vec) => JsonValue::Array(vec),
            Params::Map(map) => JsonValue::Object(map),
        }
    }
}
