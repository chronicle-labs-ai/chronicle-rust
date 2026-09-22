pub use crate::prelude::*;

/// Query parameters for listDatasetTraces
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListDatasetTracesQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque position returned as `next_cursor` by the preceding page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ListDatasetTracesQueryRequest {
    pub fn builder() -> ListDatasetTracesQueryRequestBuilder {
        <ListDatasetTracesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDatasetTracesQueryRequestBuilder {
    limit: Option<i64>,
    cursor: Option<String>,
}

impl ListDatasetTracesQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListDatasetTracesQueryRequest`].
    pub fn build(self) -> Result<ListDatasetTracesQueryRequest, BuildError> {
        Ok(ListDatasetTracesQueryRequest {
            limit: self.limit,
            cursor: self.cursor,
        })
    }
}
