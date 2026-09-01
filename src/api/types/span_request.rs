pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpanRequest {
    #[serde(default)]
    pub trace_id: String,
    #[serde(default)]
    pub span_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_span_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
}

impl SpanRequest {
    pub fn builder() -> SpanRequestBuilder {
        <SpanRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpanRequestBuilder {
    trace_id: Option<String>,
    span_id: Option<String>,
    parent_span_id: Option<String>,
    event_id: Option<String>,
    name: Option<String>,
    kind: Option<String>,
    status: Option<String>,
    started_at: Option<DateTime<FixedOffset>>,
    ended_at: Option<DateTime<FixedOffset>>,
    duration_ms: Option<f64>,
    attributes: Option<serde_json::Value>,
    links: Option<serde_json::Value>,
}

impl SpanRequestBuilder {
    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn span_id(mut self, value: impl Into<String>) -> Self {
        self.span_id = Some(value.into());
        self
    }

    pub fn parent_span_id(mut self, value: impl Into<String>) -> Self {
        self.parent_span_id = Some(value.into());
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

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
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

    pub fn duration_ms(mut self, value: f64) -> Self {
        self.duration_ms = Some(value);
        self
    }

    pub fn attributes(mut self, value: serde_json::Value) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn links(mut self, value: serde_json::Value) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpanRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`trace_id`](SpanRequestBuilder::trace_id)
    /// - [`span_id`](SpanRequestBuilder::span_id)
    /// - [`name`](SpanRequestBuilder::name)
    pub fn build(self) -> Result<SpanRequest, BuildError> {
        Ok(SpanRequest {
            trace_id: self
                .trace_id
                .ok_or_else(|| BuildError::missing_field("trace_id"))?,
            span_id: self
                .span_id
                .ok_or_else(|| BuildError::missing_field("span_id"))?,
            parent_span_id: self.parent_span_id,
            event_id: self.event_id,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            kind: self.kind,
            status: self.status,
            started_at: self.started_at,
            ended_at: self.ended_at,
            duration_ms: self.duration_ms,
            attributes: self.attributes,
            links: self.links,
        })
    }
}
