use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};

const MODEL_NAME: &str = "bonk";

pub struct ChatBot {
    ollama: Ollama,
}

impl ChatBot {
    pub fn new() -> Self {
        Self {
            ollama: Ollama::default(),
        }
    }

    pub async fn generate(&self, prompt: &str) -> String {
        let request = GenerationRequest::new(MODEL_NAME.to_string(), prompt.to_string());
        self.ollama.generate(request).await.unwrap().response // FIXME
    }
}

impl Default for ChatBot {
    fn default() -> Self {
        Self::new()
    }
}
