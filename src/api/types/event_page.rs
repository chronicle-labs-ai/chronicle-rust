pub use crate::prelude::*;

/// One page of events, newest first. `next_cursor` is present only when `has_more` is true.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EventPage {
    #[serde(default)]
    pub data: Vec<EventResult>,
    /// Whether a further page exists
    #[serde(default)]
    pub has_more: bool,
    /// Opaque cursor to pass as `cursor` on the next request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl EventPage {
    pub fn builder() -> EventPageBuilder {
        <EventPageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EventPageBuilder {
    data: Option<Vec<EventResult>>,
    has_more: Option<bool>,
    next_cursor: Option<String>,
}

impl EventPageBuilder {
    pub fn data(mut self, value: Vec<EventResult>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EventPage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](EventPageBuilder::data)
    /// - [`has_more`](EventPageBuilder::has_more)
    pub fn build(self) -> Result<EventPage, BuildError> {
        Ok(EventPage {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            has_more: self
                .has_more
                .ok_or_else(|| BuildError::missing_field("has_more"))?,
            next_cursor: self.next_cursor,
        })
    }
}
