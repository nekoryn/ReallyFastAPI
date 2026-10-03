use std::collections::HashMap;
use serde::de::DeserializeOwned;
use hyper::{Uri, http};

pub struct Req {
    pub method: String,
    pub path: String,
    pub uri: Uri,
    pub body: String,
    pub params: HashMap<String, String>,
    pub extensions: http::Extensions,
}

impl Req {
    pub fn get<T: Clone + Send + Sync + 'static>(&self) -> Option<T> {
        self.extensions.get::<T>().cloned()
    }

    pub fn json<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_str(&self.body)
    }

    pub fn query(&self, key: &str) -> Option<String> {
        let query_str = self.uri.query()?;
        let pairs: HashMap<String, String> = serde_urlencoded::from_str(query_str).ok()?;
        pairs.get(key).cloned()
    }

    pub fn query_as<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_urlencoded::de::Error> {
        let query_str = self.uri.query().unwrap_or("");
        serde_urlencoded::from_str(query_str)
    }
}