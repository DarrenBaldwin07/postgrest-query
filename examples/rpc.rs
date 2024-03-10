use postgrest_query::client::PostgrestClient;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const POSTGREST_URL: &str = "https://org-darren-demo-org-inst-postgrest-query.data-1.use1.tembo.io/restapi/v1";

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
	headers.insert("Content-Type", "application/json".parse().unwrap());

    let mut args = HashMap::new();
    args.insert("a", serde_json::Value::from(2));
	args.insert("b", serde_json::Value::from(2));

	let db = PostgrestClient::new(POSTGREST_URL.to_string(), Some(headers));
	let query: serde_json::Value = db.call("add_them", false, None, args).await.unwrap();
	// Outputs `4`
	println!("QUERY{:?}", query);
}
