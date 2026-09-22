pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentSnapshotRunsItemError {
    #[serde(default)]
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<String>,
}

impl AgentSnapshotRunsItemError {
    pub fn builder() -> AgentSnapshotRunsItemErrorBuilder {
        <AgentSnapshotRunsItemErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSnapshotRunsItemErrorBuilder {
    message: Option<String>,
    name: Option<String>,
    stack: Option<String>,
}

impl AgentSnapshotRunsItemErrorBuilder {
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

    /// Consumes the builder and constructs a [`AgentSnapshotRunsItemError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](AgentSnapshotRunsItemErrorBuilder::message)
    pub fn build(self) -> Result<AgentSnapshotRunsItemError, BuildError> {
        Ok(AgentSnapshotRunsItemError {
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            name: self.name,
            stack: self.stack,
        })
    }
}
