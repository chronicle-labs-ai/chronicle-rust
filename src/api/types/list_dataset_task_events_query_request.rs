pub use crate::prelude::*;

/// Query parameters for listDatasetTaskEvents
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListDatasetTaskEventsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque position returned as `next_cursor` by the preceding page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ListDatasetTaskEventsQueryRequest {
    pub fn builder() -> ListDatasetTaskEventsQueryRequestBuilder {
        <ListDatasetTaskEventsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDatasetTaskEventsQueryRequestBuilder {
    limit: Option<i64>,
    cursor: Option<String>,
}

impl ListDatasetTaskEventsQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListDatasetTaskEventsQueryRequest`].
    pub fn build(self) -> Result<ListDatasetTaskEventsQueryRequest, BuildError> {
        Ok(ListDatasetTaskEventsQueryRequest {
            limit: self.limit,
            cursor: self.cursor,
        })
    }
}
