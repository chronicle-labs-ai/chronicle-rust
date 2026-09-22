pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SdkKeyListResponse {
    #[serde(default)]
    pub keys: Vec<SdkKey>,
}

impl SdkKeyListResponse {
    pub fn builder() -> SdkKeyListResponseBuilder {
        <SdkKeyListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SdkKeyListResponseBuilder {
    keys: Option<Vec<SdkKey>>,
}

impl SdkKeyListResponseBuilder {
    pub fn keys(mut self, value: Vec<SdkKey>) -> Self {
        self.keys = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SdkKeyListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`keys`](SdkKeyListResponseBuilder::keys)
    pub fn build(self) -> Result<SdkKeyListResponse, BuildError> {
        Ok(SdkKeyListResponse {
            keys: self.keys.ok_or_else(|| BuildError::missing_field("keys"))?,
        })
    }
}
