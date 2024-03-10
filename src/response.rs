#[derive(Clone, Debug, PartialEq)]
pub enum ResponseType {
    Domains,
    Lists,
    Members,
    AddMember,
    Messages,
}

#[derive(Clone)]
pub struct Response {
    response_type: ResponseType,
    status: String,
    text: String,
}

impl Response {
    pub async fn new(result: Result<reqwest::Response, reqwest::Error>, response_type: ResponseType) -> Self {
        let status: String;
        let mut text = "Error (see status bar below for details)".to_string();
    
        match result {
            Ok(body) => {
                let bstatus = body.status();
                status = format!("{}: {}", bstatus.as_str(), bstatus.canonical_reason().unwrap());
                if bstatus == 200 {
                    text = body.text().await.unwrap();
                }
            }
            Err(e) => status = e.to_string(),
        }; 

        Self {
            response_type,
            status,
            text,
        }
    }

    pub fn response_type(&self) -> ResponseType {
        self.response_type.clone()
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }

    pub fn status(&self) -> String {
        self.status.clone()
    }
}