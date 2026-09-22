pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PublishVersionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "idempotencyKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_version_request_idempotency_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl PublishVersionRequest {
    pub fn builder() -> PublishVersionRequestBuilder {
        <PublishVersionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublishVersionRequestBuilder {
    description: Option<String>,
    publish_version_request_idempotency_key: Option<String>,
    label: Option<String>,
}

impl PublishVersionRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn publish_version_request_idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.publish_version_request_idempotency_key = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PublishVersionRequest`].
    pub fn build(self) -> Result<PublishVersionRequest, BuildError> {
        Ok(PublishVersionRequest {
            description: self.description,
            publish_version_request_idempotency_key: self.publish_version_request_idempotency_key,
            label: self.label,
        })
    }
}
