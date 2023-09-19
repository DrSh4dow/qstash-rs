//! # dead_letter_queue module
//! This module contains the methods implementation required to interact with the dead letter queue endpoint.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::client::error::QStashError;

use super::Client;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DlqMessage {
    pub message_id: String,
    pub url: String,
    pub topic_name: Option<String>,
    pub endpoint_name: Option<String>,
    pub key: Option<String>,
    pub method: String,
    pub header: Option<HashMap<String, Vec<String>>>,
    pub body: Option<String>,
    pub max_retries: Option<u32>,
    pub not_before: Option<u64>,
    pub created_at: u64,
    pub callback: Option<String>,
    pub dlq_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DlqResponse {
    pub messages: Vec<DlqMessage>,
}

/// The dead letter queue request.
/// It contains the optional cursor.
#[derive(Debug)]
pub struct DlqRequest {
    pub cursor: Option<u32>,
}

impl Client {
    /// Retrieve your dead letter queue.
    pub async fn get_dead_letter_queue(
        &self,
        request: Option<DlqRequest>,
    ) -> Result<DlqResponse, QStashError> {
        let mut path = match self.base_url.join(&format!("/{}/dlq", self.version)) {
            Ok(p) => p,
            Err(e) => {
                let formated_string = e.to_string();
                tracing::error!(formated_string);
                return Err(QStashError::PublishError);
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
