use postgrest_query::client::PostgrestClient;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

const POSTGREST_URL: &str = "https://org-darren-demo-org-inst-postgrest-query.data-1.use1.tembo.io/restapi/v1";

#[derive(Debug, Deserialize, Serialize)]
struct User {
	id: serde_json::Value,
	name: String,
	password: String,
}

// Run via: `POSTGREST_JWT=your_auth_jwt cargo run --example basic`
#[tokio::main]
async fn main() {
	let auth_key = format!("Bearer {}", std::env::var("POSTGREST_JWT").unwrap_or(String::from("")).to_string());
	let mut headers = HeaderMap::new();
	headers.insert("Authorization", auth_key.parse().unwrap());
	headers.insert("Content-Type", "application/json".parse().unwrap());
	let db = PostgrestClient::new(POSTGREST_URL.to_string(), Some(headers));

	let query = db.from("users").find_many::<User>().exec().await;

	println!("QUERY{:?}", query);
}
