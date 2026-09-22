pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecordAgentRunsRequestRunsItemPreparedCall {
    #[serde(rename = "activeTools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_tools: Option<Vec<String>>,
    #[serde(default)]
    pub hash: String,
    #[serde(rename = "providerOptionsHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options_hash: Option<String>,
}

impl RecordAgentRunsRequestRunsItemPreparedCall {
    pub fn builder() -> RecordAgentRunsRequestRunsItemPreparedCallBuilder {
        <RecordAgentRunsRequestRunsItemPreparedCallBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecordAgentRunsRequestRunsItemPreparedCallBuilder {
    active_tools: Option<Vec<String>>,
    hash: Option<String>,
    provider_options_hash: Option<String>,
}

impl RecordAgentRunsRequestRunsItemPreparedCallBuilder {
    pub fn active_tools(mut self, value: Vec<String>) -> Self {
        self.active_tools = Some(value);
        self
    }

    pub fn hash(mut self, value: impl Into<String>) -> Self {
        self.hash = Some(value.into());
        self
    }

    pub fn provider_options_hash(mut self, value: impl Into<String>) -> Self {
        self.provider_options_hash = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RecordAgentRunsRequestRunsItemPreparedCall`].
    /// This method will fail if any of the following fields are not set:
    /// - [`hash`](RecordAgentRunsRequestRunsItemPreparedCallBuilder::hash)
    pub fn build(self) -> Result<RecordAgentRunsRequestRunsItemPreparedCall, BuildError> {
        Ok(RecordAgentRunsRequestRunsItemPreparedCall {
            active_tools: self.active_tools,
            hash: self.hash.ok_or_else(|| BuildError::missing_field("hash"))?,
            provider_options_hash: self.provider_options_hash,
        })
    }
}
