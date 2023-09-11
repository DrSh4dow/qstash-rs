//! # qstash-rs
//!
//! qstash-rs is a crate that provides a convenient rust SDK
//! for the [Upstash QStash](https://upstash.com) API.
//!
//!
//! ## Publishing a JSON message
//!
//! The publish_json method provides a convenient way to easily publish a
//! message to qstash.
//!
//! ```rust
//! use qstash_rs::client::{Client, PublishRequest, PublishRequestUrl};
//! use std::collections::HashMap;
//!
//! #[tokio::main]
//! async fn main() {
//!    let qstash_client = Client::new("<QSTASH_TOKEN>", None, None).unwrap();
//!    match qstash_client
//!        .publish_json(
//!            PublishRequestUrl::Url("https://google.com".parse().expect("Could not parse URL")),
//!            HashMap::from([("test", "test")]),
//!            None,
//!        )
//!        .await {
//!            Ok(r) => println!("{:?}",r),
//!            Err(err) => println!("{:?}",err),
//!        };
//! }
//! ```
//!
//! If you wish to add extra options to this method use the third parameter options
//! of type [`Option<PublishOptions>`](client/struct.PublishOptions.html)
//!
//! ## Publishing any message
//!
//! whilst the publish_json method provides a convenient and more straightforward way
//! to call the API, the publish method provides all the available options that
//! qstash offers through their REST API.
//!
//! ```rust
//! use qstash_rs::client::{Client, PublishRequest, PublishRequestUrl};
//!
//! #[tokio::main]
//! async fn main() {
//!    let qstash_client = Client::new("<QSTASH_TOKEN>", None, None).unwrap();
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
//! **Note**: Replace <QSTASH_TOKEN> with your actual API token.
//!
//! Happy coding!

pub mod client;

