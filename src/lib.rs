//! # qstash-rs
//!
//! The `qstash-rs` crate provides a convenient client for the [QStash](https://upstash.com) API.
//!
//!
//! ## Client Usage
//!
//! ```rust
//! use qstash_rs::client::{Client, PublishRequest, PublishRequestUrl};
//!
//! #[tokio::main]
//! async fn main() {
//!    let qstash_client = Client::new("QSTASH_TOKEN", None, None).unwrap();
//!    match qstash_client
//!        .publish(PublishRequest::<String> {
//!            url: PublishRequestUrl::Url(
//!                "https://google.com"
//!                    .parse()
//!                    .expect("Could not convert to URL"),
//!            ),
//!            body: None,
//!            headers: None,
//!            delay: None,
//!            not_before: None,
//!            deduplication_id: None,
//!            content_based_deduplication: None,
//!            retries: None,
//!            callback: None,
//!            method: None,
//!        })
//!        .await
//!    {
//!        Ok(r) => {
//!            println!("Success: {:?}", r);
//!        }
//!        Err(e) => {
//!            panic!("Could not publish");
//!        }
//!    };
//! }
//!
//! ```

pub mod client;
