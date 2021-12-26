use reqwest::Url;
use serde_json::Value;
use tiny_jsonrpc::params::Params;

#[tokio::main]
async fn main() {
    let url = Url::parse("http://seed6.ngd.network:10332").unwrap();
    let client = tiny_jsonrpc::Client::new(&url, None, None);
    let res: Value = client
        .request(
            "validateaddress",
            Params::Array(vec![serde_json::Value::String(
                "AQVh2pG732YvtNaxEGkQUei3YA4cvo7d2i".to_string(),
            )]),
        )
        .await
        .unwrap();
    dbg!(res);
}
