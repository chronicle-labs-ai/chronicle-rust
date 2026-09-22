pub use crate::prelude::*;

/// Query parameters for archiveDataset
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ArchiveDatasetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cascade: Option<bool>,
}

impl ArchiveDatasetQueryRequest {
    pub fn builder() -> ArchiveDatasetQueryRequestBuilder {
        <ArchiveDatasetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ArchiveDatasetQueryRequestBuilder {
    cascade: Option<bool>,
}

impl ArchiveDatasetQueryRequestBuilder {
    pub fn cascade(mut self, value: bool) -> Self {
        self.cascade = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ArchiveDatasetQueryRequest`].
    pub fn build(self) -> Result<ArchiveDatasetQueryRequest, BuildError> {
        Ok(ArchiveDatasetQueryRequest {
            cascade: self.cascade,
        })
    }
}
