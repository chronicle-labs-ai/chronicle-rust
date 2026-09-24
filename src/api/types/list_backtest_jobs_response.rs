pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListBacktestJobsResponse {
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    #[serde(rename = "nextCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "nextOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<i64>,
    #[serde(default)]
    pub runs: Vec<ListBacktestJobsResponseRunsItem>,
}

impl ListBacktestJobsResponse {
    pub fn builder() -> ListBacktestJobsResponseBuilder {
        <ListBacktestJobsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBacktestJobsResponseBuilder {
    has_more: Option<bool>,
    next_cursor: Option<String>,
    next_offset: Option<i64>,
    runs: Option<Vec<ListBacktestJobsResponseRunsItem>>,
}

impl ListBacktestJobsResponseBuilder {
    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn next_offset(mut self, value: i64) -> Self {
        self.next_offset = Some(value);
        self
    }

    pub fn runs(mut self, value: Vec<ListBacktestJobsResponseRunsItem>) -> Self {
        self.runs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListBacktestJobsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`has_more`](ListBacktestJobsResponseBuilder::has_more)
    /// - [`runs`](ListBacktestJobsResponseBuilder::runs)
    pub fn build(self) -> Result<ListBacktestJobsResponse, BuildError> {
        Ok(ListBacktestJobsResponse {
            has_more: self
                .has_more
                .ok_or_else(|| BuildError::missing_field("has_more"))?,
            next_cursor: self.next_cursor,
            next_offset: self.next_offset,
            runs: self.runs.ok_or_else(|| BuildError::missing_field("runs"))?,
        })
    }
}
