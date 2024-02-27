use color_eyre::eyre::Result;
use futures::Future;
use reqwest::{Client, Response, Error};

use crate::config::Config;

#[derive(Clone)]
pub struct Request {
    config: Config,
    client: Client,
}

impl Request {
    pub fn new() -> Self {
        let config = Config::new();
        let client = Client::new();

        Self {
            config,
            client,
        }
    }

    pub fn set_config(&mut self, config:Config) {
        self.config = config;
    }

    pub fn send(&self) -> impl Future<Output = Result<Response, Error>> {
        self.client.get(format!("{}://{}:{}/3.1/domains",
                        self.config.protocol(),
                        self.config.host(),
                        self.config.port()))
                        .basic_auth(self.config.username(), Some(self.config.password()))
                        .send()
    }
}