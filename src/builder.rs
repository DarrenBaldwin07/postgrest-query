use url::Url;
use std::collections::HashMap;

pub struct PostgresQueryBuilder {
    pub url: Url,
    pub headers: HashMap<String, String>
}

impl PostgresQueryBuilder {
    pub fn new(url: String, headers: HashMap<String, String>) -> PostgresQueryBuilder {
        PostgresQueryBuilder { url: Url::parse(&url).expect("Failed to parse PostgresQueryBuilder.url"), headers }
    }

    pub fn find_many() {

    }

    pub fn find_unique() {

    }

    pub fn create() {

    }

    pub fn create_many() {

    }

    pub fn update() {

    }

    pub fn delete() {

    }

    pub fn delete_many() {

    }

}
