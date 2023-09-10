use std::fmt;

#[derive(Debug, Clone)]
pub enum QStashError {
    TokenError,
    ReqwestError,
    InvalidUrl,
    PublishError,
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
