## qstash-rs 🦀: Upstash QStash SDK for Rust

[![Crates.io](https://img.shields.io/crates/v/qstash-rs)](https://crates.io/crates/qstash-rs)

- [qstash-rs 🦀: Upstash QStash SDK for Rust](#qstash-rs-🦀-upstash-qstash-sdk-for-rust)
  - [About](#about)
  - [Installation](#installation)
  - [Client Usage](#client-usage)

### About

`qstash-rs` is a Rust library for interacting with [Upstash QStash](https://upstash.com/). It contains a client and a server (TODO) module.
The client library it is a wrapper around the [Upstash QStash REST API](https://docs.upstash.com/).

### Installation

You can install `qstash-rs` with `cargo`:

```bash
cargo add qstash-rs
```

### Client Usage

To start using the client SDK, you need to instantiate the `Client` struct with your QStash token:

```rust
#[tokio::main]
async fn main() {
  let qstash_client = Client::new("<QSTASH_TOKEN>", None, None).expect("Could not create client");
}
```

Then you can access any of the methods that the client supports. For example
to publish a new message with a JSON body to a queue:

```rust
#[tokio::main]
async fn main() {
    let qstash_client = Client::new("<QSTASH_TOKEN>", None, None).expect("Could not create client");

    match qstash_client
        .publish_json(
            PublishRequestUrl::Url("https://google.com".parse().expect("Could not parse URL")),
            HashMap::from([("test", "test")]),
            None,
        )
        .await
    {
        Ok(r) => {
            tracing::info!("Response: {:?}", r);
            for res in r {
                if res.error.is_some() {
                    panic!("This should NOT have an error");
                }
            }
        }
        Err(e) => {
            tracing::error!("{}", e.to_string());
            panic!("Could not publish");
        }
    };
}
```

A more comprehensive example can be found in the [crate documentation](https://docs.rs/qstash-rs/latest/qstash_rs/index.html)

### Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue if you have a problem, question, or suggestion.

### License

This project operates under the MIT License. Details in the [LICENSE](https://github.com/drsh4dow/qstash-rs/blob/main/LICENSE) file.
