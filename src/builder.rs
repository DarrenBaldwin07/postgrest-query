use crate::filter::PostgrestFilter;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use reqwest::{header::HeaderMap, Method};
use url::Url;

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

	pub fn find_unique<T>(&self)
	where
		T: Serialize + Deserialize<'static>,
	{
	}

	pub fn find_many<T: Serialize + DeserializeOwned>(&self) -> PostgrestFilter<T>
	where
		T: Serialize + DeserializeOwned
	{
		PostgrestFilter::new(self.url.clone(), Method::GET, self.headers.clone())
	}

	pub fn create<T>(&self, vales: T)
	where
		T: Serialize + DeserializeOwned,
	{
	}

	pub fn create_many<T>(&self, values: Vec<T>)
	where
		T: Serialize + DeserializeOwned,
	{
	}

	pub fn update<T>(&self)
	where
		T: Serialize + DeserializeOwned,
	{
	}

	pub fn delete(&self) {}

	pub fn delete_many(&self) {}
}
