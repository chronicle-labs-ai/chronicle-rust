pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DatasetSavedViewStateFiltersItem {
    #[serde(rename = "columnId")]
    #[serde(default)]
    pub column_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub operator: String,
    pub value: serde_json::Value,
}

impl DatasetSavedViewStateFiltersItem {
    pub fn builder() -> DatasetSavedViewStateFiltersItemBuilder {
        <DatasetSavedViewStateFiltersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetSavedViewStateFiltersItemBuilder {
    column_id: Option<String>,
    id: Option<String>,
    operator: Option<String>,
    value: Option<serde_json::Value>,
}

impl DatasetSavedViewStateFiltersItemBuilder {
    pub fn column_id(mut self, value: impl Into<String>) -> Self {
        self.column_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn operator(mut self, value: impl Into<String>) -> Self {
        self.operator = Some(value.into());
        self
    }

    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatasetSavedViewStateFiltersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`column_id`](DatasetSavedViewStateFiltersItemBuilder::column_id)
    /// - [`operator`](DatasetSavedViewStateFiltersItemBuilder::operator)
    /// - [`value`](DatasetSavedViewStateFiltersItemBuilder::value)
    pub fn build(self) -> Result<DatasetSavedViewStateFiltersItem, BuildError> {
        Ok(DatasetSavedViewStateFiltersItem {
            column_id: self
                .column_id
                .ok_or_else(|| BuildError::missing_field("column_id"))?,
            id: self.id,
            operator: self
                .operator
                .ok_or_else(|| BuildError::missing_field("operator"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
