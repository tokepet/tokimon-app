use serde::{Deserialize, Serialize};

/// A single normalized token-usage record collected from a provider's local
/// telemetry. The collector only stores token counts and identifiers — never
/// prompt/response/code content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UsageEvent {
    pub provider: String,
    pub tool: String,
    pub model: String,
    pub timestamp: String,
    pub session_id: Option<String>,
    pub prompt_id: Option<String>,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub cached_input_tokens: i64,
    pub cache_creation_tokens: i64,
    pub reasoning_tokens: i64,
    pub thoughts_tokens: i64,
    pub tool_tokens: i64,
    pub total_tokens: i64,
    pub source_type: String,
}
