use reqwest::{Method, Client, Url};

use crate::config::Config;

pub enum Page {
    Domains,
    Lists,
    Members,
    Messages,
}

pub async fn request(client: &mut Client, page: Page, config: &Config) -> Result<reqwest::Response, reqwest::Error> {
    client.request(Method::GET, Url::parse(&format!("{}://{}:{}/3.1/domains",
        config.protocol(),
        config.host(),
        config.port())).unwrap())
        .basic_auth(config.username(), Some(config.password()))
        .send()
        .await
}