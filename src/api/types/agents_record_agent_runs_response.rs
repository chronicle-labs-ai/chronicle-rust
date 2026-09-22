pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecordAgentRunsResponse {
    #[serde(default)]
    pub accepted: i64,
}

impl RecordAgentRunsResponse {
    pub fn builder() -> RecordAgentRunsResponseBuilder {
        <RecordAgentRunsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecordAgentRunsResponseBuilder {
    accepted: Option<i64>,
}

impl RecordAgentRunsResponseBuilder {
    pub fn accepted(mut self, value: i64) -> Self {
        self.accepted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecordAgentRunsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`accepted`](RecordAgentRunsResponseBuilder::accepted)
    pub fn build(self) -> Result<RecordAgentRunsResponse, BuildError> {
        Ok(RecordAgentRunsResponse {
            accepted: self
                .accepted
                .ok_or_else(|| BuildError::missing_field("accepted"))?,
        })
    }
}
