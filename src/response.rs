#[derive(Clone)]
pub struct Response {
    status: String,
}

impl Response {
    pub fn new(result: Result<reqwest::Response, reqwest::Error>) -> Self {
        let status: String;
    
        match result {
            Ok(body) => {
                let bstatus = body.status();
                status = format!("{}: {}", bstatus.as_str(), bstatus.canonical_reason().unwrap());
            }
            Err(e) => status = e.to_string(),
        }; 

        Self {
            status,
        }
    }

    pub fn status(self) -> String {
        self.status
    }
}