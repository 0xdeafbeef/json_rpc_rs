use reqwest::Url;
use serde_json::Value;
use tiny_jsonrpc::{Client, Params};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse("http://seed6.ngd.network:10332")?;
    let client = Client::new(url, None, None);
    let res: Value = client
        .request(
            "validateaddress",
            Params::Array(vec![serde_json::json!(
                "AQVh2pG732YvtNaxEGkQUei3YA4cvo7d2i"
            )]),
        )
        .await?;
    println!("Response: {:?}", res);
    Ok(())
}
