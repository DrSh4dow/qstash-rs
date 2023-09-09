pub struct Client {
    pub http: reqwest::Client,
    pub base_url: String,
    pub token: String,
}

impl Client {
    pub fn new(token: String, base_url: Option<String>) -> Client {
        Client {
            http: reqwest::Client::new(),
            token,
            base_url: match base_url {
                Some(b) => b,
                None => "https://qstash.upstash.io".to_owned(),
            },
        }
    }
}
