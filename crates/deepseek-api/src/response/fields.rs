use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    content: String,
    reasoning_content: Option<String>,
    tool_calls: Option<Vec<ToolCall>>,
    role: Role,
}

#[derive(Serialize, Deserialize, Debug)]
struct ToolCall {
    id: String,
    r#type: ToolCallType,
    function: Vec<Function>,
}

#[derive(Serialize, Deserialize, Debug)]
enum ToolCallType {
    #[serde(rename = "function")]
    Function,
}

#[derive(Serialize, Deserialize, Debug)]
struct Function {
    name: String,
    arguments: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FinishReason {
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "content_filter")]
    ContentFilter,
    #[serde(rename = "tool_calls")]
    ToolCalls,
    #[serde(rename = "insufficient_system_resource")]
    InsufficientSystemResource,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    completion_tokens: u64,
    prompt_tokens: u64,
    prompt_cache_hit_tokens: u64,
    prompt_cache_miss_tokens: u64,
    total_tokens: u64,
    completion_tokens_details: Option<CompletionTokensDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionTokensDetails {
    reasoning_tokens: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Role {
    #[serde(rename = "assistant")]
    Assistant,
}
