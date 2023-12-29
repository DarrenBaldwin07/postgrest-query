use reqwest::{header::HeaderMap, Client};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use url::Url;


#[derive(Debug)]
pub enum PostgrestError {
	PostgrestErrorResponse(PostgrestErrorResponse),
	ReqwestError(reqwest::Error),
}


#[derive(Debug, Deserialize, Serialize)]
pub struct PostgrestErrorResponse {
	pub hint: String,
	pub details: serde_json::Value,
	pub code: String,
	pub message: String,
}

pub struct PostgrestHandler {
    pub url: Url,
    pub headers: Option<HeaderMap>,
    pub method: reqwest::Method,
}

impl PostgrestHandler {
    pub fn new(url: Url, headers: Option<HeaderMap>, method: reqwest::Method) -> PostgrestHandler {
        PostgrestHandler { url, headers, method }
    }

    pub async fn exec<T>(self) -> Result<T, PostgrestError> where
	T: Serialize + DeserializeOwned {
        let client = Client::new();
		let res = client.request(self.method, self.url).headers(self.headers.unwrap_or(HeaderMap::new())).send().await;

		match res {
			Ok(res) => {
				if res.status().is_success() {
					match res.json::<T>().await {
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
