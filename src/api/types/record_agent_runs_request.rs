pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RecordAgentRunsRequest {
    #[serde(default)]
    pub runs: Vec<RecordAgentRunsRequestRunsItem>,
}

impl RecordAgentRunsRequest {
    pub fn builder() -> RecordAgentRunsRequestBuilder {
        <RecordAgentRunsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecordAgentRunsRequestBuilder {
    runs: Option<Vec<RecordAgentRunsRequestRunsItem>>,
}

impl RecordAgentRunsRequestBuilder {
    pub fn runs(mut self, value: Vec<RecordAgentRunsRequestRunsItem>) -> Self {
        self.runs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecordAgentRunsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`runs`](RecordAgentRunsRequestBuilder::runs)
    pub fn build(self) -> Result<RecordAgentRunsRequest, BuildError> {
        Ok(RecordAgentRunsRequest {
            runs: self.runs.ok_or_else(|| BuildError::missing_field("runs"))?,
        })
    }
}
