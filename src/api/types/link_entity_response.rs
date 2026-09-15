pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LinkEntityResponse {
    #[serde(default)]
    pub linked_count: i64,
}

impl LinkEntityResponse {
    pub fn builder() -> LinkEntityResponseBuilder {
        <LinkEntityResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LinkEntityResponseBuilder {
    linked_count: Option<i64>,
}

impl LinkEntityResponseBuilder {
    pub fn linked_count(mut self, value: i64) -> Self {
        self.linked_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LinkEntityResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`linked_count`](LinkEntityResponseBuilder::linked_count)
    pub fn build(self) -> Result<LinkEntityResponse, BuildError> {
        Ok(LinkEntityResponse {
            linked_count: self
                .linked_count
                .ok_or_else(|| BuildError::missing_field("linked_count"))?,
        })
    }
}
