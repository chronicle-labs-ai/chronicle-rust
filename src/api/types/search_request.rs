pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchRequest {
    #[serde(default)]
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque position returned by the preceding search page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl SearchRequest {
    pub fn builder() -> SearchRequestBuilder {
        <SearchRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchRequestBuilder {
    query: Option<String>,
    source: Option<String>,
    entity_type: Option<String>,
    entity_id: Option<String>,
    limit: Option<i64>,
    cursor: Option<String>,
}

impl SearchRequestBuilder {
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
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

    /// Consumes the builder and constructs a [`SearchRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query`](SearchRequestBuilder::query)
    pub fn build(self) -> Result<SearchRequest, BuildError> {
        Ok(SearchRequest {
            query: self
                .query
                .ok_or_else(|| BuildError::missing_field("query"))?,
            source: self.source,
            entity_type: self.entity_type,
            entity_id: self.entity_id,
            limit: self.limit,
            cursor: self.cursor,
        })
    }
}
