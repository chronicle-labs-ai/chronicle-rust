pub use crate::prelude::*;

/// A single entity and its event volume.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EntityInfo {
    #[serde(default)]
    pub entity_type: String,
    #[serde(default)]
    pub entity_id: String,
    #[serde(default)]
    pub event_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<DateTime<FixedOffset>>,
}

impl EntityInfo {
    pub fn builder() -> EntityInfoBuilder {
        <EntityInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntityInfoBuilder {
    entity_type: Option<String>,
    entity_id: Option<String>,
    event_count: Option<i64>,
    first_seen: Option<DateTime<FixedOffset>>,
    last_seen: Option<DateTime<FixedOffset>>,
}

impl EntityInfoBuilder {
    pub fn entity_type(mut self, value: impl Into<String>) -> Self {
        self.entity_type = Some(value.into());
        self
    }

    pub fn entity_id(mut self, value: impl Into<String>) -> Self {
        self.entity_id = Some(value.into());
        self
    }

    pub fn event_count(mut self, value: i64) -> Self {
        self.event_count = Some(value);
        self
    }

    pub fn first_seen(mut self, value: DateTime<FixedOffset>) -> Self {
        self.first_seen = Some(value);
        self
    }

    pub fn last_seen(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_seen = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EntityInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entity_type`](EntityInfoBuilder::entity_type)
    /// - [`entity_id`](EntityInfoBuilder::entity_id)
    /// - [`event_count`](EntityInfoBuilder::event_count)
    pub fn build(self) -> Result<EntityInfo, BuildError> {
        Ok(EntityInfo {
            entity_type: self
                .entity_type
                .ok_or_else(|| BuildError::missing_field("entity_type"))?,
            entity_id: self
                .entity_id
                .ok_or_else(|| BuildError::missing_field("entity_id"))?,
            event_count: self
                .event_count
                .ok_or_else(|| BuildError::missing_field("event_count"))?,
            first_seen: self.first_seen,
            last_seen: self.last_seen,
        })
    }
}
