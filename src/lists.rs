use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lists {
    entries: Option<Vec<Entry>>,
    http_etag: String,
    start: u32,
    total_size: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    advertised: bool,
    description: Option<String>,
    display_name: String,
    fqdn_listname: String,
    http_etag: String,
    list_id: String,
    mail_host: String,
    member_count: u32,
    self_link: String,
    volume: u32,
}

impl Lists {
    pub fn list_vec(&self) -> Vec<String> {
        if let Some(entries) = &self.entries {
            entries.iter().map(|entry| entry.fqdn_listname.clone()).collect()
        } else {
            Vec::new()
        }
    }

    pub fn entries(&self) -> Option<Vec<Entry>> {
        self.entries.clone()
    }
}

impl Entry {
    pub fn display_name(&self) -> String {
        self.display_name.clone()
    }

    pub fn fqdn_listname(&self) -> String {
        self.fqdn_listname.clone()
    }

    pub fn list_id(&self) -> String {
        self.list_id.clone()
    }
}
