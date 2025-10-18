use serde::{Deserialize, Serialize};

use crate::{Model, Role};

#[derive(Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub messages: Vec<Message>,
    pub model: Model,
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
