pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskSuiteVersion {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "datasetId")]
    #[serde(default)]
    pub dataset_id: String,
    #[serde(rename = "datasetRevision")]
    #[serde(default)]
    pub dataset_revision: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "eventCount")]
    #[serde(default)]
    pub event_count: i64,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "itemCount")]
    #[serde(default)]
    pub item_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default)]
    pub version: i64,
}

impl TaskSuiteVersion {
    pub fn builder() -> TaskSuiteVersionBuilder {
        <TaskSuiteVersionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskSuiteVersionBuilder {
    created_at: Option<String>,
    created_by: Option<String>,
    dataset_id: Option<String>,
    dataset_revision: Option<i64>,
    description: Option<String>,
    event_count: Option<i64>,
    id: Option<String>,
    item_count: Option<i64>,
    label: Option<String>,
    version: Option<i64>,
}

impl TaskSuiteVersionBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn dataset_id(mut self, value: impl Into<String>) -> Self {
        self.dataset_id = Some(value.into());
        self
    }

    pub fn dataset_revision(mut self, value: i64) -> Self {
        self.dataset_revision = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn event_count(mut self, value: i64) -> Self {
        self.event_count = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn item_count(mut self, value: i64) -> Self {
        self.item_count = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskSuiteVersion`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](TaskSuiteVersionBuilder::created_at)
    /// - [`dataset_id`](TaskSuiteVersionBuilder::dataset_id)
    /// - [`dataset_revision`](TaskSuiteVersionBuilder::dataset_revision)
    /// - [`event_count`](TaskSuiteVersionBuilder::event_count)
    /// - [`id`](TaskSuiteVersionBuilder::id)
    /// - [`item_count`](TaskSuiteVersionBuilder::item_count)
    /// - [`version`](TaskSuiteVersionBuilder::version)
    pub fn build(self) -> Result<TaskSuiteVersion, BuildError> {
        Ok(TaskSuiteVersion {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            created_by: self.created_by,
            dataset_id: self
                .dataset_id
                .ok_or_else(|| BuildError::missing_field("dataset_id"))?,
            dataset_revision: self
                .dataset_revision
                .ok_or_else(|| BuildError::missing_field("dataset_revision"))?,
            description: self.description,
            event_count: self
                .event_count
                .ok_or_else(|| BuildError::missing_field("event_count"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            item_count: self
                .item_count
                .ok_or_else(|| BuildError::missing_field("item_count"))?,
            label: self.label,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
