pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DatasetSavedViewState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub density: Option<String>,
    #[serde(rename = "displayProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_properties: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<DatasetSavedViewStateFiltersItem>>,
    #[serde(rename = "groupBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lens: Option<String>,
    /// Deprecated since the table moved to TanStack multi-column sort. New views write `sorting`; this stays as a back-compat fallback for views captured before the migration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordering: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(rename = "showEmptyGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_empty_groups: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sorting: Option<Vec<DatasetSavedViewStateSortingItem>>,
}

impl DatasetSavedViewState {
    pub fn builder() -> DatasetSavedViewStateBuilder {
        <DatasetSavedViewStateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetSavedViewStateBuilder {
    density: Option<String>,
    display_properties: Option<Vec<String>>,
    filters: Option<Vec<DatasetSavedViewStateFiltersItem>>,
    group_by: Option<String>,
    lens: Option<String>,
    ordering: Option<String>,
    search: Option<String>,
    show_empty_groups: Option<bool>,
    sorting: Option<Vec<DatasetSavedViewStateSortingItem>>,
}

impl DatasetSavedViewStateBuilder {
    pub fn density(mut self, value: impl Into<String>) -> Self {
        self.density = Some(value.into());
        self
    }

    pub fn display_properties(mut self, value: Vec<String>) -> Self {
        self.display_properties = Some(value);
        self
    }

    pub fn filters(mut self, value: Vec<DatasetSavedViewStateFiltersItem>) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn group_by(mut self, value: impl Into<String>) -> Self {
        self.group_by = Some(value.into());
        self
    }

    pub fn lens(mut self, value: impl Into<String>) -> Self {
        self.lens = Some(value.into());
        self
    }

    pub fn ordering(mut self, value: impl Into<String>) -> Self {
        self.ordering = Some(value.into());
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn show_empty_groups(mut self, value: bool) -> Self {
        self.show_empty_groups = Some(value);
        self
    }

    pub fn sorting(mut self, value: Vec<DatasetSavedViewStateSortingItem>) -> Self {
        self.sorting = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatasetSavedViewState`].
    pub fn build(self) -> Result<DatasetSavedViewState, BuildError> {
        Ok(DatasetSavedViewState {
            density: self.density,
            display_properties: self.display_properties,
            filters: self.filters,
            group_by: self.group_by,
            lens: self.lens,
            ordering: self.ordering,
            search: self.search,
            show_empty_groups: self.show_empty_groups,
            sorting: self.sorting,
        })
    }
}
