pub use crate::prelude::*;

/// A stored entity reference, attributed and timestamped.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EntityRef {
    #[serde(default)]
    pub event_id: String,
    #[serde(default)]
    pub entity_type: String,
    #[serde(default)]
    pub entity_id: String,
    /// What created the reference, for example ingestion or a linker agent.
    #[serde(default)]
    pub created_by: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl EntityRef {
    pub fn builder() -> EntityRefBuilder {
        <EntityRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntityRefBuilder {
    event_id: Option<String>,
    entity_type: Option<String>,
    entity_id: Option<String>,
    created_by: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl EntityRefBuilder {
    pub fn event_id(mut self, value: impl Into<String>) -> Self {
        self.event_id = Some(value.into());
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

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EntityRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_id`](EntityRefBuilder::event_id)
    /// - [`entity_type`](EntityRefBuilder::entity_type)
    /// - [`entity_id`](EntityRefBuilder::entity_id)
    /// - [`created_by`](EntityRefBuilder::created_by)
    /// - [`created_at`](EntityRefBuilder::created_at)
    pub fn build(self) -> Result<EntityRef, BuildError> {
        Ok(EntityRef {
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
            entity_type: self
                .entity_type
                .ok_or_else(|| BuildError::missing_field("entity_type"))?,
            entity_id: self
                .entity_id
                .ok_or_else(|| BuildError::missing_field("entity_id"))?,
            created_by: self
                .created_by
                .ok_or_else(|| BuildError::missing_field("created_by"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
