pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskSuiteSnapshot {
    #[serde(default)]
    pub clusters: Vec<TaskSuiteSnapshotClustersItem>,
    #[serde(default)]
    pub dataset: TaskSuiteSnapshotDataset,
    #[serde(default)]
    pub edges: Vec<TaskSuiteSnapshotEdgesItem>,
    /// Optional pre-built event index used by the Timeline tab.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<TaskSuiteSnapshotEventsItem>>,
    /// Task definitions, one per trace. Absent on snapshots built by clients that only render the timeline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<TaskSuiteSnapshotTasksItem>>,
    #[serde(default)]
    pub traces: Vec<TaskSuiteSnapshotTracesItem>,
}

impl TaskSuiteSnapshot {
    pub fn builder() -> TaskSuiteSnapshotBuilder {
        <TaskSuiteSnapshotBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskSuiteSnapshotBuilder {
    clusters: Option<Vec<TaskSuiteSnapshotClustersItem>>,
    dataset: Option<TaskSuiteSnapshotDataset>,
    edges: Option<Vec<TaskSuiteSnapshotEdgesItem>>,
    events: Option<Vec<TaskSuiteSnapshotEventsItem>>,
    tasks: Option<Vec<TaskSuiteSnapshotTasksItem>>,
    traces: Option<Vec<TaskSuiteSnapshotTracesItem>>,
}

impl TaskSuiteSnapshotBuilder {
    pub fn clusters(mut self, value: Vec<TaskSuiteSnapshotClustersItem>) -> Self {
        self.clusters = Some(value);
        self
    }

    pub fn dataset(mut self, value: TaskSuiteSnapshotDataset) -> Self {
        self.dataset = Some(value);
        self
    }

    pub fn edges(mut self, value: Vec<TaskSuiteSnapshotEdgesItem>) -> Self {
        self.edges = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<TaskSuiteSnapshotEventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn tasks(mut self, value: Vec<TaskSuiteSnapshotTasksItem>) -> Self {
        self.tasks = Some(value);
        self
    }

    pub fn traces(mut self, value: Vec<TaskSuiteSnapshotTracesItem>) -> Self {
        self.traces = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskSuiteSnapshot`].
    /// This method will fail if any of the following fields are not set:
    /// - [`clusters`](TaskSuiteSnapshotBuilder::clusters)
    /// - [`dataset`](TaskSuiteSnapshotBuilder::dataset)
    /// - [`edges`](TaskSuiteSnapshotBuilder::edges)
    /// - [`traces`](TaskSuiteSnapshotBuilder::traces)
    pub fn build(self) -> Result<TaskSuiteSnapshot, BuildError> {
        Ok(TaskSuiteSnapshot {
            clusters: self
                .clusters
                .ok_or_else(|| BuildError::missing_field("clusters"))?,
            dataset: self
                .dataset
                .ok_or_else(|| BuildError::missing_field("dataset"))?,
            edges: self
                .edges
                .ok_or_else(|| BuildError::missing_field("edges"))?,
            events: self.events,
            tasks: self.tasks,
            traces: self
                .traces
                .ok_or_else(|| BuildError::missing_field("traces"))?,
        })
    }
}
