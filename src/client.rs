use reqwest::header::HeaderMap;
use crate::builder::PostgresQueryBuilder;

pub struct PostgrestClient {
	pub url: String,
	pub headers: HeaderMap
}

impl PostgrestClient {
	pub fn new(url: String, headers: HeaderMap) -> PostgrestClient {
		PostgrestClient { url, headers }
	}

	pub fn from(&self, relation: &str) -> PostgresQueryBuilder {
		let url = format!("{}/{}", self.url, relation);
		PostgresQueryBuilder::new(url, self.headers.clone())
	}

	/// TODO
	fn schema() {}

	/// TODO
	fn rpc() {}
}
