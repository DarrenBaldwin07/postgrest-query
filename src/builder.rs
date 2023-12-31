use std::str::FromStr;

use crate::filter::PostgrestFilter;
use reqwest::{
	header::{HeaderMap, HeaderName},
	Method,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

#[derive(Debug, PartialEq, Eq)]
pub enum Count {
	Exact,
	Planned,
	Estimated,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PostgrestQuery {
	FindUnique,
	FindMany,
	Create,
	CreateMany,
	Update,
	UpdateMany,
	Delete,
	DeleteMany,
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

	/// TODO: we may not want this at all (users could just use find_many but filter that maybe?)
	pub fn find_unique<T>(self)
	where
		T: Serialize + Deserialize<'static>,
	{
	}

	pub fn find_many<T: Serialize + DeserializeOwned>(self) -> PostgrestFilter<T, T>
	where
		T: Serialize + DeserializeOwned,
	{
		PostgrestFilter::new(self.url, Method::GET, self.headers, None, PostgrestQuery::FindMany)
	}

	pub fn create<T>(mut self, values: T, default_to_null: Option<bool>, count: Option<Count>) -> PostgrestFilter<i32, T>
	where
		T: Serialize + DeserializeOwned,
	{
		let mut postgrest_pref_headers: Vec<&str> = Vec::new();
		let mut new_headers = self.headers.clone().unwrap_or_else(HeaderMap::new);

		if let Some(headers) = &self.headers {
			if let Some(prefer) = headers.get("Prefers") {
				postgrest_pref_headers.push(prefer.to_str().unwrap());
			}
		}

		if let Some(val) = default_to_null {
			if !val {
				postgrest_pref_headers.push("missing=default");
			}
		}

		// https://postgrest.org/en/stable/references/api/pagination_count.html?highlight=count
		if let Some(val) = count {
			match val {
				Count::Exact => postgrest_pref_headers.push("count=exact"),
				Count::Planned => postgrest_pref_headers.push("count=planned"),
				Count::Estimated => postgrest_pref_headers.push("count=estimated"),
			}
		}

		new_headers.insert(HeaderName::from_str("Prefer").unwrap(), postgrest_pref_headers.join(",").parse().unwrap());

		self.headers = Some(new_headers);

		PostgrestFilter::new(self.url, Method::POST, self.headers, Some(values), PostgrestQuery::Create)
	}

	pub fn create_many<T>(mut self, values: Vec<T>, default_to_null: Option<bool>, count: Option<Count>) -> PostgrestFilter<i32, Vec<T>>
	where
		T: Serialize + DeserializeOwned,
	{
		let mut postgrest_pref_headers: Vec<&str> = Vec::new();
		let mut new_headers = self.headers.clone().unwrap_or_else(HeaderMap::new);

		if let Some(headers) = &self.headers {
			if let Some(prefer) = headers.get("Prefers") {
				postgrest_pref_headers.push(prefer.to_str().unwrap());
			}
		}

		if let Some(val) = default_to_null {
			if !val {
				postgrest_pref_headers.push("missing=default");
			}
		}

		// https://postgrest.org/en/stable/references/api/pagination_count.html?highlight=count
		if let Some(val) = count {
			match val {
				Count::Exact => postgrest_pref_headers.push("count=exact"),
				Count::Planned => postgrest_pref_headers.push("count=planned"),
				Count::Estimated => postgrest_pref_headers.push("count=estimated"),
			}
		}

		new_headers.insert(HeaderName::from_str("Prefer").unwrap(), postgrest_pref_headers.join(",").parse().unwrap());

		self.headers = Some(new_headers);

		PostgrestFilter::new(self.url, Method::POST, self.headers, Some(values), PostgrestQuery::CreateMany)
	}

	pub fn update<T>(self, values: T) -> PostgrestFilter<T, T>
	where
		T: Serialize + DeserializeOwned,
	{
		PostgrestFilter::new(self.url, Method::POST, self.headers, None, PostgrestQuery::Update)
	}

	pub fn update_many<T>(self, values: Vec<T>) -> PostgrestFilter<T, T>
	where
		T: Serialize + DeserializeOwned,
	{
		PostgrestFilter::new(self.url, Method::PATCH, self.headers, None, PostgrestQuery::UpdateMany)
	}

	pub fn delete(self) {}

	pub fn delete_many(self) {}
}
