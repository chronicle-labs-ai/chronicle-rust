pub use crate::prelude::*;

/// Query parameters for deleteDatasetTask
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteDatasetTaskQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl DeleteDatasetTaskQueryRequest {
    pub fn builder() -> DeleteDatasetTaskQueryRequestBuilder {
        <DeleteDatasetTaskQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteDatasetTaskQueryRequestBuilder {
    reason: Option<String>,
}

impl DeleteDatasetTaskQueryRequestBuilder {
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteDatasetTaskQueryRequest`].
    pub fn build(self) -> Result<DeleteDatasetTaskQueryRequest, BuildError> {
        Ok(DeleteDatasetTaskQueryRequest {
            reason: self.reason,
        })
    }
}
