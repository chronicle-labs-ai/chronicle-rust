pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct IngestRequest {
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub topic: String,
    #[serde(default)]
    pub event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<FixedOffset>>,
}

impl IngestRequest {
    pub fn builder() -> IngestRequestBuilder {
        <IngestRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IngestRequestBuilder {
    source: Option<String>,
    topic: Option<String>,
    event_type: Option<String>,
    entities: Option<HashMap<String, String>>,
    payload: Option<serde_json::Value>,
    timestamp: Option<DateTime<FixedOffset>>,
}

impl IngestRequestBuilder {
    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn topic(mut self, value: impl Into<String>) -> Self {
        self.topic = Some(value.into());
        self
    }

    pub fn event_type(mut self, value: impl Into<String>) -> Self {
        self.event_type = Some(value.into());
        self
    }

    pub fn entities(mut self, value: HashMap<String, String>) -> Self {
        self.entities = Some(value);
        self
    }

    pub fn payload(mut self, value: serde_json::Value) -> Self {
        self.payload = Some(value);
        self
    }

    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IngestRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source`](IngestRequestBuilder::source)
    /// - [`topic`](IngestRequestBuilder::topic)
    /// - [`event_type`](IngestRequestBuilder::event_type)
    pub fn build(self) -> Result<IngestRequest, BuildError> {
        Ok(IngestRequest {
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            topic: self
                .topic
                .ok_or_else(|| BuildError::missing_field("topic"))?,
            event_type: self
                .event_type
                .ok_or_else(|| BuildError::missing_field("event_type"))?,
            entities: self.entities,
            payload: self.payload,
            timestamp: self.timestamp,
        })
    }
}
