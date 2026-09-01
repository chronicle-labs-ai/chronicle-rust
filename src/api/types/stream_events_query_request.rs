pub use crate::prelude::*;

/// Query parameters for streamEvents
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StreamEventsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
}

impl StreamEventsQueryRequest {
    pub fn builder() -> StreamEventsQueryRequestBuilder {
        <StreamEventsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamEventsQueryRequestBuilder {
    source: Option<String>,
    event_type: Option<String>,
    entity_type: Option<String>,
    entity_id: Option<String>,
}

impl StreamEventsQueryRequestBuilder {
    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn event_type(mut self, value: impl Into<String>) -> Self {
        self.event_type = Some(value.into());
        self
    }

    pub fn entity_type(mut self, value: impl Into<String>) -> Self {
        self.entity_type = Some(value.into());
        self
    }

    pub fn entity_id(mut self, value: impl Into<String>) -> Self {
        self.entity_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StreamEventsQueryRequest`].
    pub fn build(self) -> Result<StreamEventsQueryRequest, BuildError> {
        Ok(StreamEventsQueryRequest {
            source: self.source,
            event_type: self.event_type,
            entity_type: self.entity_type,
            entity_id: self.entity_id,
        })
    }
}
