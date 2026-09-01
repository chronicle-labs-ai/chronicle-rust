pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddEntityRefRequest {
    #[serde(default)]
    pub event_id: String,
    #[serde(default)]
    pub entity_type: String,
    #[serde(default)]
    pub entity_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
}

impl AddEntityRefRequest {
    pub fn builder() -> AddEntityRefRequestBuilder {
        <AddEntityRefRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddEntityRefRequestBuilder {
    event_id: Option<String>,
    entity_type: Option<String>,
    entity_id: Option<String>,
    created_by: Option<String>,
}

impl AddEntityRefRequestBuilder {
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

    /// Consumes the builder and constructs a [`AddEntityRefRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_id`](AddEntityRefRequestBuilder::event_id)
    /// - [`entity_type`](AddEntityRefRequestBuilder::entity_type)
    /// - [`entity_id`](AddEntityRefRequestBuilder::entity_id)
    pub fn build(self) -> Result<AddEntityRefRequest, BuildError> {
        Ok(AddEntityRefRequest {
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
            entity_type: self
                .entity_type
                .ok_or_else(|| BuildError::missing_field("entity_type"))?,
            entity_id: self
                .entity_id
                .ok_or_else(|| BuildError::missing_field("entity_id"))?,
            created_by: self.created_by,
        })
    }
}
