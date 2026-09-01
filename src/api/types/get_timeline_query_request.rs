pub use crate::prelude::*;

/// Query parameters for getTimeline
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTimelineQueryRequest {
    /// Page size. Values above the maximum are reduced to it, not rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque cursor from a previous response's next_cursor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Relative time window, for example last_7d.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_linked: Option<bool>,
}

impl GetTimelineQueryRequest {
    pub fn builder() -> GetTimelineQueryRequestBuilder {
        <GetTimelineQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTimelineQueryRequestBuilder {
    limit: Option<i64>,
    cursor: Option<String>,
    since: Option<String>,
    include_linked: Option<bool>,
}

impl GetTimelineQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn since(mut self, value: impl Into<String>) -> Self {
        self.since = Some(value.into());
        self
    }

    pub fn include_linked(mut self, value: bool) -> Self {
        self.include_linked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTimelineQueryRequest`].
    pub fn build(self) -> Result<GetTimelineQueryRequest, BuildError> {
        Ok(GetTimelineQueryRequest {
            limit: self.limit,
            cursor: self.cursor,
            since: self.since,
            include_linked: self.include_linked,
        })
    }
}
