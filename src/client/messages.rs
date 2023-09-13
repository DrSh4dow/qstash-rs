//! # messages module
//! This module contains the methods implementation required to interact with the messages endpoint.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::client::error::QStashError;

use super::Client;

/// The message struct.
/// It contains the message_id, url, topic_name, endpoint_name, key, method, header, body, max_retries, not_before, created_at and callback.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub message_id: String,
    pub url: String,
    pub topic_name: Option<String>,
    pub endpoint_name: Option<String>,
    pub key: Option<String>,
    pub method: Option<String>,
    pub header: Option<HashMap<String, Vec<String>>>,
    pub body: Option<String>,
    pub max_retries: Option<u32>,
    pub not_before: Option<u64>,
    pub created_at: u64,
    pub callback: Option<String>,
}

impl Client {
    /// get_message Retrieve a message by its id
    pub async fn get_message(&self, message_id: &str) -> Result<Message, QStashError> {
        let path = match self
            .base_url
            .join(&format!("/{}/messages/{}", self.version, message_id))
        {
            Ok(p) => p,
            Err(e) => {
                let formated_string = e.to_string();
                tracing::error!(formated_string);
                return Err(QStashError::GetMessageError);
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
                return Err(QStashError::GetMessageError);
            }
        };

        let response = match response.json().await {
            Ok(r) => r,
            Err(e) => {
                let formated_string = e.to_string();
                tracing::error!(formated_string);
                return Err(QStashError::GetMessageError);
            }
        };

        Ok(response)
    }

    /// cancel_message cancels the message with the given id.
    /// Cancelling a message will remove it from QStash and stop it from being delivered in the future.
    /// If a message is in flight to your API, it might be too late to cancel.
    pub async fn cancel_message(&self, message_id: &str) -> Result<(), QStashError> {
        let path = match self
            .base_url
            .join(&format!("/{}/messages/{}", self.version, message_id))
        {
            Ok(p) => p,
            Err(e) => {
                let formated_string = e.to_string();
                tracing::error!(formated_string);
                return Err(QStashError::DeleteMessageError);
            }
        };

        match self.http.delete(path).send().await {
            Ok(r) => {
                tracing::debug!("{:?}", r);
                if r.status().is_success() {
                    Ok(())
                } else {
                    Err(QStashError::DeleteMessageError)
                }
            }
            Err(e) => {
                let formated_string = e.to_string();
                tracing::error!(formated_string);
                Err(QStashError::DeleteMessageError)
            }
        }
    }
}
