pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskSuiteSnapshotClustersItem {
    /// CSS color for the cluster (e.g. `"var(--c-event-teal)"`).
    #[serde(default)]
    pub color: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub label: String,
    /// Optional pre-computed centroid hint in normalized [0..1] space.
    #[serde(rename = "similarityCenter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity_center: Option<Vec<f64>>,
    #[serde(rename = "traceIds")]
    #[serde(default)]
    pub trace_ids: Vec<String>,
}

impl TaskSuiteSnapshotClustersItem {
    pub fn builder() -> TaskSuiteSnapshotClustersItemBuilder {
        <TaskSuiteSnapshotClustersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskSuiteSnapshotClustersItemBuilder {
    color: Option<String>,
    description: Option<String>,
    id: Option<String>,
    label: Option<String>,
    similarity_center: Option<Vec<f64>>,
    trace_ids: Option<Vec<String>>,
}

impl TaskSuiteSnapshotClustersItemBuilder {
    pub fn color(mut self, value: impl Into<String>) -> Self {
        self.color = Some(value.into());
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

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn similarity_center(mut self, value: Vec<f64>) -> Self {
        self.similarity_center = Some(value);
        self
    }

    pub fn trace_ids(mut self, value: Vec<String>) -> Self {
        self.trace_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskSuiteSnapshotClustersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`color`](TaskSuiteSnapshotClustersItemBuilder::color)
    /// - [`id`](TaskSuiteSnapshotClustersItemBuilder::id)
    /// - [`label`](TaskSuiteSnapshotClustersItemBuilder::label)
    /// - [`trace_ids`](TaskSuiteSnapshotClustersItemBuilder::trace_ids)
    pub fn build(self) -> Result<TaskSuiteSnapshotClustersItem, BuildError> {
        Ok(TaskSuiteSnapshotClustersItem {
            color: self
                .color
                .ok_or_else(|| BuildError::missing_field("color"))?,
            description: self.description,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            similarity_center: self.similarity_center,
            trace_ids: self
                .trace_ids
                .ok_or_else(|| BuildError::missing_field("trace_ids"))?,
        })
    }
}
