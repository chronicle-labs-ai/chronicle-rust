pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSdkKeyRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub scopes: Vec<CreateSdkKeyRequestScopesItem>,
}

impl CreateSdkKeyRequest {
    pub fn builder() -> CreateSdkKeyRequestBuilder {
        <CreateSdkKeyRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSdkKeyRequestBuilder {
    name: Option<String>,
    scopes: Option<Vec<CreateSdkKeyRequestScopesItem>>,
}

impl CreateSdkKeyRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn scopes(mut self, value: Vec<CreateSdkKeyRequestScopesItem>) -> Self {
        self.scopes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSdkKeyRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateSdkKeyRequestBuilder::name)
    /// - [`scopes`](CreateSdkKeyRequestBuilder::scopes)
    pub fn build(self) -> Result<CreateSdkKeyRequest, BuildError> {
        Ok(CreateSdkKeyRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            scopes: self
                .scopes
                .ok_or_else(|| BuildError::missing_field("scopes"))?,
        })
    }
}
