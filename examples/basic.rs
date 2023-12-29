use postgrest_query::client::PostgrestClient;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

const POSTGREST_URL: &str = "http://localhost:3000";

#[derive(Debug, Deserialize, Serialize)]
struct User {
    id: i32,
    name: String,
    password: String,
}

// Run via: `POSTGREST_JWT=your_auth_jwt cargo run --example basic`
#[tokio::main]
async fn main() {
    let auth_key = format!("Bearer {}", std::env::var("POSTGREST_JWT").unwrap_or(String::from("")).to_string());
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", auth_key.parse().unwrap());

    let client = PostgrestClient::new(POSTGREST_URL.to_string(), Some(headers));
    let filter = client.from("users").find_many::<User>().exec().await;
}
