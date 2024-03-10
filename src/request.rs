use std::collections::HashMap;

use reqwest::{Method, Client, Url};

use crate::config::Config;

pub enum ReqType {
    Domains,
    Lists,
    Members,
    AddMember(String),
    Messages,
}

pub async fn request(client: &mut Client, req_t: ReqType, config: &Config) -> Result<reqwest::Response, reqwest::Error> {
    let mut method = Method::GET;
    let mut map: HashMap<String, String>  = HashMap::new();
    let url = match req_t {
        ReqType::Domains => {
            Url::parse(&format!("{}://{}:{}/3.1/domains",
                config.protocol(),
                config.host(),
                config.port())).unwrap()
        }
        ReqType::Lists => {
            Url::parse(&format!("{}://{}:{}/3.1/lists",
                config.protocol(),
                config.host(),
                config.port())).unwrap()
        }
        ReqType::Members => {
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
        ReqType::AddMember(address) => {
            method = Method::POST;
            map.insert("list_id".to_string(), config.list().unwrap().list_id());
            map.insert("subscriber".to_string(), address);
            map.insert("display_name".to_string(), "".to_string());
            map.insert("pre_verified".to_string(), "true".to_string());
            map.insert("pre_confirmed".to_string(), "true".to_string());
            map.insert("pre_approved".to_string(), "true".to_string());
            map.insert("send_welcome_message".to_string(), "false".to_string());
            Url::parse(&format!("{}://{}:{}/3.1/members",
                config.protocol(),
                config.host(),
                config.port())).unwrap()
        }
        ReqType::Messages => {
            Url::parse(&format!("{}://{}:{}/3.1/lists/{}/held",
                config.protocol(),
                config.host(),
                config.port(),
                config.list().unwrap().fqdn_listname())).unwrap()
        }
    };
    client.request(method, url)
        .basic_auth(config.username(), Some(config.password()))
        .json(&map)
        .send()
        .await
}