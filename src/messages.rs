use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Messages {
    entries: Vec<Entry>,
    http_etag: String,
    start: u32,
    total_size: u32,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    extra: u32,
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
       self.entries.iter().map(|entry| entry.desciption().clone()).collect() 
    }

    pub fn entries(&self) -> Vec<Entry> {
        self.entries.clone()
    }
}

impl Entry {
    pub fn desciption(&self) -> String {
        format!("{}: {}", self.sender, self.original_subject)
    }
}
