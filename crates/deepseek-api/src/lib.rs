mod request;
mod response;

use async_stream::stream;
use futures::Stream;
use request::ChatCompletionRequest;
use response::{Chunk, Response};
use serde::{Deserialize, Serialize};

pub use request::Message;

const API_URL: &str = "https://api.deepseek.com/chat/completions";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Model {
    #[serde(rename = "deepseek-chat")]
    DeepSeekChat,
    #[serde(rename = "deepseek-reasoner")]
    DeepSeekReasoner,
}

pub struct Client {
    model: Model,
    api_key: String,
}

impl Client {
    pub fn new(model: Model, api_key: &str) -> Self {
        Self {
            model,
            api_key: api_key.to_string(),
        }
    }

    #[must_use]
    pub async fn chat(&self, messages: Vec<Message>) -> Response {
        let client = reqwest::Client::new();
        let resp = client
            .post(API_URL)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(
                serde_json::to_string(&ChatCompletionRequest {
                    model: self.model.clone(),
                    messages: messages,
                    stream: false,
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
            .post(API_URL)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(
                serde_json::to_string(&ChatCompletionRequest {
                    model: self.model.clone(),
                    messages: messages,
                    stream: true,
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
}
