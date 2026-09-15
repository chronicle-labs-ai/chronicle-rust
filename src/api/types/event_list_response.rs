pub use crate::prelude::*;

/// A bounded page of events. An absent `next_cursor` means the page is final.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EventListResponse {
    #[serde(default)]
    pub data: Vec<EventResult>,
    /// Opaque position for the next request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl EventListResponse {
    pub fn builder() -> EventListResponseBuilder {
        <EventListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EventListResponseBuilder {
    data: Option<Vec<EventResult>>,
    next_cursor: Option<String>,
}

impl EventListResponseBuilder {
    pub fn data(mut self, value: Vec<EventResult>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EventListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](EventListResponseBuilder::data)
    pub fn build(self) -> Result<EventListResponse, BuildError> {
        Ok(EventListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            next_cursor: self.next_cursor,
        })
    }
}
