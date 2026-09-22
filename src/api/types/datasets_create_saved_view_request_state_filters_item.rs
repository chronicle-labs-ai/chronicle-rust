pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateSavedViewRequestStateFiltersItem {
    #[serde(rename = "columnId")]
    #[serde(default)]
    pub column_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub operator: String,
    pub value: serde_json::Value,
}

impl CreateSavedViewRequestStateFiltersItem {
    pub fn builder() -> CreateSavedViewRequestStateFiltersItemBuilder {
        <CreateSavedViewRequestStateFiltersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSavedViewRequestStateFiltersItemBuilder {
    column_id: Option<String>,
    id: Option<String>,
    operator: Option<String>,
    value: Option<serde_json::Value>,
}

impl CreateSavedViewRequestStateFiltersItemBuilder {
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

    /// Consumes the builder and constructs a [`CreateSavedViewRequestStateFiltersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`column_id`](CreateSavedViewRequestStateFiltersItemBuilder::column_id)
    /// - [`operator`](CreateSavedViewRequestStateFiltersItemBuilder::operator)
    /// - [`value`](CreateSavedViewRequestStateFiltersItemBuilder::value)
    pub fn build(self) -> Result<CreateSavedViewRequestStateFiltersItem, BuildError> {
        Ok(CreateSavedViewRequestStateFiltersItem {
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
