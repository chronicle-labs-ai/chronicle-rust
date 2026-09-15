pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateLinkResponse {
    #[serde(default)]
    pub link_id: String,
}

impl CreateLinkResponse {
    pub fn builder() -> CreateLinkResponseBuilder {
        <CreateLinkResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateLinkResponseBuilder {
    link_id: Option<String>,
}

impl CreateLinkResponseBuilder {
    pub fn link_id(mut self, value: impl Into<String>) -> Self {
        self.link_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateLinkResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`link_id`](CreateLinkResponseBuilder::link_id)
    pub fn build(self) -> Result<CreateLinkResponse, BuildError> {
        Ok(CreateLinkResponse {
            link_id: self
                .link_id
                .ok_or_else(|| BuildError::missing_field("link_id"))?,
        })
    }
}
