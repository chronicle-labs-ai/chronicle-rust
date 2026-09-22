pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DatasetSavedViewPatchStateSortingItem {
    #[serde(default)]
    pub desc: bool,
    #[serde(default)]
    pub id: String,
}

impl DatasetSavedViewPatchStateSortingItem {
    pub fn builder() -> DatasetSavedViewPatchStateSortingItemBuilder {
        <DatasetSavedViewPatchStateSortingItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetSavedViewPatchStateSortingItemBuilder {
    desc: Option<bool>,
    id: Option<String>,
}

impl DatasetSavedViewPatchStateSortingItemBuilder {
    pub fn desc(mut self, value: bool) -> Self {
        self.desc = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DatasetSavedViewPatchStateSortingItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`desc`](DatasetSavedViewPatchStateSortingItemBuilder::desc)
    /// - [`id`](DatasetSavedViewPatchStateSortingItemBuilder::id)
    pub fn build(self) -> Result<DatasetSavedViewPatchStateSortingItem, BuildError> {
        Ok(DatasetSavedViewPatchStateSortingItem {
            desc: self.desc.ok_or_else(|| BuildError::missing_field("desc"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
