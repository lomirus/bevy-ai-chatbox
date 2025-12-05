mod request;
mod response;

use async_stream::stream;
use futures::Stream;
use request::ChatCompletionRequest;
use response::{Response, streaming::Chunk};
use serde::{Deserialize, Serialize};

pub use request::Message;
pub use response::{FinishReason, streaming};

use crate::{request::Thinking, response::UserBalance};

const BASE_URL: &str = "https://api.deepseek.com";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Model {
    #[serde(rename = "deepseek-chat")]
    DeepSeekChat,
    #[serde(rename = "deepseek-reasoner")]
    DeepSeekReasoner,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Role {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "tool")]
    Tool,
}

pub struct Client {
    pub model: Model,
    pub thinking: Option<Thinking>,
    pub api_key: String,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.
    pub frequency_penalty: Option<f32>,

    /// The maximum number of tokens that can be generated in the chat completion.
    ///
    /// The total length of input tokens and generated tokens is limited by the model's context length.
    ///
    /// For the value range and default value, please refer to the [documentation](https://api-docs.deepseek.com/quick_start/pricing).
    pub max_tokens: Option<u32>,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.
    pub presence_penalty: Option<f32>,

    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
    ///
    /// We generally recommend altering this or `top_p` but not both.
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// We generally recommend altering this or `temperature`` but not both.
    pub top_p: Option<f32>,
}

impl Client {
    #[must_use]
    pub fn new(model: Model, api_key: &str) -> Self {
        Self {
            model,
            thinking: None,
            api_key: api_key.to_string(),
            frequency_penalty: None,
            max_tokens: None,
            presence_penalty: None,
            temperature: None,
            top_p: None,
        }
    }

    #[must_use]
    pub async fn chat(&self, messages: Vec<Message>) -> Response {
        let client = reqwest::Client::new();
        let resp = client
            .post(format!("{BASE_URL}/chat/completions"))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(
                serde_json::to_string(&ChatCompletionRequest {
                    model: self.model.clone(),
                    messages,
                    thinking: self.thinking.clone(),
                    stream: false,
                    frequency_penalty: self.frequency_penalty,
                    max_tokens: self.max_tokens,
                    presence_penalty: self.presence_penalty,
                    temperature: self.temperature,
                    top_p: self.top_p,
                })
                .unwrap(),
            )
            .send()
            .await
            .unwrap();
        resp.json::<Response>().await.unwrap()
    }

    #[must_use]
    pub async fn streaming_chat(&self, messages: Vec<Message>) -> impl Stream<Item = Chunk> {
        let client = reqwest::Client::new();
        let mut resp = client
            .post(format!("{BASE_URL}/chat/completions"))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(
                serde_json::to_string(&ChatCompletionRequest {
                    model: self.model.clone(),
                    messages,
                    thinking: self.thinking.clone(),
                    stream: true,
                    frequency_penalty: self.frequency_penalty,
                    max_tokens: self.max_tokens,
                    presence_penalty: self.presence_penalty,
                    temperature: self.temperature,
                    top_p: self.top_p,
                })
                .unwrap(),
            )
            .send()
            .await
            .unwrap();

        stream! {
            while let Some(chunk) = resp.chunk().await.unwrap() {
                let s = String::from_utf8(chunk.to_vec()).unwrap();
                for data in s.trim().split("\n\n").map(|s| s[6..].to_string()) {
                    if data == "[DONE]" {
                        break;
                    }
                    yield serde_json::from_str(&data).unwrap();
                }
            }
        }
    }

    #[must_use]
    /// Get user current balance
    pub async fn user_balance(&self) -> UserBalance {
        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{BASE_URL}/user/balance"))
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .unwrap();
        resp.json().await.unwrap()
    }
}
