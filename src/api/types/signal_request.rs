pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SignalRequest {
    #[serde(default)]
    pub event_id: String,
    #[serde(default)]
    pub signal_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment: Option<String>,
}

impl SignalRequest {
    pub fn builder() -> SignalRequestBuilder {
        <SignalRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SignalRequestBuilder {
    event_id: Option<String>,
    signal_name: Option<String>,
    timestamp: Option<DateTime<FixedOffset>>,
    properties: Option<serde_json::Value>,
    attachment_id: Option<String>,
    signal_type: Option<String>,
    sentiment: Option<String>,
}

impl SignalRequestBuilder {
    pub fn event_id(mut self, value: impl Into<String>) -> Self {
        self.event_id = Some(value.into());
        self
    }

    pub fn signal_name(mut self, value: impl Into<String>) -> Self {
        self.signal_name = Some(value.into());
        self
    }

    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn properties(mut self, value: serde_json::Value) -> Self {
        self.properties = Some(value);
        self
    }

    pub fn attachment_id(mut self, value: impl Into<String>) -> Self {
        self.attachment_id = Some(value.into());
        self
    }

    pub fn signal_type(mut self, value: impl Into<String>) -> Self {
        self.signal_type = Some(value.into());
        self
    }

    pub fn sentiment(mut self, value: impl Into<String>) -> Self {
        self.sentiment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SignalRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_id`](SignalRequestBuilder::event_id)
    /// - [`signal_name`](SignalRequestBuilder::signal_name)
    pub fn build(self) -> Result<SignalRequest, BuildError> {
        Ok(SignalRequest {
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
            signal_name: self
                .signal_name
                .ok_or_else(|| BuildError::missing_field("signal_name"))?,
            timestamp: self.timestamp,
            properties: self.properties,
            attachment_id: self.attachment_id,
            signal_type: self.signal_type,
            sentiment: self.sentiment,
        })
    }
}
