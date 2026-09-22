pub use crate::prelude::*;

/// Query parameters for listDatasetTraceEvents
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListDatasetTraceEventsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque position returned as `next_cursor` by the preceding page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ListDatasetTraceEventsQueryRequest {
    pub fn builder() -> ListDatasetTraceEventsQueryRequestBuilder {
        <ListDatasetTraceEventsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDatasetTraceEventsQueryRequestBuilder {
    limit: Option<i64>,
    cursor: Option<String>,
}

impl ListDatasetTraceEventsQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListDatasetTraceEventsQueryRequest`].
    pub fn build(self) -> Result<ListDatasetTraceEventsQueryRequest, BuildError> {
        Ok(ListDatasetTraceEventsQueryRequest {
            limit: self.limit,
            cursor: self.cursor,
        })
    }
}
