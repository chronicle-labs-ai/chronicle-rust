pub use crate::prelude::*;

/// Query parameters for listBacktestJobs
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListBacktestJobsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque position returned as `next_cursor` by the preceding page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Deprecated compatibility input. Pass the opaque `cursor` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl ListBacktestJobsQueryRequest {
    pub fn builder() -> ListBacktestJobsQueryRequestBuilder {
        <ListBacktestJobsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBacktestJobsQueryRequestBuilder {
    mode: Option<String>,
    status: Option<String>,
    limit: Option<i64>,
    cursor: Option<String>,
    offset: Option<i64>,
}

impl ListBacktestJobsQueryRequestBuilder {
    pub fn mode(mut self, value: impl Into<String>) -> Self {
        self.mode = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListBacktestJobsQueryRequest`].
    pub fn build(self) -> Result<ListBacktestJobsQueryRequest, BuildError> {
        Ok(ListBacktestJobsQueryRequest {
            mode: self.mode,
            status: self.status,
            limit: self.limit,
            cursor: self.cursor,
            offset: self.offset,
        })
    }
}
