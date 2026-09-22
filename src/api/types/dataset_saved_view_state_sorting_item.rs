pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DatasetSavedViewStateSortingItem {
    #[serde(default)]
    pub desc: bool,
    #[serde(default)]
    pub id: String,
}

impl DatasetSavedViewStateSortingItem {
    pub fn builder() -> DatasetSavedViewStateSortingItemBuilder {
        <DatasetSavedViewStateSortingItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetSavedViewStateSortingItemBuilder {
    desc: Option<bool>,
    id: Option<String>,
}

impl DatasetSavedViewStateSortingItemBuilder {
    pub fn desc(mut self, value: bool) -> Self {
        self.desc = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DatasetSavedViewStateSortingItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`desc`](DatasetSavedViewStateSortingItemBuilder::desc)
    /// - [`id`](DatasetSavedViewStateSortingItemBuilder::id)
    pub fn build(self) -> Result<DatasetSavedViewStateSortingItem, BuildError> {
        Ok(DatasetSavedViewStateSortingItem {
            desc: self.desc.ok_or_else(|| BuildError::missing_field("desc"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
