use std::str::FromStr;

use crate::{filter::PostgrestFilter, handler::{PostgrestError, PostgrestHandler}};
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

pub struct PostgrestQueryBuilder {
	pub url: Url,
	pub headers: Option<HeaderMap>,
}

impl PostgrestQueryBuilder {
	pub fn new(url: String, headers: Option<HeaderMap>) -> Self {
		PostgrestQueryBuilder {
			url: Url::parse(&url).expect("Failed to parse PostgrestQueryBuilder.url"),
			headers,
		}
	}

	/// TODO: we may not want this at all (users could just use `find_many`` but filter that)
	pub fn find_unique<T>(self)
	where
		T: Serialize + Deserialize<'static>,
	{
	}

	/// Perform a SELECT query on the table/view.
	/// Returns all rows for the specified relation (table)
	///
	/// # Example
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
		PostgrestFilter::new(self.url, Method::PATCH, self.headers, Some(values), PostgrestQuery::Update)
	}

	pub fn upsert<T>(
		mut self,
		values: T,
		on_conflict: Option<String>,
		default_to_null: Option<bool>,
		count: Option<Count>,
		ignore_duplicates: Option<bool>,
	) -> PostgrestFilter<T, T>
	where
		T: Serialize + DeserializeOwned,
	{
		let ignore_duplicates = if ignore_duplicates.unwrap_or(false) {
			"ignore"
		} else {
			"merge"
		};

		let resolution = format!("resolution={ignoreDuplicates}-duplicates", ignoreDuplicates = ignore_duplicates);

		let mut postgrest_pref_headers: Vec<&str> = vec![&resolution];
		let mut new_headers = self.headers.clone().unwrap_or_else(HeaderMap::new);

		if let Some(on_conflict) = on_conflict {
			self.url.query_pairs_mut().append_pair("on_conflict", &on_conflict);
		}

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

		PostgrestFilter::new(self.url, Method::POST, self.headers, Some(values), PostgrestQuery::Update)
	}

	/// Perform a DELETE query on the table/view.
	///
	/// > Note: using `.delete()` should always be paried with filters to raget specific row(s)
	/// # Example
	pub fn delete<T>(mut self, count: Option<Count>) -> PostgrestFilter<T, T>
	where
		T: Serialize + DeserializeOwned,
	{
		let mut postgrest_pref_headers: Vec<&str> = Vec::new();
		let mut new_headers = self.headers.clone().unwrap_or_else(HeaderMap::new);

		if let Some(val) = count {
			match val {
				Count::Exact => postgrest_pref_headers.push("count=exact"),
				Count::Planned => postgrest_pref_headers.push("count=planned"),
				Count::Estimated => postgrest_pref_headers.push("count=estimated"),
			}
		}

		if let Some(headers) = &self.headers {
			if let Some(prefer) = headers.get("Prefers") {
				postgrest_pref_headers.insert(0, prefer.to_str().unwrap());
			}
		}

		new_headers.insert(HeaderName::from_str("Prefer").unwrap(), postgrest_pref_headers.join(",").parse().unwrap());
		self.headers = Some(new_headers);

		PostgrestFilter::new(self.url, Method::DELETE, self.headers, None, PostgrestQuery::Delete)
	}

	/// TODO: Maybe it makes sense not to have this?
	pub fn delete_many(self) {}
}


/// Builder that goes streight to exec and doesnt have any filter methods on it
pub struct PostgrestExecBuilder<T> where
T: Serialize + DeserializeOwned {
	pub url: Url,
	pub headers: Option<HeaderMap>,
	pub method: Method,
	pub query_type: PostgrestQuery,
	pub _marker: std::marker::PhantomData<T>,
}

impl<T> PostgrestExecBuilder<T> where
T: Serialize + DeserializeOwned {
	pub fn new(url: Url, headers: Option<HeaderMap>, method: Method, query_type: PostgrestQuery) -> Self {
		PostgrestExecBuilder {
			url,
			headers,
			method,
			query_type,
			_marker: std::marker::PhantomData,
		}
	}

	pub fn exec_blocking(self) -> Result<T, PostgrestError> {
		let handler: PostgrestHandler<T> = PostgrestHandler::new(self.url, self.headers, self.method, None, self.query_type);
		handler.exec_blocking()
	}

	pub async fn exec(self) -> Result<T, PostgrestError> {
		let handler: PostgrestHandler<T> = PostgrestHandler::new(self.url, self.headers, self.method, None, self.query_type);
		handler.exec().await
	}
}
