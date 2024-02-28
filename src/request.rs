use reqwest::Client;

use crate::config::Config;

#[derive(Clone)]
pub struct Request {
    config: Config,
    client: Client,
    status_string: String,
}

impl Request {
    pub fn new() -> Self {
        let config = Config::new();
        let client = Client::new();
        let status_string = String::new();

        Self {
            config,
            client,
            status_string,
        }
    }

    pub fn set_config(&mut self, config:Config) {
        self.config = config;
    }

    pub async fn send(&mut self) {
        let response = self.client.get(format!("{}://{}:{}/3.1/domains",
                        self.config.protocol(),
                        self.config.host(),
                        self.config.port()))
                        .basic_auth(self.config.username(), Some(self.config.password()))
                        .send()
                        .await;

        match response {
            Ok(body) => {
                let status = body.status();
                self.status_string = format!("{}: {}", status.as_str(), status.canonical_reason().unwrap());
            }
            Err(e) => self.status_string = e.to_string(),
        };       
    }

    pub fn status_string(&self) -> String {
        self.status_string.clone()
    }
}