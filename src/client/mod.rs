//! # QStash client.
//! This is the main struct you will use to interact with the QStash API.
//! It is initialized with a token and optionally a base url and a version.
//! The default base url is `https://qstash.upstash.io`.

mod error;
pub mod events;
pub mod publish;
mod request;

use error::*;
pub use request::*;

use reqwest::{header, Url};

/// The version of the QStash API to use.
/// The default is V2.
pub enum Version {
    V1,
    V2,
}

/// The QStash client.
pub struct Client {
    pub http: reqwest::Client,
    base_url: Url,
    version: String,
}

impl Client {
    /// Initialize a new QStash client.
    /// The token is required.
    /// The base url and version are optional.
    pub fn new(
        token: &str,
        base_url: Option<&str>,
        version: Option<Version>,
    ) -> Result<Client, QStashError> {
        // intialize default headers
        let mut value = match header::HeaderValue::from_str(&format!("Bearer {token}")) {
            Ok(v) => v,
            Err(e) => {
                let formated_string = e.to_string();
                tracing::error!(formated_string);
                return Err(QStashError::TokenError);
            }
        };

        value.set_sensitive(true);
        let mut headers = header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, value);

        // initialize reqwest client
        let http = match reqwest::Client::builder().default_headers(headers).build() {
            Ok(c) => c,
            Err(e) => {
                let formated_string = e.to_string();
                tracing::error!(formated_string);
                return Err(QStashError::ReqwestError);
            }
        };

        let version = match version.unwrap_or(Version::V2) {
            Version::V1 => String::from("v1"),
            Version::V2 => String::from("v2"),
        };

        // parsing url from the provided value or use default
        let url = match Url::parse(base_url.unwrap_or("https://qstash.upstash.io")) {
            Ok(u) => u,
            Err(e) => {
                let formated_string = e.to_string();
                tracing::error!(formated_string);
                return Err(QStashError::InvalidUrl);
            }
        };

        Ok(Self {
            http,
            base_url: url,
            version,
        })
    }
}
