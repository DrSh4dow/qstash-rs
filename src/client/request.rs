use reqwest::{header::HeaderMap, Method};

#[derive(Debug, Clone)]
pub enum PublishRequestUrl {
    Url(reqwest::Url),
    Topic(String),
}

#[derive(Debug, Clone)]
pub struct PublishRequest<T>
where
    T: Into<reqwest::Body>,
{
    pub url: PublishRequestUrl,
    /// The message to send.
    /// This can be anything, but please set the `Content-Type` header accordingly.
    /// You can leave this empty if you want to send a message with no body.
    pub body: Option<T>,

    /// Optionally send along headers with the message.
    /// These headers will be sent to your destination.
    ///
    /// We highly recommend sending a `Content-Type` header along, as this will help your destination
    /// server to understand the content of the message.
    pub headers: Option<HeaderMap>,

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
    pub content_based_deduplication: Option<bool>,

    ///
    /// In case your destination server is unavaialble or returns a status code outside of the 200-299
    /// range, we will retry the request after a certain amount of time.
    ///
    /// Configure how many times you would like the delivery to be retried
    ///
    /// @default The maximum retry quota associated with your account.
    ///
    pub retries: Option<u32>,

    ///
    /// Use a callback url to forward the response of your destination server to your callback url.
    ///
    /// The callback url must be publicly accessible
    ///
    /// @default None
    ///
    pub callback: Option<String>,

    ///
    ///The method to use when sending a request to your API
    ///
    ///@default `POST`
    ///
    pub method: Option<Method>,
}

impl<T: Into<reqwest::Body>> PublishRequest<T> {
    pub fn new(url: PublishRequestUrl) -> Self {
        Self {
            url,
            body: None,
            headers: None,
            delay: None,
            not_before: None,
            deduplication_id: None,
            content_based_deduplication: None,
            retries: None,
            callback: None,
            method: None,
        }
    }
}
