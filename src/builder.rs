use crate::filter::PostgrestFilter;
use reqwest::{header::HeaderMap, Method};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

pub enum Count {
	Exact,
	Planned,
	Estimated,
}

pub struct PostgresQueryBuilder {
	pub url: Url,
	pub headers: Option<HeaderMap>,
}

impl PostgresQueryBuilder {
	pub fn new(url: String, headers: Option<HeaderMap>) -> Self {
		PostgresQueryBuilder {
			url: Url::parse(&url).expect("Failed to parse PostgresQueryBuilder.url"),
			headers,
		}
	}

	/// TODO: we may not want this at all
	pub fn find_unique<T>(self)
	where
		T: Serialize + Deserialize<'static>,
	{
	}

	pub fn find_many<T: Serialize + DeserializeOwned>(self) -> PostgrestFilter<T>
	where
		T: Serialize + DeserializeOwned,
	{
		PostgrestFilter::new(self.url, Method::GET, self.headers)
	}

	pub fn create<T>(self, values: T, count: Option<Count>, default_to_null: bool) -> PostgrestFilter<T>
	where
		T: Serialize + DeserializeOwned,
	{
		PostgrestFilter::new(self.url, Method::POST, self.headers)
	}

	pub fn create_many<T>(self, values: Vec<T>) -> PostgrestFilter<T>
	where
		T: Serialize + DeserializeOwned,
	{
		PostgrestFilter::new(self.url, Method::POST, self.headers)
	}

	pub fn update<T>(self) -> PostgrestFilter<T>
	where
		T: Serialize + DeserializeOwned,
	{
		PostgrestFilter::new(self.url, Method::POST, self.headers)
	}

	pub fn delete(self) {}

	pub fn delete_many(self) {}
}
