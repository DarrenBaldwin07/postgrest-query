use reqwest::{Client, Method};
use std::{collections::HashMap, f32::consts::E};
use serde::{Serialize, de::DeserializeOwned, Deserialize};
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


#[derive(Debug, Deserialize, Serialize)]
pub struct PostgrestErrorResponse {
    pub hint: String,
    pub details: serde_json::Value,
    pub code: String,
    pub message: String,
}

pub enum PostgrestError {
    PostgrestErrorResponse(PostgrestErrorResponse),
    ReqwestError(reqwest::Error),
}

pub struct PostgrestFilter<T> where T: Serialize + DeserializeOwned {
	pub url: Url,
	pub headers: HashMap<String, String>,
	pub method: Method,
    _marker: std::marker::PhantomData<T>,
}

impl<T> PostgrestFilter<T> where T: Serialize + DeserializeOwned {
	pub fn new(url: Url, method: Method, headers: HashMap<String, String>) -> Self {
		PostgrestFilter { url, headers, method, _marker: std::marker::PhantomData }
	}

    // TODO: question this builder pattern for filtering - maybe we can make this better?
    pub fn eq(&mut self, column: &str, value: &str) {
        self.url.query_pairs_mut().append_pair(column, format!("eq.{}", value).as_str());
    }

    pub fn neq(&mut self, column: &str, value: &str) {
        self.url.query_pairs_mut().append_pair(column, format!("neq.{}", value).as_str());
    }

    pub fn gt(&mut self, column: &str, value: &str) {
        self.url.query_pairs_mut().append_pair(column, format!("gt.{}", value).as_str());
    }

	pub fn filter(values: T, filter_method: FilterType) {}

    // TODO: like the normal `.exec()` but with the blocking reqwest client
	pub async fn exec_blocking(self) {}

	pub async fn exec(self) -> Result<T, PostgrestError> {
		let client = Client::new();
		let res = client.request(self.method, self.url).send().await;

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
