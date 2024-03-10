use reqwest::{Method, Client, Url};

use crate::config::Config;

pub enum Page {
    Domains,
    Lists,
    Members,
    Messages,
}

pub async fn request(client: &mut Client, page: Page, config: &Config) -> Result<reqwest::Response, reqwest::Error> {
    let url = match page {
        Page::Domains => {
            Url::parse(&format!("{}://{}:{}/3.1/domains",
                config.protocol(),
                config.host(),
                config.port())).unwrap()
        }
        Page::Lists => {
            Url::parse(&format!("{}://{}:{}/3.1/lists",
                config.protocol(),
                config.host(),
                config.port())).unwrap()
        }
        Page::Members => {
            if let Some(list) = config.list() {
                Url::parse(&format!("{}://{}:{}/3.1/lists/{}/roster/member",
                    config.protocol(),
                    config.host(),
                    config.port(),
                    list.fqdn_listname())).unwrap()                
            } else {
                Url::parse(&format!("{}://{}:{}/3.1/members",
                    config.protocol(),
                    config.host(),
                    config.port())).unwrap()
            }
        }
        Page::Messages => {
            Url::parse(&format!("{}://{}:{}/3.1/lists/{}/held",
                config.protocol(),
                config.host(),
                config.port(),
                config.list().unwrap().fqdn_listname())).unwrap()
        }
    };
    client.request(Method::GET, url)
        .basic_auth(config.username(), Some(config.password()))
        .send()
        .await
}