use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

impl OllamaRequest {
    pub fn new(model: String, prompt: String, stream: bool) -> Self {
        Self {
            model,
            prompt,
            stream,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct OllamaResponse {
    response: String,
}

impl OllamaResponse {
    pub fn response(&self) -> String {
        self.response.clone()
    }
}
