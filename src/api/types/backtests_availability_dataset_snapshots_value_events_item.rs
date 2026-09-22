pub use crate::prelude::*;

/// A single event rendered as a mark on the stream timeline. The full shape lives next to dataset shapes because `DatasetSnapshot.events` is the only consumer that ships over the wire today.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BacktestsAvailabilityDatasetSnapshotsValueEventsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    /// Optional explicit color override; falls back to the source color.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Looser, app-defined grouping key (e.g. `conversation_id`).
    #[serde(rename = "correlationKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_key: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// ISO timestamp.
    #[serde(rename = "occurredAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub occurred_at: DateTime<FixedOffset>,
    /// Direct causal predecessor.
    #[serde(rename = "parentEventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_event_id: Option<String>,
    /// Raw payload — shown JSON-pretty in the detail panel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
    /// Source/system the event came from (e.g. `intercom`, `stripe`).
    #[serde(default)]
    pub source: String,
    /// Optional grouping (capture stream id) — currently informational.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<String>,
    /// Trace this event belongs to.
    #[serde(rename = "traceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    /// Human-friendly label for the trace.
    #[serde(rename = "traceLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_label: Option<String>,
    /// Event type within the source (e.g. `conversation.created`).
    #[serde(default)]
    pub r#type: String,
}

impl BacktestsAvailabilityDatasetSnapshotsValueEventsItem {
    pub fn builder() -> BacktestsAvailabilityDatasetSnapshotsValueEventsItemBuilder {
        <BacktestsAvailabilityDatasetSnapshotsValueEventsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BacktestsAvailabilityDatasetSnapshotsValueEventsItemBuilder {
    actor: Option<String>,
    color: Option<String>,
    correlation_key: Option<String>,
    id: Option<String>,
    message: Option<String>,
    occurred_at: Option<DateTime<FixedOffset>>,
    parent_event_id: Option<String>,
    payload: Option<serde_json::Value>,
    source: Option<String>,
    stream: Option<String>,
    trace_id: Option<String>,
    trace_label: Option<String>,
    r#type: Option<String>,
}

impl BacktestsAvailabilityDatasetSnapshotsValueEventsItemBuilder {
    pub fn actor(mut self, value: impl Into<String>) -> Self {
        self.actor = Some(value.into());
        self
    }

    pub fn color(mut self, value: impl Into<String>) -> Self {
        self.color = Some(value.into());
        self
    }

    pub fn correlation_key(mut self, value: impl Into<String>) -> Self {
        self.correlation_key = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn occurred_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.occurred_at = Some(value);
        self
    }

    pub fn parent_event_id(mut self, value: impl Into<String>) -> Self {
        self.parent_event_id = Some(value.into());
        self
    }

    pub fn payload(mut self, value: serde_json::Value) -> Self {
        self.payload = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn stream(mut self, value: impl Into<String>) -> Self {
        self.stream = Some(value.into());
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn trace_label(mut self, value: impl Into<String>) -> Self {
        self.trace_label = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BacktestsAvailabilityDatasetSnapshotsValueEventsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BacktestsAvailabilityDatasetSnapshotsValueEventsItemBuilder::id)
    /// - [`occurred_at`](BacktestsAvailabilityDatasetSnapshotsValueEventsItemBuilder::occurred_at)
    /// - [`source`](BacktestsAvailabilityDatasetSnapshotsValueEventsItemBuilder::source)
    /// - [`r#type`](BacktestsAvailabilityDatasetSnapshotsValueEventsItemBuilder::r#type)
    pub fn build(self) -> Result<BacktestsAvailabilityDatasetSnapshotsValueEventsItem, BuildError> {
        Ok(BacktestsAvailabilityDatasetSnapshotsValueEventsItem {
            actor: self.actor,
            color: self.color,
            correlation_key: self.correlation_key,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            message: self.message,
            occurred_at: self
                .occurred_at
                .ok_or_else(|| BuildError::missing_field("occurred_at"))?,
            parent_event_id: self.parent_event_id,
            payload: self.payload,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            stream: self.stream,
            trace_id: self.trace_id,
            trace_label: self.trace_label,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
