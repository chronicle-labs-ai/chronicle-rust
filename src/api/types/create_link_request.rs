pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateLinkRequest {
    #[serde(default)]
    pub source_event_id: String,
    #[serde(default)]
    pub target_event_id: String,
    #[serde(default)]
    pub link_type: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub confidence: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
}

impl CreateLinkRequest {
    pub fn builder() -> CreateLinkRequestBuilder {
        <CreateLinkRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateLinkRequestBuilder {
    source_event_id: Option<String>,
    target_event_id: Option<String>,
    link_type: Option<String>,
    confidence: Option<f64>,
    reasoning: Option<String>,
    created_by: Option<String>,
}

impl CreateLinkRequestBuilder {
    pub fn source_event_id(mut self, value: impl Into<String>) -> Self {
        self.source_event_id = Some(value.into());
        self
    }

    pub fn target_event_id(mut self, value: impl Into<String>) -> Self {
        self.target_event_id = Some(value.into());
        self
    }

    pub fn link_type(mut self, value: impl Into<String>) -> Self {
        self.link_type = Some(value.into());
        self
    }

    pub fn confidence(mut self, value: f64) -> Self {
        self.confidence = Some(value);
        self
    }

    pub fn reasoning(mut self, value: impl Into<String>) -> Self {
        self.reasoning = Some(value.into());
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateLinkRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source_event_id`](CreateLinkRequestBuilder::source_event_id)
    /// - [`target_event_id`](CreateLinkRequestBuilder::target_event_id)
    /// - [`link_type`](CreateLinkRequestBuilder::link_type)
    /// - [`confidence`](CreateLinkRequestBuilder::confidence)
    pub fn build(self) -> Result<CreateLinkRequest, BuildError> {
        Ok(CreateLinkRequest {
            source_event_id: self
                .source_event_id
                .ok_or_else(|| BuildError::missing_field("source_event_id"))?,
            target_event_id: self
                .target_event_id
                .ok_or_else(|| BuildError::missing_field("target_event_id"))?,
            link_type: self
                .link_type
                .ok_or_else(|| BuildError::missing_field("link_type"))?,
            confidence: self
                .confidence
                .ok_or_else(|| BuildError::missing_field("confidence"))?,
            reasoning: self.reasoning,
            created_by: self.created_by,
        })
    }
}
