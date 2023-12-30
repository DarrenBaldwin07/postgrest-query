use reqwest::{blocking::Client as BlockingClient, header::HeaderMap, Client};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
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
	pub body: Option<T>
}

impl<T> PostgrestHandler<T> where
T: Serialize + DeserializeOwned {
	pub fn new(url: Url, headers: Option<HeaderMap>, method: reqwest::Method, body: Option<T>) -> PostgrestHandler<T> {
		PostgrestHandler { url, headers, method, body }
	}

	pub fn exec_blocking<O>(self) -> Result<O, PostgrestError>
	where
		O: Serialize + DeserializeOwned,
	{
		let client = BlockingClient::new();
		let mut req_builder = client
			.request(self.method, self.url)
			.headers(self.headers.unwrap_or(HeaderMap::new()));


		if let Some(body) = self.body {
			req_builder = req_builder.json(&body);
		}

		let res = req_builder.send();

		match res {
			Ok(res) => {
				if res.status().is_success() {
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
		let mut req_builder = client
			.request(self.method, self.url)
			.headers(self.headers.unwrap_or(HeaderMap::new()));


		if let Some(body) = self.body {
			req_builder = req_builder.json(&body);
		}

		let res = req_builder.send().await;


		match res {
			Ok(res) => {
				if res.status().is_success() {
					match res.json::<O>().await {
						Ok(res) => {
							return Ok(res);
						}
						Err(e) => {
							return Err(PostgrestError::ReqwestError(e));
						}
					}
				}
				println!("{:?}", res);
				let err = res.json::<PostgrestErrorResponse>().await.unwrap();
				return Err(PostgrestError::PostgrestErrorResponse(err));
			}
			Err(e) => {
				return Err(PostgrestError::ReqwestError(e));
			}
		}
	}
}
