use crate::builder::PostgrestQuery;
use reqwest::{blocking::Client as BlockingClient, header::{HeaderMap, USER_AGENT as REQWEST_USER_AGENT}, Client};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use url::Url;

#[derive(Debug)]
pub enum PostgrestError {
	PostgrestErrorResponse(PostgrestErrorResponse),
	ReqwestError(reqwest::Error),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostgrestErrorResponse {
	pub hint: Option<String>,
	pub details: Option<serde_json::Value>,
	pub code: Option<String>,
	pub message: Option<String>,
}

pub struct PostgrestHandler<T> {
	pub url: Url,
	pub headers: Option<HeaderMap>,
	pub method: reqwest::Method,
	pub query_type: PostgrestQuery,
	pub body: Option<T>,
}

pub const USER_AGENT: &str = concat!("postgrest-query", env!("CARGO_PKG_VERSION"));


impl<T> PostgrestHandler<T>
where
	T: Serialize + DeserializeOwned,
{
	pub fn new(url: Url, headers: Option<HeaderMap>, method: reqwest::Method, body: Option<T>, query_type: PostgrestQuery) -> PostgrestHandler<T> {
		PostgrestHandler {
			url,
			headers,
			method,
			body,
			query_type,
		}
	}

	pub fn exec_blocking<O>(self) -> Result<O, PostgrestError>
	where
		O: Serialize + DeserializeOwned,
	{
		let client = BlockingClient::new();
		let mut headers = self.headers.unwrap_or_else(HeaderMap::new);
		headers.insert(REQWEST_USER_AGENT, USER_AGENT.parse().unwrap());
		let mut req_builder = client.request(self.method, self.url).headers(headers);

		if let Some(body) = &self.body {
			req_builder = req_builder.json(&body);
		}

		let res = req_builder.send();

		match res {
			Ok(res) => {
				if res.status().is_success() {
					// Before we try and deserialize the response, check to make sure this isnt a mutation query
					if self.query_type == PostgrestQuery::Create {
						// Calc + return num of inputted rows
						let json_value: Value = serde_json::json!(1);
						let result: Result<O, _> = serde_json::from_value(json_value);
						return Ok(result.unwrap());
					}

					if self.query_type == PostgrestQuery::CreateMany {
						// We can safely unwrap here because we know we have a body due to `query_type`
						let body = self.body.unwrap();
						let json_body: Value = serde_json::json!(body);
						let result: Result<Vec<Value>, _> = serde_json::from_value(json_body);

						let num_rows = result.unwrap().len();
						let json_value: Value = serde_json::json!(num_rows);
						let result: Result<O, _> = serde_json::from_value(json_value);
						return Ok(result.unwrap());
					}
					match res.json::<O>() {
						Ok(res) => {
							return Ok(res);
						}
						Err(e) => {
							return Err(PostgrestError::ReqwestError(e));
						}
					}
				}
				let err = res.json::<PostgrestErrorResponse>().unwrap();
				return Err(PostgrestError::PostgrestErrorResponse(err));
			}
			Err(e) => {
				return Err(PostgrestError::ReqwestError(e));
			}
		}
	}

	pub async fn exec<O>(self) -> Result<O, PostgrestError>
	where
		O: Serialize + DeserializeOwned,
	{
		let client = Client::new();
		let mut headers = self.headers.unwrap_or_else(HeaderMap::new);
		headers.insert(REQWEST_USER_AGENT, USER_AGENT.parse().unwrap());
		let mut req_builder = client.request(self.method, self.url).headers(headers);

		if let Some(body) = &self.body {
			req_builder = req_builder.json(body);
		}

		let res = req_builder.send().await;

		match res {
			Ok(res) => {
				if res.status().is_success() {
					// Before we try and deserialize the response, check to make sure this isnt a mutation query
					if self.query_type == PostgrestQuery::Create {
						// Calc + return num of inputted rows
						let json_value: Value = serde_json::json!(1);
						let result: Result<O, _> = serde_json::from_value(json_value);
						return Ok(result.unwrap());
					}

					if self.query_type == PostgrestQuery::CreateMany {
						// We can safely unwrap here because we know we have a body due to `query_type`
						let body = self.body.unwrap();
						let json_body: Value = serde_json::json!(body);
						let result: Result<Vec<Value>, _> = serde_json::from_value(json_body);

						let num_rows = result.unwrap().len();
						let json_value: Value = serde_json::json!(num_rows);
						let result: Result<O, _> = serde_json::from_value(json_value);
						return Ok(result.unwrap());
					}
					match res.json::<O>().await {
						Ok(res) => {
							return Ok(res);
						}
						Err(e) => {
							return Err(PostgrestError::ReqwestError(e));
						}
					}
				}
				let err = res.json::<PostgrestErrorResponse>().await.unwrap();
				return Err(PostgrestError::PostgrestErrorResponse(err));
			}
			Err(e) => {
				return Err(PostgrestError::ReqwestError(e));
			}
		}
	}
}
