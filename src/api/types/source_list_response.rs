pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SourceListResponse {
    #[serde(default)]
    pub data: Vec<SourceInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl SourceListResponse {
    pub fn builder() -> SourceListResponseBuilder {
        <SourceListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourceListResponseBuilder {
    data: Option<Vec<SourceInfo>>,
    next_cursor: Option<String>,
}

impl SourceListResponseBuilder {
    pub fn data(mut self, value: Vec<SourceInfo>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SourceListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](SourceListResponseBuilder::data)
    pub fn build(self) -> Result<SourceListResponse, BuildError> {
        Ok(SourceListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            next_cursor: self.next_cursor,
        })
    }
}
