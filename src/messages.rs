use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Messages {
    entries: Option<Vec<Entry>>,
    http_etag: String,
    start: u32,
    total_size: u32,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    extra: Option<u32>,
    hold_date: String,
    http_etag: String,
    message_id: String,
    msg: String,
    original_subject: String,
    reason: String,
    request_id: u32,
    self_link: String,
    sender: String,
    subject: String,
}

impl Messages {
    pub fn list_vec(&self) -> Vec<String> {
        if let Some(entries) = &self.entries {
            entries.iter().map(|entry| entry.description().clone()).collect()
        } else {
            vec!["No held messages ".to_string()]
        }
    }

    pub fn entries(&self) -> Option<Vec<Entry>> {
        self.entries.clone()
    }
}

impl Entry {
    pub fn description(&self) -> String {
        format!("{}: {}", self.sender, self.original_subject)
    }

    pub fn request_id(&self) -> u32 {
        self.request_id
    }
}
