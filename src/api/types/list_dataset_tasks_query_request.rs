pub use crate::prelude::*;

/// Query parameters for listDatasetTasks
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListDatasetTasksQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque position returned as `next_cursor` by the preceding page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ListDatasetTasksQueryRequest {
    pub fn builder() -> ListDatasetTasksQueryRequestBuilder {
        <ListDatasetTasksQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDatasetTasksQueryRequestBuilder {
    limit: Option<i64>,
    cursor: Option<String>,
}

impl ListDatasetTasksQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListDatasetTasksQueryRequest`].
    pub fn build(self) -> Result<ListDatasetTasksQueryRequest, BuildError> {
        Ok(ListDatasetTasksQueryRequest {
            limit: self.limit,
            cursor: self.cursor,
        })
    }
}
