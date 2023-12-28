use crate::filter::PostgrestFilter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

pub struct PostgresQueryBuilder {
	pub url: Url,
	pub headers: HashMap<String, String>,
}

impl PostgresQueryBuilder {
	pub fn new(url: String, headers: HashMap<String, String>) -> Self {
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

	pub fn find_many<T>(&self)
	where
		T: Serialize + Deserialize<'static>,
	{

	}

	pub fn create<T>(&self, vales: T)
	where
		T: Serialize + Deserialize<'static>,
	{
	}

	pub fn create_many<T>(&self, values: Vec<T>)
	where
		T: Serialize + Deserialize<'static>,
	{
	}

	pub fn update<T>(&self)
	where
		T: Serialize + Deserialize<'static>,
	{
	}

	pub fn delete(&self) {}

	pub fn delete_many(&self) {}
}
