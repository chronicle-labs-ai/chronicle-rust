pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SetTaskVerifiersRequest {
    #[serde(default)]
    pub verifiers: Vec<SetTaskVerifiersRequestVerifiersItem>,
}

impl SetTaskVerifiersRequest {
    pub fn builder() -> SetTaskVerifiersRequestBuilder {
        <SetTaskVerifiersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetTaskVerifiersRequestBuilder {
    verifiers: Option<Vec<SetTaskVerifiersRequestVerifiersItem>>,
}

impl SetTaskVerifiersRequestBuilder {
    pub fn verifiers(mut self, value: Vec<SetTaskVerifiersRequestVerifiersItem>) -> Self {
        self.verifiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetTaskVerifiersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`verifiers`](SetTaskVerifiersRequestBuilder::verifiers)
    pub fn build(self) -> Result<SetTaskVerifiersRequest, BuildError> {
        Ok(SetTaskVerifiersRequest {
            verifiers: self
                .verifiers
                .ok_or_else(|| BuildError::missing_field("verifiers"))?,
        })
    }
}
