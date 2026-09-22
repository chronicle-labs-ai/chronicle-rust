pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatasetSavedView {
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    pub scope: DatasetSavedViewScope,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortcut: Option<String>,
    #[serde(default)]
    pub state: DatasetSavedViewState,
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl DatasetSavedView {
    pub fn builder() -> DatasetSavedViewBuilder {
        <DatasetSavedViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetSavedViewBuilder {
    created_by: Option<String>,
    description: Option<String>,
    id: Option<String>,
    name: Option<String>,
    scope: Option<DatasetSavedViewScope>,
    shortcut: Option<String>,
    state: Option<DatasetSavedViewState>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl DatasetSavedViewBuilder {
    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn scope(mut self, value: DatasetSavedViewScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn shortcut(mut self, value: impl Into<String>) -> Self {
        self.shortcut = Some(value.into());
        self
    }

    pub fn state(mut self, value: DatasetSavedViewState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatasetSavedView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DatasetSavedViewBuilder::id)
    /// - [`name`](DatasetSavedViewBuilder::name)
    /// - [`scope`](DatasetSavedViewBuilder::scope)
    /// - [`state`](DatasetSavedViewBuilder::state)
    pub fn build(self) -> Result<DatasetSavedView, BuildError> {
        Ok(DatasetSavedView {
            created_by: self.created_by,
            description: self.description,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            scope: self
                .scope
                .ok_or_else(|| BuildError::missing_field("scope"))?,
            shortcut: self.shortcut,
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
            updated_at: self.updated_at,
        })
    }
}
