use crate::settings;
use reqwest::{Client, RequestBuilder};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct OpenAIMessage {
    pub role: String,
    pub content: String,
}

#[derive(serde::Serialize, Debug)]
pub struct OpenAIRequest {
    pub model: String,
    pub messages: Vec<OpenAIMessage>,
    pub temperature: f32,
}

#[derive(serde::Deserialize, Debug)]
pub struct OpenAIUsage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

#[derive(serde::Deserialize, Debug)]
pub struct OpenAIChoice {
    pub message: OpenAIMessage,
    pub finish_reason: String,
    pub index: i8,
}

#[derive(serde::Deserialize, Debug)]
pub struct OpenAIResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub usage: OpenAIUsage,
    pub choices: Vec<OpenAIChoice>,
}

pub struct OpenAI {
    client: Client,
    api_token: String,
}

impl OpenAI {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            api_token: settings::Settings::get().open_ai_secret,
        }
    }

    pub fn chat(&self) -> OpenAIRequestBuilder {
        OpenAIRequestBuilder {
            request: self
                .client
                .post("https://api.openai.com/v1/chat/completions")
                .header("Authorization", self.bearer_token()),
        }
    }

    fn bearer_token(&self) -> String {
        "Bearer ".to_owned() + self.api_token.clone().as_str()
    }
}

pub struct OpenAIRequestBuilder {
    request: RequestBuilder,
}

impl OpenAIRequestBuilder {
    pub async fn create(&self, data: &OpenAIRequest) -> reqwest::Result<OpenAIResponse> {
        let response = self.request.try_clone().unwrap().json(data).send().await;

        match response {
            Ok(response) => response.json().await,
            Err(error) => Err(error),
        }
    }
}
