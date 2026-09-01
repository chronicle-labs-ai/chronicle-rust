pub use crate::prelude::*;

/// Query parameters for listEntities
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEntitiesQueryRequest {
    /// Page size. Values above 200 are clamped to 200.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque position returned as `next_cursor` by the preceding page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ListEntitiesQueryRequest {
    pub fn builder() -> ListEntitiesQueryRequestBuilder {
        <ListEntitiesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEntitiesQueryRequestBuilder {
    limit: Option<i64>,
    cursor: Option<String>,
}

impl ListEntitiesQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListEntitiesQueryRequest`].
    pub fn build(self) -> Result<ListEntitiesQueryRequest, BuildError> {
        Ok(ListEntitiesQueryRequest {
            limit: self.limit,
            cursor: self.cursor,
        })
    }
}
