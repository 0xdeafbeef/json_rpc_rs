use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value as JsonValue};

use crate::error::ParamsError as Error;

/// Request parameters
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Params {
    /// No parameters
    None,
    /// Array of values
    Array(Vec<JsonValue>),
    /// Map of values
    Map(serde_json::Map<String, JsonValue>),
}

impl Params {
    /// Parse incoming `Params` into expected common.
    pub fn parse<D>(self) -> Result<D, Error>
    where
        D: DeserializeOwned,
    {
        let value: JsonValue = self.into();
        from_value(value).map_err(|e| Error::InvalidParams(format!("Invalid params: {}.", e)))
    }

    pub fn to_value(&self) -> JsonValue {
        match self {
            Params::None => JsonValue::Null,
            Params::Array(a) => JsonValue::Array(a.clone()),
            Params::Map(a) => JsonValue::Object(a.clone()),
        }
    }

    /// Check for no params, returns Err if any params
    pub fn expect_no_params(self) -> Result<(), Error> {
        match self {
            Params::None => Ok(()),
            Params::Array(ref v) if v.is_empty() => Ok(()),
            _ => Err(Error::InvalidParams(
                "No parameters were expected".to_string(),
            )),
        }
    }
}

impl From<Params> for JsonValue {
    fn from(params: Params) -> JsonValue {
        match params {
            Params::Array(vec) => JsonValue::Array(vec),
            Params::Map(map) => JsonValue::Object(map),
            Params::None => JsonValue::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value as JsonValue;

    use crate::error::{ParamsError as Error, ParamsError};
    use crate::params::Params;

    #[test]
    fn params_deserialization() {
        let s = r#"[null, true, -1, 4, 2.3, "hello", [0], {"key": "value"}, []]"#;
        let deserialized: Params = serde_json::from_str(s).unwrap();

        let mut map = serde_json::Map::new();
        map.insert("key".to_string(), JsonValue::String("value".to_string()));

        assert_eq!(
            Params::Array(vec![
                JsonValue::Null,
                JsonValue::Bool(true),
                JsonValue::from(-1),
                JsonValue::from(4),
                JsonValue::from(2.3),
                JsonValue::String("hello".to_string()),
                JsonValue::Array(vec![JsonValue::from(0)]),
                JsonValue::Object(map),
                JsonValue::Array(vec![]),
            ]),
            deserialized
        );
    }

    #[test]
    fn should_return_meaningful_error_when_deserialization_fails() {
        // given
        let s = r#"[1, true]"#;
        let params = || serde_json::from_str::<Params>(s).unwrap();

        // when
        let v1: Result<(Option<u8>, String), Error> = params().parse();
        let v2: Result<(u8, bool, String), Error> = params().parse();
        let err1 = v1.unwrap_err();
        let err2 = v2.unwrap_err();

        // then
        assert!(matches!(err1, ParamsError::InvalidParams(_)));
        assert!(matches!(err2, ParamsError::InvalidParams(_)));
    }

    #[test]
    fn single_param_parsed_as_tuple() {
        let params: (u64,) = Params::Array(vec![JsonValue::from(1)]).parse().unwrap();
        assert_eq!(params, (1,));
    }
}
