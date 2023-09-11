//! # QStash client.
//! This is the main struct you will use to interact with the QStash API.
//! It is initialized with a token and optionally a base url and a version.
//! The default base url is <https://qstash.upstash.io>.


mod error;
mod request;

use error::*;
pub use request::*;

use reqwest::{
    header::{self},
    Url,
};
use serde::{Deserialize, Serialize};

/// The version of the QStash API to use.
/// The default is V2.
pub enum Version {
    V1,
    V2,
}

/// The response from the QStash API.
/// If the request is successful, the response will contain a message_id and a url.
/// The url is the url of the message in the queue.
/// If the request is not successful, the response will contain an error.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QstashResponse {
    pub message_id: Option<String>,
    pub url: Option<String>,
    pub error: Option<String>,
    pub deduplicated: Option<bool>,
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

    pub async fn publish<T: Into<reqwest::Body>>(
        &self,
        request: PublishRequest<T>,
    ) -> Result<Vec<QstashResponse>, QStashError> {
        let mut headers = request.headers.unwrap_or(header::HeaderMap::new());

        let method = match header::HeaderValue::from_str(
            request.method.unwrap_or(reqwest::Method::POST).as_str(),
        ) {
            Ok(v) => v,
            Err(e) => {
                let formated_string = e.to_string();
                tracing::error!(formated_string);
                return Err(QStashError::PublishError);
            }
        };
        headers.insert("Upstash-Method", method);

        if let Some(delay) = request.delay {
            let delay = match header::HeaderValue::from_str(&format!("{}s", delay)) {
                Ok(v) => v,
                Err(e) => {
                    let formated_string = e.to_string();
                    tracing::error!(formated_string);
                    return Err(QStashError::PublishError);
                }
            };
            headers.insert("Upstash-Delay", delay);
        }

        if let Some(not_before) = request.not_before {
            let not_before = match header::HeaderValue::from_str(&format!("{}", not_before)) {
                Ok(v) => v,
                Err(e) => {
                    let formated_string = e.to_string();
                    tracing::error!(formated_string);
                    return Err(QStashError::PublishError);
                }
            };
            headers.insert("Upstash-Not-Before", not_before);
        }

        if let Some(deduplication_id) = request.deduplication_id {
            let deduplication_id = match header::HeaderValue::from_str(&deduplication_id) {
                Ok(v) => v,
                Err(e) => {
                    let formated_string = e.to_string();
                    tracing::error!(formated_string);
                    return Err(QStashError::PublishError);
                }
            };
            headers.insert("Upstash-Deduplication-Id", deduplication_id);
        }

        if let Some(content_based_deduplication) = request.content_based_deduplication {
            let content_based_deduplication =
                match header::HeaderValue::from_str(match content_based_deduplication {
                    true => "true",
                    false => "false",
                }) {
                    Ok(v) => v,
                    Err(e) => {
                        let formated_string = e.to_string();
                        tracing::error!(formated_string);
                        return Err(QStashError::PublishError);
                    }
                };
            headers.insert(
                "Upstash-Content-Based-Deduplication",
                content_based_deduplication,
            );
        }

        if let Some(retries) = request.retries {
            let retries = match header::HeaderValue::from_str(&format!("{}", retries)) {
                Ok(v) => v,
                Err(e) => {
                    let formated_string = e.to_string();
                    tracing::error!(formated_string);
                    return Err(QStashError::PublishError);
                }
            };
            headers.insert("Upstash-Retries", retries);
        }

        if let Some(callback) = request.callback {
            let callback = match header::HeaderValue::from_str(&callback) {
                Ok(v) => v,
                Err(e) => {
                    let formated_string = e.to_string();
                    tracing::error!(formated_string);
                    return Err(QStashError::PublishError);
                }
            };
            headers.insert("Upstash-Callback", callback);
        }

        let request_url = match &request.url {
            PublishRequestUrl::Url(v) => v.to_string(),
            PublishRequestUrl::Topic(v) => v.clone(),
        };

        let path = match self
            .base_url
            .join(&format!("/{}/publish/{}", self.version, request_url))
        {
            Ok(p) => p,
            Err(e) => {
                let formated_string = e.to_string();
                tracing::error!(formated_string);
                return Err(QStashError::PublishError);
            }
        };

        let response = match request.body {
            Some(b) => match self.http.post(path).headers(headers).body(b).send().await {
                Ok(r) => {
                    tracing::debug!("{:?}", r);
                    r
                }
                Err(e) => {
                    let formated_string = e.to_string();
                    tracing::error!(formated_string);
                    return Err(QStashError::PublishError);
                }
            },
            None => match self.http.post(path).headers(headers).send().await {
                Ok(r) => {
                    tracing::debug!("{:?}", r);
                    r
                }
                Err(e) => {
                    let formated_string = e.to_string();
                    tracing::error!(formated_string);
                    return Err(QStashError::PublishError);
                }
            },
        };

        let response: Vec<QstashResponse> = match request.url {
            PublishRequestUrl::Url(_) => match response.json().await {
                Ok(r) => vec![r],
                Err(e) => {
                    let formated_string = e.to_string();
                    tracing::error!(formated_string);
                    return Err(QStashError::PublishError);
                }
            },
            PublishRequestUrl::Topic(_) => match response.json().await {
                Ok(r) => r,
                Err(e) => {
                    let formated_string = e.to_string();
                    tracing::error!(formated_string);
                    return Err(QStashError::PublishError);
                }
            },
        };

        Ok(response)
    }
}
