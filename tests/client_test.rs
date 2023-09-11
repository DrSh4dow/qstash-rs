use qstash_rs::client::{Client, PublishRequest};
use serde::Deserialize;
use std::sync::Once;
use tracing_test::traced_test;

static INIT: Once = Once::new();

#[derive(Deserialize, Debug)]
struct Config {
    qstash_token: String,
}

fn prepare() -> Config {
    INIT.call_once(|| {
        dotenvy::dotenv().expect(".dotenv not found");
    });

    match envy::from_env::<Config>() {
        Ok(c) => c,
        Err(error) => panic!("{:#?}", error),
    }
}

#[test]
#[traced_test]
fn client_instantiate_should_work() {
    let config = prepare();
    match Client::new(&config.qstash_token, None, None) {
        Ok(_) => {
            tracing::info!("Client initialized successfully!");
        }
        Err(e) => {
            tracing::error!("{}", e);
            panic!("Could not initialize client");
        }
    };
}

#[tokio::test]
#[traced_test]
async fn publish_should_work() {
    let config = prepare();
    let qstash_client = match Client::new(&config.qstash_token, None, None) {
        Ok(c) => {
            tracing::info!("Client initialized successfully!");
            c
        }
        Err(e) => {
            tracing::error!("{}", e.to_string());
            panic!("Could not initialize client");
        }
    };

    match qstash_client
        .publish(PublishRequest::<String> {
            url: qstash_rs::client::PublishRequestUrl::Url(
                "https://google.com"
                    .parse()
                    .expect("Could not convert to URL"),
            ),
            body: None,
            headers: None,
            delay: None,
            not_before: None,
            deduplication_id: None,
            content_based_deduplication: None,
            retries: None,
            callback: None,
            method: None,
        })
        .await
    {
        Ok(r) => {
            tracing::info!("Success: {:?}", r);
        }
        Err(e) => {
            tracing::error!("{}", e.to_string());
            panic!("Could not publish");
        }
    };
}

#[tokio::test]
#[traced_test]
async fn publish_should_contain_error() {
    let qstash_client = match Client::new("false_token", None, None) {
        Ok(c) => {
            tracing::info!("Client initialized successfully!");
            c
        }
        Err(e) => {
            tracing::error!("{}", e.to_string());
            panic!("Could not initialize client");
        }
    };

    match qstash_client
        .publish(PublishRequest::<String> {
            url: qstash_rs::client::PublishRequestUrl::Url(
                "https://google.com"
                    .parse()
                    .expect("could not convert to URL"),
            ),
            body: None,
            headers: None,
            delay: None,
            not_before: None,
            deduplication_id: None,
            content_based_deduplication: None,
            retries: None,
            callback: None,
            method: None,
        })
        .await
    {
        Ok(r) => {
            tracing::info!("Success: {:?}", r);
            for res in r {
                if res.error.is_none() {
                    panic!("This should have an error");
                }
            }
        }
        Err(e) => {
            tracing::error!("{}", e.to_string());
            panic!("Could not publish");
        }
    };
}
