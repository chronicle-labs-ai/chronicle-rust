pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EntityTypeListResponse {
    #[serde(default)]
    pub data: Vec<EntityTypeInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl EntityTypeListResponse {
    pub fn builder() -> EntityTypeListResponseBuilder {
        <EntityTypeListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntityTypeListResponseBuilder {
    data: Option<Vec<EntityTypeInfo>>,
    next_cursor: Option<String>,
}

impl EntityTypeListResponseBuilder {
    pub fn data(mut self, value: Vec<EntityTypeInfo>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EntityTypeListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](EntityTypeListResponseBuilder::data)
    pub fn build(self) -> Result<EntityTypeListResponse, BuildError> {
        Ok(EntityTypeListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            next_cursor: self.next_cursor,
        })
    }
}
