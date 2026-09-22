pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateEnvironmentRequest {
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateEnvironmentRequest {
    pub fn builder() -> CreateEnvironmentRequestBuilder {
        <CreateEnvironmentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateEnvironmentRequestBuilder {
    slug: Option<String>,
    label: Option<String>,
    description: Option<String>,
}

impl CreateEnvironmentRequestBuilder {
    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateEnvironmentRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`slug`](CreateEnvironmentRequestBuilder::slug)
    /// - [`label`](CreateEnvironmentRequestBuilder::label)
    pub fn build(self) -> Result<CreateEnvironmentRequest, BuildError> {
        Ok(CreateEnvironmentRequest {
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            description: self.description,
        })
    }
}
