use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Domains {
    entries: Option<Vec<Entry>>,
    http_etag: String,
    start: u32,
    total_size: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    alias_domain: Option<String>,
    description: Option<String>,
    http_etag: String,
    mail_host: String,
    self_link: String,
}

impl Domains {
    pub fn list_vec(&self) -> Vec<String> {
        if let Some(entries) = &self.entries {
            entries.iter().map(|entry| entry.mail_host()).collect()
        } else {
            Vec::new()
        }
    }

    pub fn entries(&self) -> Option<Vec<Entry>> {
        self.entries.clone()
    }
}

impl Entry {
    pub fn mail_host(&self) -> String {
        self.mail_host.clone()
    }
}