use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Members {
    entries: Option<Vec<Entry>>,
    http_etag: String,
    start: u32,
    total_size: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    address: String,
    bounce_score: u32,
    delivery_mode: String,
    display_name: String,
    email: String,
    http_etag: String,
    last_warning_sent: String,
    list_id: String,
    member_id: String,
    role: String,
    self_link: String,
    subscription_mode: String,
    total_warnings_sent: u32,
    user: String,
}

impl Members {
    pub fn list_vec(&self) -> Vec<String> {
        if let Some(entries) = &self.entries {
            entries.iter().map(|entry| entry.email.clone()).collect()
        } else {
            Vec::new()
        }
    }

    pub fn entries(&self) -> Option<Vec<Entry>> {
        self.entries.clone()
    }
}

// impl Entry {
//     pub fn email(&self) -> String {
//         self.email.clone()
//     }
// }
