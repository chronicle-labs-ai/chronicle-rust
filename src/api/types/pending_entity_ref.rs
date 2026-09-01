pub use crate::prelude::*;

/// An entity reference as supplied at ingest, before it is stored.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PendingEntityRef {
    #[serde(default)]
    pub entity_type: String,
    #[serde(default)]
    pub entity_id: String,
}

impl PendingEntityRef {
    pub fn builder() -> PendingEntityRefBuilder {
        <PendingEntityRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PendingEntityRefBuilder {
    entity_type: Option<String>,
    entity_id: Option<String>,
}

impl PendingEntityRefBuilder {
    pub fn entity_type(mut self, value: impl Into<String>) -> Self {
        self.entity_type = Some(value.into());
        self
    }

    pub fn entity_id(mut self, value: impl Into<String>) -> Self {
        self.entity_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PendingEntityRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entity_type`](PendingEntityRefBuilder::entity_type)
    /// - [`entity_id`](PendingEntityRefBuilder::entity_id)
    pub fn build(self) -> Result<PendingEntityRef, BuildError> {
        Ok(PendingEntityRef {
            entity_type: self
                .entity_type
                .ok_or_else(|| BuildError::missing_field("entity_type"))?,
            entity_id: self
                .entity_id
                .ok_or_else(|| BuildError::missing_field("entity_id"))?,
        })
    }
}
