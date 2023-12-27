use std::collections::HashMap;
use url::Url;
use reqwest::{Client, Method};

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

pub struct PostgrestFilter {
    pub url: Url,
    pub headers: HashMap<String, String>,
    pub method: Method,
}

impl PostgrestFilter {
    pub fn new() -> Self {
        let url = Url::parse("http://localhost:3000").expect("Failed to parse PostgresQueryBuilder.url");
        let headers = HashMap::new();
        let method = Method::GET;
        PostgrestFilter { url, headers, method }
    }

    pub fn filter() {

    }

    pub async fn exec_blocking(self) {

    }

    pub async fn exec(self) {
        let client = Client::new();
        let res = client.request(self.method, self.url).send().await;
    }

}
