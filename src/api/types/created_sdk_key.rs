pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatedSdkKey {
    #[serde(flatten)]
    pub sdk_key_fields: SdkKey,
    /// One-time plaintext SDK bearer token.
    #[serde(default)]
    pub bearer: String,
}

impl CreatedSdkKey {
    pub fn builder() -> CreatedSdkKeyBuilder {
        <CreatedSdkKeyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatedSdkKeyBuilder {
    sdk_key_fields: Option<SdkKey>,
    bearer: Option<String>,
}

impl CreatedSdkKeyBuilder {
    pub fn sdk_key_fields(mut self, value: SdkKey) -> Self {
        self.sdk_key_fields = Some(value);
        self
    }

    pub fn bearer(mut self, value: impl Into<String>) -> Self {
        self.bearer = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatedSdkKey`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sdk_key_fields`](CreatedSdkKeyBuilder::sdk_key_fields)
    /// - [`bearer`](CreatedSdkKeyBuilder::bearer)
    pub fn build(self) -> Result<CreatedSdkKey, BuildError> {
        Ok(CreatedSdkKey {
            sdk_key_fields: self
                .sdk_key_fields
                .ok_or_else(|| BuildError::missing_field("sdk_key_fields"))?,
            bearer: self
                .bearer
                .ok_or_else(|| BuildError::missing_field("bearer"))?,
        })
    }
}
