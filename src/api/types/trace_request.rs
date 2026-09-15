pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TraceRequest {
    #[serde(default)]
    pub trace_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spans: Option<Vec<SpanRequest>>,
}

impl TraceRequest {
    pub fn builder() -> TraceRequestBuilder {
        <TraceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TraceRequestBuilder {
    trace_id: Option<String>,
    event_id: Option<String>,
    name: Option<String>,
    started_at: Option<DateTime<FixedOffset>>,
    ended_at: Option<DateTime<FixedOffset>>,
    attributes: Option<serde_json::Value>,
    spans: Option<Vec<SpanRequest>>,
}

impl TraceRequestBuilder {
    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn event_id(mut self, value: impl Into<String>) -> Self {
        self.event_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn ended_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.ended_at = Some(value);
        self
    }

    pub fn attributes(mut self, value: serde_json::Value) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn spans(mut self, value: Vec<SpanRequest>) -> Self {
        self.spans = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TraceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`trace_id`](TraceRequestBuilder::trace_id)
    pub fn build(self) -> Result<TraceRequest, BuildError> {
        Ok(TraceRequest {
            trace_id: self
                .trace_id
                .ok_or_else(|| BuildError::missing_field("trace_id"))?,
            event_id: self.event_id,
            name: self.name,
            started_at: self.started_at,
            ended_at: self.ended_at,
            attributes: self.attributes,
            spans: self.spans,
        })
    }
}
