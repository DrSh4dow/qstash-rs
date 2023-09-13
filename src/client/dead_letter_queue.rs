//! # dead_letter_queue module
//! This module contains the methods implementation required to interact with the dead letter queue endpoint.

use reqwest::Response;

use crate::client::error::QStashError;

use super::Client;

/// The dead letter queue request.
/// It contains the cursor.
#[derive(Debug)]
pub struct DlqRequest {
    pub cursor: Option<u32>,
}

impl Client {
    /// Retrieve your dead letter queue.
    /// this method is not public yet because there is currently a bug in Upstash's REST API.
    async fn _get_dead_letter_queue(
        &self,
        request: Option<DlqRequest>,
    ) -> Result<Response, QStashError> {
        let mut path = match self
            .base_url
            .join(&format!("/{}/dlq/messages", self.version))
        {
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

        // let response = match response.json().await {
        //     Ok(r) => r,
        //     Err(e) => {
        //         let formated_string = e.to_string();
        //         tracing::error!(formated_string);
        //         return Err(QStashError::EventError);
        //     }
        // };

        Ok(response)
    }
}
