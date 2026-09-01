pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LinkEntityRequest {
    #[serde(default)]
    pub from_entity_type: String,
    #[serde(default)]
    pub from_entity_id: String,
    #[serde(default)]
    pub to_entity_type: String,
    #[serde(default)]
    pub to_entity_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
}

impl LinkEntityRequest {
    pub fn builder() -> LinkEntityRequestBuilder {
        <LinkEntityRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LinkEntityRequestBuilder {
    from_entity_type: Option<String>,
    from_entity_id: Option<String>,
    to_entity_type: Option<String>,
    to_entity_id: Option<String>,
    created_by: Option<String>,
}

impl LinkEntityRequestBuilder {
    pub fn from_entity_type(mut self, value: impl Into<String>) -> Self {
        self.from_entity_type = Some(value.into());
        self
    }

    pub fn from_entity_id(mut self, value: impl Into<String>) -> Self {
        self.from_entity_id = Some(value.into());
        self
    }

    pub fn to_entity_type(mut self, value: impl Into<String>) -> Self {
        self.to_entity_type = Some(value.into());
        self
    }

    pub fn to_entity_id(mut self, value: impl Into<String>) -> Self {
        self.to_entity_id = Some(value.into());
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LinkEntityRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_entity_type`](LinkEntityRequestBuilder::from_entity_type)
    /// - [`from_entity_id`](LinkEntityRequestBuilder::from_entity_id)
    /// - [`to_entity_type`](LinkEntityRequestBuilder::to_entity_type)
    /// - [`to_entity_id`](LinkEntityRequestBuilder::to_entity_id)
    pub fn build(self) -> Result<LinkEntityRequest, BuildError> {
        Ok(LinkEntityRequest {
            from_entity_type: self
                .from_entity_type
                .ok_or_else(|| BuildError::missing_field("from_entity_type"))?,
            from_entity_id: self
                .from_entity_id
                .ok_or_else(|| BuildError::missing_field("from_entity_id"))?,
            to_entity_type: self
                .to_entity_type
                .ok_or_else(|| BuildError::missing_field("to_entity_type"))?,
            to_entity_id: self
                .to_entity_id
                .ok_or_else(|| BuildError::missing_field("to_entity_id"))?,
            created_by: self.created_by,
        })
    }
}
