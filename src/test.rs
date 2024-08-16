use crate::{Client, Error, Params};
use serde_json::json;

#[test]
fn test_params_parse() {
    // Test parsing array params
    let params = Params::Array(vec![json!(1), json!("test"), json!(true)]);
    let parsed: Vec<serde_json::Value> = params.parse().unwrap();
    assert_eq!(parsed, vec![json!(1), json!("test"), json!(true)]);

    // Test parsing object params
    let mut map = serde_json::Map::new();
    map.insert("key1".to_string(), json!(42));
    map.insert("key2".to_string(), json!("value"));
    let params = Params::Map(map);
    let parsed: serde_json::Map<String, serde_json::Value> = params.parse().unwrap();
    assert_eq!(parsed["key1"], json!(42));
    assert_eq!(parsed["key2"], json!("value"));

    // Test parsing empty params
    let params = Params::None;
    let parsed: Option<serde_json::Value> = params.parse().unwrap();
    assert_eq!(parsed, None);
}

#[test]
fn test_params_to_value() {
    // Test array params
    let params = Params::Array(vec![json!(1), json!("test"), json!(true)]);
    let value = params.to_value();
    assert_eq!(value, json!([1, "test", true]));

    // Test object params
    let mut map = serde_json::Map::new();
    map.insert("key1".to_string(), json!(42));
    map.insert("key2".to_string(), json!("value"));
    let params = Params::Map(map);
    let value = params.to_value();
    assert_eq!(value, json!({"key1": 42, "key2": "value"}));

    // Test empty params
    let params = Params::None;
    let value = params.to_value();
    assert_eq!(value, json!(null));
}

#[test]
fn test_client_parse_response() {
    // Test successful response
    let response = json!({
        "jsonrpc": "2.0",
        "result": 42,
        "id": 1
    });
    let parsed: Result<i32, Error> = Client::parse_response(response);
    assert_eq!(parsed.unwrap(), 42);

    // Test error response
    let response = json!({
        "jsonrpc": "2.0",
        "error": {
            "code": -32600,
            "message": "Invalid Request"
        },
        "id": 1
    });
    let parsed: Result<serde_json::Value, Error> = Client::parse_response(response);
    assert!(matches!(parsed, Err(Error::JsonRpc(_))));

    // Test invalid response (missing both result and error)
    let response = json!({
        "jsonrpc": "2.0",
        "id": 1
    });
    let parsed: Result<serde_json::Value, Error> = Client::parse_response(response);
    assert!(matches!(parsed, Err(Error::Serialization(_))));

    let answer = json!({"jsonrpc":"2.0","result":null,"id":1});
    let parsed: Result<Option<String>, Error> = Client::parse_response(answer);
    assert_eq!(parsed.unwrap(), None);
}

#[test]
fn test_params_parse_invalid() {
    // Test parsing invalid params
    let params = Params::Array(vec![json!(1), json!("test"), json!(true)]);
    let result: Result<String, _> = params.parse();
    assert!(result.is_err());
}

#[test]
fn test_rpc_call_with_positional_parameters() {
    let response = json!({
        "jsonrpc": "2.0",
        "result": 19,
        "id": 1
    });

    let parsed: Result<i32, Error> = Client::parse_response(response);
    assert_eq!(parsed.unwrap(), 19);
}

#[test]
fn test_rpc_call_with_named_parameters() {
    let response = json!({
        "jsonrpc": "2.0",
        "result": 19,
        "id": 3
    });

    let parsed: Result<i32, Error> = Client::parse_response(response);
    assert_eq!(parsed.unwrap(), 19);
}

#[test]
fn test_rpc_call_of_non_existent_method() {
    let response = json!({
        "jsonrpc": "2.0",
        "error": {
            "code": -32601,
            "message": "Method not found"
        },
        "id": "1"
    });

    let parsed: Result<serde_json::Value, Error> = Client::parse_response(response);
    assert!(matches!(parsed, Err(Error::JsonRpc(_))));
    if let Err(Error::JsonRpc(error)) = parsed {
        assert_eq!(error.code, -32601);
        assert_eq!(error.message, "Method not found");
    }
}

#[test]
fn test_rpc_call_with_invalid_json() {
    let response = json!({
        "jsonrpc": "2.0",
        "error": {
            "code": -32700,
            "message": "Parse error"
        },
        "id": null
    });

    let parsed: Result<serde_json::Value, Error> = Client::parse_response(response);
    assert!(matches!(parsed, Err(Error::JsonRpc(_))));
    if let Err(Error::JsonRpc(error)) = parsed {
        assert_eq!(error.code, -32700);
        assert_eq!(error.message, "Parse error");
    }
}

#[test]
fn test_rpc_call_with_invalid_request_object() {
    let response = json!({
        "jsonrpc": "2.0",
        "error": {
            "code": -32600,
            "message": "Invalid Request"
        },
        "id": null
    });

    let parsed: Result<serde_json::Value, Error> = Client::parse_response(response);
    assert!(matches!(parsed, Err(Error::JsonRpc(_))));
    if let Err(Error::JsonRpc(error)) = parsed {
        assert_eq!(error.code, -32600);
        assert_eq!(error.message, "Invalid Request");
    }
}

#[test]
fn test_rpc_call_batch_invalid_json() {
    let response = json!({
        "jsonrpc": "2.0",
        "error": {
            "code": -32700,
            "message": "Parse error"
        },
        "id": null
    });

    let parsed: Result<serde_json::Value, Error> = Client::parse_response(response);
    assert!(matches!(parsed, Err(Error::JsonRpc(_))));
    if let Err(Error::JsonRpc(error)) = parsed {
        assert_eq!(error.code, -32700);
        assert_eq!(error.message, "Parse error");
    }
}

#[test]
fn test_rpc_call_with_an_empty_array() {
    let response = json!({
        "jsonrpc": "2.0",
        "error": {
            "code": -32600,
            "message": "Invalid Request"
        },
        "id": null
    });

    let parsed: Result<serde_json::Value, Error> = Client::parse_response(response);
    assert!(matches!(parsed, Err(Error::JsonRpc(_))));
    if let Err(Error::JsonRpc(error)) = parsed {
        assert_eq!(error.code, -32600);
        assert_eq!(error.message, "Invalid Request");
    }
}
