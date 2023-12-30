use crate::handler::{PostgrestError, PostgrestHandler};
use reqwest::{header::HeaderMap, Method};
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

#[derive(Debug)]
pub enum FilterType {
	Eq,
	Neq,
	Gt,
	Gte,
	Lt,
	Lte,
	Like,
	Ilike,
	Is,
	In,
	Cs,
	Cd,
	Sl,
	Sr,
	Nxl,
	Nxr,
	Adj,
	Ov,
	Fts,
	Plfts,
	Phfts,
	Wfts,
}

impl std::fmt::Display for FilterType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let lowercase_str = match self {
			FilterType::Eq => "eq",
			FilterType::Neq => "neq",
			FilterType::Gt => "gt",
			FilterType::Gte => "gte",
			FilterType::Lt => "lt",
			FilterType::Lte => "lte",
			FilterType::Like => "like",
			FilterType::Ilike => "ilike",
			FilterType::Is => "is",
			FilterType::In => "in",
			FilterType::Cs => "cs",
			FilterType::Cd => "cd",
			FilterType::Sl => "sl",
			FilterType::Sr => "sr",
			FilterType::Nxl => "nxl",
			FilterType::Nxr => "nxr",
			FilterType::Adj => "adj",
			FilterType::Ov => "ov",
			FilterType::Fts => "fts",
			FilterType::Plfts => "plfts",
			FilterType::Phfts => "phfts",
			FilterType::Wfts => "wfts",
		};
		write!(f, "{}", lowercase_str)
	}
}

pub struct PostgrestFilter<T>
where
	T: Serialize + DeserializeOwned,
{
	pub url: Url,
	pub headers: Option<HeaderMap>,
	pub method: Method,
    pub body: Option<T>,
}

impl<T> PostgrestFilter<T>
where
	T: Serialize + DeserializeOwned,
{
	pub fn new(url: Url, method: Method, headers: Option<HeaderMap>, body: Option<T>) -> Self {
		PostgrestFilter {
			url,
			headers,
			method,
            body,
		}
	}

	// TODO: question this builder pattern for filtering - maybe we can make this better?
	pub fn eq(mut self, column: &str, value: &str) -> Self {
		self.url.query_pairs_mut().append_pair(column, format!("eq.{}", value).as_str());
		self
	}

	pub fn neq(mut self, column: &str, value: &str) -> Self {
		self.url.query_pairs_mut().append_pair(column, format!("neq.{}", value).as_str());
		self
	}

	pub fn gt(mut self, column: &str, value: &str) -> Self {
		self.url.query_pairs_mut().append_pair(column, format!("gt.{}", value).as_str());
		self
	}

	pub fn gte(mut self, column: &str, value: &str) -> Self {
		self.url.query_pairs_mut().append_pair(column, format!("gte.{}", value).as_str());
		self
	}

	pub fn lt(mut self, column: &str, value: &str) -> Self {
		self.url.query_pairs_mut().append_pair(column, format!("lt.{}", value).as_str());
		self
	}

	pub fn lte(mut self, column: &str, value: &str) -> Self {
		self.url.query_pairs_mut().append_pair(column, format!("lte.{}", value).as_str());
		self
	}

	pub fn like(mut self, column: &str, value: &str) -> Self {
		self.url.query_pairs_mut().append_pair(column, format!("like.{}", value).as_str());
		self
	}

	pub fn ilike(mut self, column: &str, value: &str) -> Self {
		self.url.query_pairs_mut().append_pair(column, format!("ilike.{}", value).as_str());
		self
	}

	pub fn is(mut self, column: &str, value: &str) -> Self {
		self.url.query_pairs_mut().append_pair(column, format!("is.{}", value).as_str());
		self
	}

	pub fn in_filter(mut self, column: &str, value: &str) -> Self {
		self.url.query_pairs_mut().append_pair(column, format!("in.{}", value).as_str());
		self
	}

	pub fn cs(mut self, column: &str, value: &str) -> Self {
		self.url.query_pairs_mut().append_pair(column, format!("cs.{}", value).as_str());
		self
	}

	pub fn cd(mut self, column: &str, value: &str) -> Self {
		self.url.query_pairs_mut().append_pair(column, format!("cd.{}", value).as_str());
		self
	}

	pub fn sl(mut self, column: &str, value: &str) -> Self {
		self.url.query_pairs_mut().append_pair(column, format!("sl.{}", value).as_str());
		self
	}

	pub fn sr(mut self, column: &str, value: &str) -> Self {
		self.url.query_pairs_mut().append_pair(column, format!("sr.{}", value).as_str());
		self
	}

	pub fn nxl(mut self, column: &str, value: &str) -> Self {
		self.url.query_pairs_mut().append_pair(column, format!("nxl.{}", value).as_str());
		self
	}

	pub fn nxr(mut self, column: &str, value: &str) -> Self {
		self.url.query_pairs_mut().append_pair(column, format!("nxr.{}", value).as_str());
		self
	}

	pub fn adj(mut self, column: &str, value: &str) -> Self {
		self.url.query_pairs_mut().append_pair(column, format!("adj.{}", value).as_str());
		self
	}

	pub fn ov(mut self, column: &str, value: &str) -> Self {
		self.url.query_pairs_mut().append_pair(column, format!("ov.{}", value).as_str());
		self
	}

	pub fn fts(mut self, column: &str, value: &str) -> Self {
		self.url.query_pairs_mut().append_pair(column, format!("fts.{}", value).as_str());
		self
	}

	pub fn plfts(mut self, column: &str, value: &str) -> Self {
		self.url.query_pairs_mut().append_pair(column, format!("plfts.{}", value).as_str());
		self
	}

	pub fn phfts(mut self, column: &str, value: &str) -> Self {
		self.url.query_pairs_mut().append_pair(column, format!("phfts.{}", value).as_str());
		self
	}

	pub fn wfts(mut self, column: &str, value: &str) -> Self {
		self.url.query_pairs_mut().append_pair(column, format!("wfts.{}", value).as_str());
		self
	}

	pub fn filter(mut self, column: &str, value: &str, filter_method: FilterType) -> Self {
		self.url
			.query_pairs_mut()
			.append_pair(column, format!("{}.{}", filter_method, value).as_str());
		self
	}

	pub fn exec_blocking(self) -> Result<T, PostgrestError> {
		let handler = PostgrestHandler::new(self.url, self.headers, self.method, self.body);
		handler.exec_blocking()
	}

	pub async fn exec(self) -> Result<T, PostgrestError> {
		let handler = PostgrestHandler::new(self.url, self.headers, self.method, self.body);
		handler.exec().await
	}
}
