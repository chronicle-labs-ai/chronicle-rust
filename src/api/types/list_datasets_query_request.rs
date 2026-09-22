pub use crate::prelude::*;

/// Query parameters for listDatasets
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListDatasetsQueryRequest {
    #[serde(rename = "includeArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque position returned as `next_cursor` by the preceding page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ListDatasetsQueryRequest {
    pub fn builder() -> ListDatasetsQueryRequestBuilder {
        <ListDatasetsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDatasetsQueryRequestBuilder {
    include_archived: Option<bool>,
    query: Option<String>,
    limit: Option<i64>,
    cursor: Option<String>,
}

impl ListDatasetsQueryRequestBuilder {
    pub fn include_archived(mut self, value: bool) -> Self {
        self.include_archived = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
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

    /// Consumes the builder and constructs a [`ListDatasetsQueryRequest`].
    pub fn build(self) -> Result<ListDatasetsQueryRequest, BuildError> {
        Ok(ListDatasetsQueryRequest {
            include_archived: self.include_archived,
            query: self.query,
            limit: self.limit,
            cursor: self.cursor,
        })
    }
}
