pub use crate::prelude::*;

/// Query parameters for removeTraceFromDataset
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RemoveTraceFromDatasetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl RemoveTraceFromDatasetQueryRequest {
    pub fn builder() -> RemoveTraceFromDatasetQueryRequestBuilder {
        <RemoveTraceFromDatasetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RemoveTraceFromDatasetQueryRequestBuilder {
    reason: Option<String>,
}

impl RemoveTraceFromDatasetQueryRequestBuilder {
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RemoveTraceFromDatasetQueryRequest`].
    pub fn build(self) -> Result<RemoveTraceFromDatasetQueryRequest, BuildError> {
        Ok(RemoveTraceFromDatasetQueryRequest {
            reason: self.reason,
        })
    }
}
