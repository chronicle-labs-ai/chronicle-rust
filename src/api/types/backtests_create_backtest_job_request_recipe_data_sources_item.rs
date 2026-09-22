pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateBacktestJobRequestRecipeDataSourcesItem {
    #[serde(default)]
    pub count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<CreateBacktestJobRequestRecipeDataSourcesItemFilters>,
    #[serde(default)]
    pub id: String,
    pub kind: CreateBacktestJobRequestRecipeDataSourcesItemKind,
    #[serde(default)]
    pub label: String,
}

impl CreateBacktestJobRequestRecipeDataSourcesItem {
    pub fn builder() -> CreateBacktestJobRequestRecipeDataSourcesItemBuilder {
        <CreateBacktestJobRequestRecipeDataSourcesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBacktestJobRequestRecipeDataSourcesItemBuilder {
    count: Option<i64>,
    filters: Option<CreateBacktestJobRequestRecipeDataSourcesItemFilters>,
    id: Option<String>,
    kind: Option<CreateBacktestJobRequestRecipeDataSourcesItemKind>,
    label: Option<String>,
}

impl CreateBacktestJobRequestRecipeDataSourcesItemBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn filters(mut self, value: CreateBacktestJobRequestRecipeDataSourcesItemFilters) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: CreateBacktestJobRequestRecipeDataSourcesItemKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateBacktestJobRequestRecipeDataSourcesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](CreateBacktestJobRequestRecipeDataSourcesItemBuilder::count)
    /// - [`id`](CreateBacktestJobRequestRecipeDataSourcesItemBuilder::id)
    /// - [`kind`](CreateBacktestJobRequestRecipeDataSourcesItemBuilder::kind)
    /// - [`label`](CreateBacktestJobRequestRecipeDataSourcesItemBuilder::label)
    pub fn build(self) -> Result<CreateBacktestJobRequestRecipeDataSourcesItem, BuildError> {
        Ok(CreateBacktestJobRequestRecipeDataSourcesItem {
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
            filters: self.filters,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
        })
    }
}
