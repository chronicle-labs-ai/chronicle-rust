pub use crate::prelude::*;

/// Query parameters for queryEvents
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryEventsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// Page size. Values above 200 are clamped to 200.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque position returned as `next_cursor` by the preceding page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Relative time window, for example last_7d.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
}

impl QueryEventsQueryRequest {
    pub fn builder() -> QueryEventsQueryRequestBuilder {
        <QueryEventsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryEventsQueryRequestBuilder {
    source: Option<String>,
    topic: Option<String>,
    event_type: Option<String>,
    entity_type: Option<String>,
    entity_id: Option<String>,
    limit: Option<i64>,
    cursor: Option<String>,
    since: Option<String>,
}

impl QueryEventsQueryRequestBuilder {
    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn topic(mut self, value: impl Into<String>) -> Self {
        self.topic = Some(value.into());
        self
    }

    pub fn event_type(mut self, value: impl Into<String>) -> Self {
        self.event_type = Some(value.into());
        self
    }

    pub fn entity_type(mut self, value: impl Into<String>) -> Self {
        self.entity_type = Some(value.into());
        self
    }

    pub fn entity_id(mut self, value: impl Into<String>) -> Self {
        self.entity_id = Some(value.into());
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

    pub fn since(mut self, value: impl Into<String>) -> Self {
        self.since = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryEventsQueryRequest`].
    pub fn build(self) -> Result<QueryEventsQueryRequest, BuildError> {
        Ok(QueryEventsQueryRequest {
            source: self.source,
            topic: self.topic,
            event_type: self.event_type,
            entity_type: self.entity_type,
            entity_id: self.entity_id,
            limit: self.limit,
            cursor: self.cursor,
            since: self.since,
        })
    }
}
