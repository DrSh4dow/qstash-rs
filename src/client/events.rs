//! # events module
//! This module contains the methods implementation required to interact with the events endpoint.
//! The events endpoint is used to retrieve your logs.

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use super::{error::QStashError, Client};

/// The state of the message.
#[derive(Debug, Serialize, Deserialize, Default)]
pub enum State {
    CREATED,
    ACTIVE,
    DELIVERED,
    #[default]
    ERROR,
    CANCELED,
    RETRY,
    FAILED,
}

/// The event struct.
/// It contains the time, state, message_id, next_delivery_time, error, url, topic_name and endpoint_name.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub time: u64,
    #[serde(deserialize_with = "ok_or_default")]
    pub state: State,
    pub message_id: String,
    pub next_delivery_time: Option<u64>,
    pub error: Option<String>,
    pub url: Option<String>,
    pub topic_name: Option<String>,
    pub endpoint_name: Option<String>,
}

/// Deserialize with default value.
fn ok_or_default<'t, 'd, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'t> + Default,
    D: Deserializer<'d>,
{
    let v: Value = Deserialize::deserialize(deserializer)?;
    Ok(T::deserialize(v).unwrap_or_default())
}

/// The event request.
/// It contains the cursor.
#[derive(Debug)]
pub struct EventRequest {
    pub cursor: Option<u32>,
}

/// The event response.
/// It contains the cursor and the events.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetEventsResponse {
    pub cursor: Option<String>,
    pub events: Vec<Event>,
}

impl Client {
    /// Retrieve your logs.
    ///
    /// The logs endpoint is paginated and returns only 100 logs at a time.
    /// If you want to receive more logs, you can use the cursor to paginate.
    ///
    /// The cursor is a unix timestamp with millisecond precision
    ///
    /// @example
    /// ```rust
    /// ```
    pub async fn get_events(
        &self,
        request: Option<EventRequest>,
    ) -> Result<GetEventsResponse, QStashError> {
        let mut path = match self.base_url.join(&format!("/{}/events", self.version)) {
            Ok(p) => p,
            Err(e) => {
                let formated_string = e.to_string();
                tracing::error!(formated_string);
                return Err(QStashError::EventError);
            }
        };

        if let Some(request) = request {
            if let Some(cursor) = request.cursor {
                path.set_query(Some(&format!("cursor={}", cursor)));
            }
        };

        let response = match self.http.get(path).send().await {
            Ok(r) => {
                tracing::debug!("{:?}", r);
                r
            }
            Err(e) => {
                let formated_string = e.to_string();
                tracing::error!(formated_string);
                return Err(QStashError::EventError);
            }
        };

        let response = match response.json().await {
            Ok(r) => r,
            Err(e) => {
                let formated_string = e.to_string();
                tracing::error!(formated_string);
                return Err(QStashError::EventError);
            }
        };

        Ok(response)
    }
}
