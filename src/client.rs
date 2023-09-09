use reqwest::{header, Url};
use std::fmt;

#[derive(Debug, Clone)]
pub enum QStashError {
    TokenError,
    ReqwestError,
    InvalidUrl,
}

impl fmt::Display for QStashError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QStashError::TokenError => write!(f, "Could not parse token"),
            QStashError::ReqwestError => write!(f, "Reqwest failed to initialize"),
            QStashError::InvalidUrl => write!(f, "Invalid Url"),
        }
    }
}

pub struct Client {
    pub http: reqwest::Client,
    base_url: Url,
}

impl Client {
    pub fn new(token: &str, base_url: Option<&str>) -> Result<Client, QStashError> {
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

        // parsing url from the provided value or use default
        let url = match Url::parse(base_url.unwrap_or("https://qstash.upstash.com")) {
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
        })
    }
}
