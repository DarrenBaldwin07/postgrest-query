use crate::builder::PostgresQueryBuilder;

pub struct PostgrestClient {
    pub url: String
}

impl PostgrestClient {
    pub fn new(url: String) -> PostgrestClient {
        PostgrestClient { url }
    }

    pub fn from(&self, relation: &str) {
        let url = format!("{}/{}", self.url, relation);
        // This will just return a new PostgresQueryBuilder
    }
}
