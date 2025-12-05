use serde::{Deserialize, Serialize};

use crate::{Model, Role};

#[derive(Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub messages: Vec<Message>,
    pub model: Model,
    pub thinking: Option<Thinking>,
    pub stream: bool,
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
