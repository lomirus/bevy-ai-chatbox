use serde::{Deserialize, Serialize};

use crate::{Model, Role};

#[derive(Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub messages: Vec<Message>,
    pub model: Model,
    pub thinking: Option<Thinking>,
    pub stream: bool,

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub content: String,
    pub role: Role,
}

impl Message {
    pub fn system(content: &str) -> Self {
        Self {
            content: content.to_string(),
            role: Role::System,
        }
    }

    pub fn user(content: &str) -> Self {
        Self {
            content: content.to_string(),
            role: Role::User,
        }
    }

    pub fn assistant(content: &str) -> Self {
        Self {
            content: content.to_string(),
            role: Role::Assistant,
        }
    }

    pub fn tool(content: &str) -> Self {
        Self {
            content: content.to_string(),
            role: Role::Tool,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Thinking {
    pub r#type: ThinkingType,
}

impl Thinking {
    pub const fn enabled() -> Self {
        Thinking {
            r#type: ThinkingType::Enabled,
        }
    }

    pub const fn disabled() -> Self {
        Thinking {
            r#type: ThinkingType::Disabled,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ThinkingType {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}
