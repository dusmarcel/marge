use std::fs;
use std::io::Error;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    username: String,
    password: String,
    protocol: String,
    host: String,
    port: i32,
}

impl Config {
    pub fn new() -> Config {
        let username = "restadmin".to_string();
        let password = "restpass".to_string();
        let protocol = "http".to_string();
        let host = "localhost".to_string();
        let port = 8001;

        Config {
            username,
            password,
            protocol,
            host,
            port,
        }
    }

    pub fn new_from_file(config_dir: &PathBuf) -> Result<Config, Error> {
        let config: Config;
        let mut path = PathBuf::new();
        path.push(config_dir);
        path.push("config.json");
        let result = fs::File::open(path);
        match result {
            Ok(file) => {
                let result = serde_json::from_reader(file);
                match result {
                    Ok(jsession) => {
                        config = jsession;
                        Ok(config)
                    },
                    Err(e) => Err(e.into()),
                }
            },
            Err(e) => Err(e),
        }
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }

    pub fn password(&self) -> &String {
        &self.password
    }

    pub fn set_protocol(&mut self, protocol: String) {
        self.protocol = protocol;
    }

    pub fn protocol(&self) -> &String {
        &self.protocol
    }

    pub fn set_host(&mut self, host: String) {
        self.host = host;
    }

    pub fn host(&self) -> &String {
        &self.host
    }

    pub fn set_port(&mut self, port: i32) {
        self.port = port;
    }

    pub fn port(&self) -> i32 {
        self.port
    }

    pub fn save(&self, config_dir: &PathBuf) {
        let result = fs::create_dir_all(&config_dir);
        match result {
            Ok(_) => {
                let mut path = PathBuf::new();
                path.push(config_dir);
                path.push("config.json");
                let result = fs::File::create(&path);
                match result {
                    Ok(file) => {
                        let result = serde_json::to_writer_pretty(file, &self);
                        match result {
                            Ok(_) => (),
                            Err(e) => eprintln!("Error while writing config file: {}", e.to_string()),
                        }
                    }
                    Err(e) => eprintln!("Error while creating config file: {}", e.to_string()),
                }
            }
            Err(e) => eprintln!("Error while creating config directory: {}", e.to_string()),
        }
    }
}