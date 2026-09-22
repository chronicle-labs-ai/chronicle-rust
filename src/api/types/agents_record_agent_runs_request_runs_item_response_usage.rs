pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecordAgentRunsRequestRunsItemResponseUsage {
    #[serde(rename = "cachedInputTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_input_tokens: Option<i64>,
    #[serde(rename = "inputTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_tokens: Option<i64>,
    #[serde(rename = "outputTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_tokens: Option<i64>,
    #[serde(rename = "reasoningTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_tokens: Option<i64>,
    #[serde(rename = "totalTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<i64>,
}

impl RecordAgentRunsRequestRunsItemResponseUsage {
    pub fn builder() -> RecordAgentRunsRequestRunsItemResponseUsageBuilder {
        <RecordAgentRunsRequestRunsItemResponseUsageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecordAgentRunsRequestRunsItemResponseUsageBuilder {
    cached_input_tokens: Option<i64>,
    input_tokens: Option<i64>,
    output_tokens: Option<i64>,
    reasoning_tokens: Option<i64>,
    total_tokens: Option<i64>,
}

impl RecordAgentRunsRequestRunsItemResponseUsageBuilder {
    pub fn cached_input_tokens(mut self, value: i64) -> Self {
        self.cached_input_tokens = Some(value);
        self
    }

    pub fn input_tokens(mut self, value: i64) -> Self {
        self.input_tokens = Some(value);
        self
    }

    pub fn output_tokens(mut self, value: i64) -> Self {
        self.output_tokens = Some(value);
        self
    }

    pub fn reasoning_tokens(mut self, value: i64) -> Self {
        self.reasoning_tokens = Some(value);
        self
    }

    pub fn total_tokens(mut self, value: i64) -> Self {
        self.total_tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecordAgentRunsRequestRunsItemResponseUsage`].
    pub fn build(self) -> Result<RecordAgentRunsRequestRunsItemResponseUsage, BuildError> {
        Ok(RecordAgentRunsRequestRunsItemResponseUsage {
            cached_input_tokens: self.cached_input_tokens,
            input_tokens: self.input_tokens,
            output_tokens: self.output_tokens,
            reasoning_tokens: self.reasoning_tokens,
            total_tokens: self.total_tokens,
        })
    }
}
