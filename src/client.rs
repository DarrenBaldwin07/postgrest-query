use crate::builder::{PostgrestQueryBuilder, Count};
use reqwest::header::HeaderMap;

pub struct PostgrestClient {
	pub url: String,
	pub headers: Option<HeaderMap>,
}

impl PostgrestClient {
	pub fn new(url: String, headers: Option<HeaderMap>) -> PostgrestClient {
		PostgrestClient { url, headers }
	}

	pub fn from(&self, relation: &str) -> PostgrestQueryBuilder {
		let url = format!("{}/{}", self.url, relation);
		PostgrestQueryBuilder::new(url, self.headers.clone())
	}

	/// TODO https://postgrest.org/en/stable/references/api/schemas.html
	fn schema() {}

	/// Call a function in your database via postgrest
	/// TODO: https://postgrest.org/en/stable/references/api/stored_procedures.html
	fn call(&self, function: &str, head: Option<bool>, count: Count) {
		let mut url = format!("{}/rpc/{}", self.url, function);
	}
}
