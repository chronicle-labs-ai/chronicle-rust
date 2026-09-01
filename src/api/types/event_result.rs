pub use crate::prelude::*;

/// An event with its entity references and optional search distance.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EventResult {
    #[serde(default)]
    pub event: Event,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_refs: Option<Vec<EntityRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_distance: Option<f64>,
}

impl EventResult {
    pub fn builder() -> EventResultBuilder {
        <EventResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EventResultBuilder {
    event: Option<Event>,
    entity_refs: Option<Vec<EntityRef>>,
    search_distance: Option<f64>,
}

impl EventResultBuilder {
    pub fn event(mut self, value: Event) -> Self {
        self.event = Some(value);
        self
    }

    pub fn entity_refs(mut self, value: Vec<EntityRef>) -> Self {
        self.entity_refs = Some(value);
        self
    }

    pub fn search_distance(mut self, value: f64) -> Self {
        self.search_distance = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EventResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event`](EventResultBuilder::event)
    pub fn build(self) -> Result<EventResult, BuildError> {
        Ok(EventResult {
            event: self
                .event
                .ok_or_else(|| BuildError::missing_field("event"))?,
            entity_refs: self.entity_refs,
            search_distance: self.search_distance,
        })
    }
}
