pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RefreshMembershipRequest {
    #[serde(rename = "idempotencyKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

impl RefreshMembershipRequest {
    pub fn builder() -> RefreshMembershipRequestBuilder {
        <RefreshMembershipRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefreshMembershipRequestBuilder {
    idempotency_key: Option<String>,
}

impl RefreshMembershipRequestBuilder {
    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RefreshMembershipRequest`].
    pub fn build(self) -> Result<RefreshMembershipRequest, BuildError> {
        Ok(RefreshMembershipRequest {
            idempotency_key: self.idempotency_key,
        })
    }
}
