use crate::builder::PostgresQueryBuilder;
use reqwest::header::HeaderMap;

pub struct PostgrestClient {
	pub url: String,
	pub headers: Option<HeaderMap>,
}

impl PostgrestClient {
	pub fn new(url: String, headers: Option<HeaderMap>) -> PostgrestClient {
		PostgrestClient { url, headers }
	}

	pub fn from(&self, relation: &str) -> PostgresQueryBuilder {
		let url = format!("{}/{}", self.url, relation);
		PostgresQueryBuilder::new(url, self.headers.clone())
	}

	/// TODO https://postgrest.org/en/stable/references/api/schemas.html
	fn schema() {}

	/// TODO: https://postgrest.org/en/stable/references/api/stored_procedures.html
	fn rpc() {}
}
