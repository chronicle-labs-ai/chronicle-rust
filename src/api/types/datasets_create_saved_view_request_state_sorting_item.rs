pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSavedViewRequestStateSortingItem {
    #[serde(default)]
    pub desc: bool,
    #[serde(default)]
    pub id: String,
}

impl CreateSavedViewRequestStateSortingItem {
    pub fn builder() -> CreateSavedViewRequestStateSortingItemBuilder {
        <CreateSavedViewRequestStateSortingItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSavedViewRequestStateSortingItemBuilder {
    desc: Option<bool>,
    id: Option<String>,
}

impl CreateSavedViewRequestStateSortingItemBuilder {
    pub fn desc(mut self, value: bool) -> Self {
        self.desc = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateSavedViewRequestStateSortingItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`desc`](CreateSavedViewRequestStateSortingItemBuilder::desc)
    /// - [`id`](CreateSavedViewRequestStateSortingItemBuilder::id)
    pub fn build(self) -> Result<CreateSavedViewRequestStateSortingItem, BuildError> {
        Ok(CreateSavedViewRequestStateSortingItem {
            desc: self.desc.ok_or_else(|| BuildError::missing_field("desc"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
