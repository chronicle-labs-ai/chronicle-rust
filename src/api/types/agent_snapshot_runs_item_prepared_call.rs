pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentSnapshotRunsItemPreparedCall {
    #[serde(rename = "activeTools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_tools: Option<Vec<String>>,
    #[serde(default)]
    pub hash: String,
    #[serde(rename = "providerOptionsHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options_hash: Option<String>,
}

impl AgentSnapshotRunsItemPreparedCall {
    pub fn builder() -> AgentSnapshotRunsItemPreparedCallBuilder {
        <AgentSnapshotRunsItemPreparedCallBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSnapshotRunsItemPreparedCallBuilder {
    active_tools: Option<Vec<String>>,
    hash: Option<String>,
    provider_options_hash: Option<String>,
}

impl AgentSnapshotRunsItemPreparedCallBuilder {
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

    /// Consumes the builder and constructs a [`AgentSnapshotRunsItemPreparedCall`].
    /// This method will fail if any of the following fields are not set:
    /// - [`hash`](AgentSnapshotRunsItemPreparedCallBuilder::hash)
    pub fn build(self) -> Result<AgentSnapshotRunsItemPreparedCall, BuildError> {
        Ok(AgentSnapshotRunsItemPreparedCall {
            active_tools: self.active_tools,
            hash: self.hash.ok_or_else(|| BuildError::missing_field("hash"))?,
            provider_options_hash: self.provider_options_hash,
        })
    }
}
