use reqwest::{Client, Method};
use std::collections::HashMap;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use url::Url;

#[derive(Debug)]
enum FilterType {
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

    // TODO: question this builder pattern for filtering a little - maybe we can make this better?
    pub fn eq(&mut self, column: &str, value: &str) {
        self.url.query_pairs_mut().append_pair(column, format!("eq.{}", value).as_str());
    }

	pub fn filter(values: T, filter_method: FilterType) {}

    // TODO: like the normal `.exec()` but with the blocking reqwest client
	pub async fn exec_blocking(self) {}

	pub async fn exec(self) -> Result<T, reqwest::Error> {
		let client = Client::new();
		let res = client.request(self.method, self.url).send().await;

        match res {
            Ok(res) => {
                return res.json::<T>().await;
            }
            Err(e) => {
                return Err(e);
            }
        }
	}
}

