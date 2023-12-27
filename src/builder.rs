use url::Url;
use std::collections::HashMap;

pub struct PostgresQueryBuilder {
    pub url: Url,
    pub headers: HashMap<String, String>
}

impl PostgresQueryBuilder {
    pub fn new(url: String, headers: HashMap<String, String>) -> Self {
        PostgresQueryBuilder { url: Url::parse(&url).expect("Failed to parse PostgresQueryBuilder.url"), headers }
    }


    pub fn find_unique<T>(&self) {

    }

    pub fn find_many<T>(&self) {

    }

    pub fn create<T>(&self, vales: T) {

    }

    pub fn create_many<T>(&self, values: Vec<T>) {

    }

    pub fn update<T>(&self) {

    }

    pub fn delete(&self) {

    }

    pub fn delete_many(&self) {

    }
}
