use qstash_rs::{self, Client};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    token: String,
}

fn prepare() -> Config {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().expect(".dotenv not found");

    match envy::from_env::<Config>() {
        Ok(c) => c,
        Err(error) => panic!("{:#?}", error),
    }
}

#[test]
fn client_instantiate_should_work() {
    let config = prepare();
    match Client::new(&config.token, None, None) {
        Ok(_) => {
            tracing::info!("Client initialized successfully!");
        }
        Err(e) => {
            tracing::error!("{}", e);
            panic!("Could not initialize client");
        }
    };
}
