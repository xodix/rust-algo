use reqwest;
use serde_json::json;

#[tokio::main]
async fn fetch_data_text(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let result = reqwest::get(url).await?.text().await?;
    Ok(result)
}

#[tokio::main]
async fn post_data() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://httpbin.org/post")
        .json(&json!({"fuck": "me"}))
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

println!("{:?}", fetch_data_text("http://swapi.dev/api/people/1/"));
// println!("{:?}", post_data());