pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AcceptedResponse {
    #[serde(default)]
    pub accepted: i64,
}

impl AcceptedResponse {
    pub fn builder() -> AcceptedResponseBuilder {
        <AcceptedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AcceptedResponseBuilder {
    accepted: Option<i64>,
}

impl AcceptedResponseBuilder {
    pub fn accepted(mut self, value: i64) -> Self {
        self.accepted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AcceptedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`accepted`](AcceptedResponseBuilder::accepted)
    pub fn build(self) -> Result<AcceptedResponse, BuildError> {
        Ok(AcceptedResponse {
            accepted: self
                .accepted
                .ok_or_else(|| BuildError::missing_field("accepted"))?,
        })
    }
}
