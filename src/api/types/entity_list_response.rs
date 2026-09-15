pub use crate::prelude::*;

/// A stable page ordered by event count and entity ID.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EntityListResponse {
    #[serde(default)]
    pub data: Vec<EntityInfo>,
    /// Opaque position for the next request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl EntityListResponse {
    pub fn builder() -> EntityListResponseBuilder {
        <EntityListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntityListResponseBuilder {
    data: Option<Vec<EntityInfo>>,
    next_cursor: Option<String>,
}

impl EntityListResponseBuilder {
    pub fn data(mut self, value: Vec<EntityInfo>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EntityListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](EntityListResponseBuilder::data)
    pub fn build(self) -> Result<EntityListResponse, BuildError> {
        Ok(EntityListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            next_cursor: self.next_cursor,
        })
    }
}
