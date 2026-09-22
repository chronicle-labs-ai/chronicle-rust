pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentSnapshotRunsItemToolCallsItemError {
    #[serde(default)]
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<String>,
}

impl AgentSnapshotRunsItemToolCallsItemError {
    pub fn builder() -> AgentSnapshotRunsItemToolCallsItemErrorBuilder {
        <AgentSnapshotRunsItemToolCallsItemErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSnapshotRunsItemToolCallsItemErrorBuilder {
    message: Option<String>,
    name: Option<String>,
    stack: Option<String>,
}

impl AgentSnapshotRunsItemToolCallsItemErrorBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn stack(mut self, value: impl Into<String>) -> Self {
        self.stack = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentSnapshotRunsItemToolCallsItemError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](AgentSnapshotRunsItemToolCallsItemErrorBuilder::message)
    pub fn build(self) -> Result<AgentSnapshotRunsItemToolCallsItemError, BuildError> {
        Ok(AgentSnapshotRunsItemToolCallsItemError {
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            name: self.name,
            stack: self.stack,
        })
    }
}
