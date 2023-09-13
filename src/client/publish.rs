use reqwest::{
    header::{self, HeaderMap},
    Method,
};
use serde::Serialize;

use super::{
    error::QStashError, Client, PublishOptions, PublishRequest, PublishRequestUrl, QstashResponse,
};

impl Client {
    pub async fn publish<T: Into<reqwest::Body>>(
        &self,
        request: PublishRequest<T>,
    ) -> Result<Vec<QstashResponse>, QStashError> {
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

        let headers = match Client::generate_headers(PublishOptions {
            headers: request.headers,
            delay: request.delay,
            not_before: request.not_before,
            deduplication_id: request.deduplication_id,
            content_based_deduplication: request.content_based_deduplication,
            retries: request.retries,
            callback: request.callback,
            method: request.method,
        }) {
            Ok(h) => h,
            Err(e) => {
                let formated_string = e.to_string();
                tracing::error!(formated_string);
                return Err(QStashError::PublishError);
            }
        };

        let request_builder = self.http.request(Method::POST, path).headers(headers);

        let response = match request.body {
            Some(b) => match request_builder.body(b).send().await {
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
            None => match request_builder.send().await {
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

    /// publishJSON is a utility that automatically serializes the body
    /// and sets the `Content-Type` header to `application/json`.
    ///
    /// body can be any serializable type.
    ///
    ///
    /// # Example
    /// ```
    /// use qstash_rs::client::{PublishRequestUrl, Client};
    /// use std::collections::HashMap;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///
    /// let qstash_client = Client::new("<QSTASH_TOKEN>", None, None).expect("could not initialize client");
    ///
    ///
    /// match qstash_client
    ///     .publish_json(
    ///         PublishRequestUrl::Url("https://google.com".parse().expect("Could not parse URL")),
    ///         HashMap::from([("test", "test")]),
    ///         None,
    ///     )
    ///     .await {
    ///         Ok(r) => println!("{:?}",r),
    ///         Err(err) => println!("{:?}",err),
    ///     };
    ///
    /// }
    ///
    /// ```
    ///
    pub async fn publish_json<T: Serialize>(
        &self,
        url: PublishRequestUrl,
        body: T,
        options: Option<PublishOptions>,
    ) -> Result<Vec<QstashResponse>, QStashError> {
        let request_url = match &url {
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

        let headers = match options {
            Some(options) => match Client::generate_headers(options) {
                Ok(h) => h,
                Err(e) => {
                    let formated_string = e.to_string();
                    tracing::error!(formated_string);
                    return Err(QStashError::PublishError);
                }
            },
            None => header::HeaderMap::new(),
        };

        let response = match self
            .http
            .request(Method::POST, path)
            .headers(headers)
            .json(&body)
            .send()
            .await
        {
            Ok(r) => {
                tracing::debug!("{:?}", r);
                r
            }
            Err(e) => {
                let formated_string = e.to_string();
                tracing::error!(formated_string);
                return Err(QStashError::PublishError);
            }
        };

        let response: Vec<QstashResponse> = match url {
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

    /// generate_headers generates the headers for the request.
    /// The headers are generated from the provided options.
    /// If no options are provided, the default headers are used.
    fn generate_headers(request: PublishOptions) -> Result<HeaderMap, QStashError> {
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

        Ok(headers)
    }
}
