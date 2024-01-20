use crate::builder::{Count, PostgrestQuery, PostgrestQueryBuilder};
use crate::filter::PostgrestFilter;
use reqwest::{header::{HeaderMap, HeaderName}, Method};
use url::Url;
use serde::{de::DeserializeOwned, Serialize};
use std::str::FromStr;
use std::collections::HashMap;
use serde::de::value::MapDeserializer;
pub struct PostgrestClient {
	pub url: String,
	pub headers: Option<HeaderMap>,
}

impl PostgrestClient {
	pub fn new(url: String, headers: Option<HeaderMap>) -> PostgrestClient {
		PostgrestClient { url, headers }
	}

	pub fn from(self, relation: &str) -> PostgrestQueryBuilder {
		let url = format!("{}/{}", self.url, relation);
		PostgrestQueryBuilder::new(url, self.headers.clone())
	}

	/// TODO: https://postgrest.org/en/stable/references/api/schemas.html
	fn schema() {}

	/// Call a function in your database via postgrest
	/// TODO: https://postgrest.org/en/stable/references/api/stored_procedures.html
	fn call<T>(self, function: &str, head: bool, count: Option<Count>, args: HashMap<&str, serde_json::Value>) -> PostgrestFilter<T, T>
	where
		T: Serialize + DeserializeOwned,
	{
		let url = format!("{}/rpc/{}", self.url, function);
		let req_method: Method;
		let req_body: Option<T>;
		let mut query_url = Url::parse(&url).expect("Failed to parse PostgrestClient.url");

		if head {
			req_method = Method::HEAD;
			for (key, value) in args {
				query_url.query_pairs_mut().append_pair(key, value.as_str().unwrap());
			}
			req_body = None;
		} else {
			req_method = Method::POST;
			req_body = Some(T::deserialize(MapDeserializer::new(args.into_iter())).unwrap());
		}

		if let Some(val) = count {
			let mut new_headers = self.headers.clone().unwrap_or_else(HeaderMap::new);
			new_headers.insert(HeaderName::from_str("Prefer").unwrap(), val.to_string().parse().unwrap());
		}

		PostgrestFilter::new(query_url, req_method, self.headers, req_body, PostgrestQuery::Call)
	}
}
