pub use crate::prelude::*;

/// A stored event. Immutable once written.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Event {
    /// Time-ordered identifier, sorts in creation order.
    #[serde(default)]
    pub event_id: String,
    /// Organisation the event belongs to.
    #[serde(default)]
    pub org_id: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub topic: String,
    #[serde(default)]
    pub event_type: String,
    /// When the event happened at the source.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub event_time: DateTime<FixedOffset>,
    /// When Chronicle received it.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub ingestion_time: DateTime<FixedOffset>,
    /// Source-defined body. Free-form by design.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<MediaAttachment>,
    /// Entity references supplied with the event at ingest time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_refs: Option<Vec<PendingEntityRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_body: Option<String>,
}

impl Event {
    pub fn builder() -> EventBuilder {
        <EventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EventBuilder {
    event_id: Option<String>,
    org_id: Option<String>,
    source: Option<String>,
    topic: Option<String>,
    event_type: Option<String>,
    event_time: Option<DateTime<FixedOffset>>,
    ingestion_time: Option<DateTime<FixedOffset>>,
    payload: Option<HashMap<String, serde_json::Value>>,
    media: Option<MediaAttachment>,
    entity_refs: Option<Vec<PendingEntityRef>>,
    raw_body: Option<String>,
}

impl EventBuilder {
    pub fn event_id(mut self, value: impl Into<String>) -> Self {
        self.event_id = Some(value.into());
        self
    }

    pub fn org_id(mut self, value: impl Into<String>) -> Self {
        self.org_id = Some(value.into());
        self
    }

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

    pub fn event_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.event_time = Some(value);
        self
    }

    pub fn ingestion_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.ingestion_time = Some(value);
        self
    }

    pub fn payload(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.payload = Some(value);
        self
    }

    pub fn media(mut self, value: MediaAttachment) -> Self {
        self.media = Some(value);
        self
    }

    pub fn entity_refs(mut self, value: Vec<PendingEntityRef>) -> Self {
        self.entity_refs = Some(value);
        self
    }

    pub fn raw_body(mut self, value: impl Into<String>) -> Self {
        self.raw_body = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Event`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_id`](EventBuilder::event_id)
    /// - [`org_id`](EventBuilder::org_id)
    /// - [`source`](EventBuilder::source)
    /// - [`topic`](EventBuilder::topic)
    /// - [`event_type`](EventBuilder::event_type)
    /// - [`event_time`](EventBuilder::event_time)
    /// - [`ingestion_time`](EventBuilder::ingestion_time)
    pub fn build(self) -> Result<Event, BuildError> {
        Ok(Event {
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
            org_id: self
                .org_id
                .ok_or_else(|| BuildError::missing_field("org_id"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            topic: self
                .topic
                .ok_or_else(|| BuildError::missing_field("topic"))?,
            event_type: self
                .event_type
                .ok_or_else(|| BuildError::missing_field("event_type"))?,
            event_time: self
                .event_time
                .ok_or_else(|| BuildError::missing_field("event_time"))?,
            ingestion_time: self
                .ingestion_time
                .ok_or_else(|| BuildError::missing_field("ingestion_time"))?,
            payload: self.payload,
            media: self.media,
            entity_refs: self.entity_refs,
            raw_body: self.raw_body,
        })
    }
}
