use serde::{Deserialize, Serialize};
use regex::Regex;

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
        let re = Regex::new(r"(?s)<think>.*?</think>").unwrap();
        let result = re.replace_all(&self.response, "")
            .replace("```", "``")
            .replace("**", "*")
            .trim()
            .to_string();
        // println!("Risposta ricevuta: {}", result);
        result
    }
}
