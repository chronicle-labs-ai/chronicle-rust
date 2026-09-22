pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DatasetSavedViewPatch {
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<DatasetSavedViewPatchScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortcut: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<DatasetSavedViewPatchState>,
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl DatasetSavedViewPatch {
    pub fn builder() -> DatasetSavedViewPatchBuilder {
        <DatasetSavedViewPatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetSavedViewPatchBuilder {
    created_by: Option<String>,
    description: Option<String>,
    name: Option<String>,
    scope: Option<DatasetSavedViewPatchScope>,
    shortcut: Option<String>,
    state: Option<DatasetSavedViewPatchState>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl DatasetSavedViewPatchBuilder {
    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn scope(mut self, value: DatasetSavedViewPatchScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn shortcut(mut self, value: impl Into<String>) -> Self {
        self.shortcut = Some(value.into());
        self
    }

    pub fn state(mut self, value: DatasetSavedViewPatchState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatasetSavedViewPatch`].
    pub fn build(self) -> Result<DatasetSavedViewPatch, BuildError> {
        Ok(DatasetSavedViewPatch {
            created_by: self.created_by,
            description: self.description,
            name: self.name,
            scope: self.scope,
            shortcut: self.shortcut,
            state: self.state,
            updated_at: self.updated_at,
        })
    }
}
