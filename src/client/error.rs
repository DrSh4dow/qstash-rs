//! # error module
//!
//! This module contains the error type for the crate.
//! It is used to return errors from the crate.

use std::fmt;

/// The error type for the crate.
/// It is used to return errors from the crate.
/// The errors are:
/// - TokenError: Could not parse token
/// - ReqwestError: Reqwest failed to initialize
/// - InvalidUrl: Invalid Url
/// - PublishError: Error publishing message
/// - EventError: Error getting events
/// - DeadLetterQueueError: Error getting DLQ List
#[derive(Debug, Clone)]
pub enum QStashError {
    TokenError,
    ReqwestError,
    InvalidUrl,
    PublishError,
    EventError,
    DeadLetterQueueError,
    GetMessageError,
    DeleteMessageError,
}

impl fmt::Display for QStashError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QStashError::TokenError => write!(f, "Could not parse token"),
            QStashError::ReqwestError => write!(f, "Reqwest failed to initialize"),
            QStashError::InvalidUrl => write!(f, "Invalid Url"),
            QStashError::PublishError => write!(f, "Error publishing message"),
            QStashError::EventError => write!(f, "Error getting events"),
            QStashError::DeadLetterQueueError => write!(f, "Error getting DLQ List"),
            QStashError::GetMessageError => write!(f, "Error getting message"),
            QStashError::DeleteMessageError => write!(f, "Error deleting message"),
        }
    }
}
