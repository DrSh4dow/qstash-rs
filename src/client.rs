use reqwest::{
    header::{self, HeaderMap},
    Method, Url,
};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone)]
pub enum QStashError {
    TokenError,
    ReqwestError,
    InvalidUrl,
    PublishError,
}

#[derive(Serialize, Default)]
pub struct PublishRequest<T> {
    /// The message to send.
    /// This can be anything as long it can be serialized.
    /// You can leave this empty if you want to send a message with no body.
    pub body: Option<T>,

    /// Optionally send along headers with the message.
    /// These headers will be sent to your destination.
    ///
    /// We highly recommend sending a `Content-Type` header along, as this will help your destination
    /// server to understand the content of the message.
    #[serde(skip_serializing)]
    headers: Option<HeaderMap>,

    /// Optionally delay the delivery of this message.
    /// In seconds.
    pub delay: Option<u32>,

    /// Optionally set the absolute delay of this message.
    /// This will override the delay option.
    /// The message will not delivered until the specified time.
    ///
    /// Unix timestamp in seconds.
    pub not_before: Option<u32>,

    /// Provide a unique id for deduplication. This id will be used to detect duplicate messages.
    /// If a duplicate message is detected, the request will be accepted but not enqueued.
    /// We store deduplication ids for 90 days. Afterwards it is possible that the message with the
    /// same deduplication id is delivered again.
    ///
    /// When scheduling a message, the deduplication happens before the schedule is created.
    pub deduplication_id: Option<String>,

    ///
    /// If true, the message content will get hashed and used as deduplication id.
    /// If a duplicate message is detected, the request will be accepted but not enqueued.
    ///
    /// The content based hash includes the following values:
    ///    - All headers, except Upstash-Authorization, this includes all headers you are sending.
    ///    - The entire raw request body The destination from the url path
    ///
    /// We store deduplication ids for 90 days. Afterwards it is possible that the message with the
    /// same deduplication id is delivered again.
    ///
    /// When scheduling a message, the deduplication happens before the schedule is created.
    content_based_deduplication: Option<bool>,

    ///
    /// In case your destination server is unavaialble or returns a status code outside of the 200-299
    /// range, we will retry the request after a certain amount of time.
    ///
    /// Configure how many times you would like the delivery to be retried
    ///
    /// @default The maximum retry quota associated with your account.
    ///
    retries: Option<u32>,

    ///
    /// Use a callback url to forward the response of your destination server to your callback url.
    ///
    /// The callback url must be publicly accessible
    ///
    /// @default None
    ///
    callback: Option<String>,

    ///
    ///The method to use when sending a request to your API
    ///
    ///@default `POST`
    ///
    #[serde(skip_serializing)]
    method: Option<Method>,
}

impl fmt::Display for QStashError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QStashError::TokenError => write!(f, "Could not parse token"),
            QStashError::ReqwestError => write!(f, "Reqwest failed to initialize"),
            QStashError::InvalidUrl => write!(f, "Invalid Url"),
            QStashError::PublishError => write!(f, "Error publishing message"),
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

    pub fn publish<T>(&self, request: PublishRequest<T>) -> Result<(), QStashError> {
        Ok(())
    }
}
