use reqwest::Url;
use json_rpc::params::Params;
use json_rpc::JsonValue;
use serde_json::Value;

#[tokio::main]
async fn main()
{
    let url = Url::parse("http://seed6.ngd.network:10332").unwrap();
    let client = json_rpc::Client::new(&url, None, None);
   let res:Value =
    client
        .request(
            "validateaddress",
            Params::Array(vec![JsonValue::String("AQVh2pG732YvtNaxEGkQUei3YA4cvo7d2i".to_string())]),
        )
        .await.unwrap();
    dbg!(res);
}