pub use crate::prelude::*;

/// An entity type in use and how much of it there is.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EntityTypeInfo {
    #[serde(default)]
    pub entity_type: String,
    #[serde(default)]
    pub entity_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<DateTime<FixedOffset>>,
}

impl EntityTypeInfo {
    pub fn builder() -> EntityTypeInfoBuilder {
        <EntityTypeInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntityTypeInfoBuilder {
    entity_type: Option<String>,
    entity_count: Option<i64>,
    first_seen: Option<DateTime<FixedOffset>>,
    last_seen: Option<DateTime<FixedOffset>>,
}

impl EntityTypeInfoBuilder {
    pub fn entity_type(mut self, value: impl Into<String>) -> Self {
        self.entity_type = Some(value.into());
        self
    }

    pub fn entity_count(mut self, value: i64) -> Self {
        self.entity_count = Some(value);
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

    /// Consumes the builder and constructs a [`EntityTypeInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entity_type`](EntityTypeInfoBuilder::entity_type)
    /// - [`entity_count`](EntityTypeInfoBuilder::entity_count)
    pub fn build(self) -> Result<EntityTypeInfo, BuildError> {
        Ok(EntityTypeInfo {
            entity_type: self
                .entity_type
                .ok_or_else(|| BuildError::missing_field("entity_type"))?,
            entity_count: self
                .entity_count
                .ok_or_else(|| BuildError::missing_field("entity_count"))?,
            first_seen: self.first_seen,
            last_seen: self.last_seen,
        })
    }
}
