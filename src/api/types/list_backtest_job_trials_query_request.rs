pub use crate::prelude::*;

/// Query parameters for listBacktestJobTrials
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListBacktestJobTrialsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque position returned as `next_cursor` by the preceding page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Deprecated compatibility input. Pass the opaque `cursor` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl ListBacktestJobTrialsQueryRequest {
    pub fn builder() -> ListBacktestJobTrialsQueryRequestBuilder {
        <ListBacktestJobTrialsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBacktestJobTrialsQueryRequestBuilder {
    limit: Option<i64>,
    cursor: Option<String>,
    offset: Option<i64>,
}

impl ListBacktestJobTrialsQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`ListBacktestJobTrialsQueryRequest`].
    pub fn build(self) -> Result<ListBacktestJobTrialsQueryRequest, BuildError> {
        Ok(ListBacktestJobTrialsQueryRequest {
            limit: self.limit,
            cursor: self.cursor,
            offset: self.offset,
        })
    }
}
