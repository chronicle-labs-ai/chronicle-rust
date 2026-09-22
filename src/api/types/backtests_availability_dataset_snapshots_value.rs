pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BacktestsAvailabilityDatasetSnapshotsValue {
    #[serde(default)]
    pub clusters: Vec<BacktestsAvailabilityDatasetSnapshotsValueClustersItem>,
    #[serde(default)]
    pub dataset: BacktestsAvailabilityDatasetSnapshotsValueDataset,
    #[serde(default)]
    pub edges: Vec<BacktestsAvailabilityDatasetSnapshotsValueEdgesItem>,
    /// Optional pre-built event index used by the Timeline tab.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<BacktestsAvailabilityDatasetSnapshotsValueEventsItem>>,
    /// Task definitions, one per trace. Absent on snapshots built by clients that only render the timeline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<BacktestsAvailabilityDatasetSnapshotsValueTasksItem>>,
    #[serde(default)]
    pub traces: Vec<BacktestsAvailabilityDatasetSnapshotsValueTracesItem>,
}

impl BacktestsAvailabilityDatasetSnapshotsValue {
    pub fn builder() -> BacktestsAvailabilityDatasetSnapshotsValueBuilder {
        <BacktestsAvailabilityDatasetSnapshotsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BacktestsAvailabilityDatasetSnapshotsValueBuilder {
    clusters: Option<Vec<BacktestsAvailabilityDatasetSnapshotsValueClustersItem>>,
    dataset: Option<BacktestsAvailabilityDatasetSnapshotsValueDataset>,
    edges: Option<Vec<BacktestsAvailabilityDatasetSnapshotsValueEdgesItem>>,
    events: Option<Vec<BacktestsAvailabilityDatasetSnapshotsValueEventsItem>>,
    tasks: Option<Vec<BacktestsAvailabilityDatasetSnapshotsValueTasksItem>>,
    traces: Option<Vec<BacktestsAvailabilityDatasetSnapshotsValueTracesItem>>,
}

impl BacktestsAvailabilityDatasetSnapshotsValueBuilder {
    pub fn clusters(
        mut self,
        value: Vec<BacktestsAvailabilityDatasetSnapshotsValueClustersItem>,
    ) -> Self {
        self.clusters = Some(value);
        self
    }

    pub fn dataset(mut self, value: BacktestsAvailabilityDatasetSnapshotsValueDataset) -> Self {
        self.dataset = Some(value);
        self
    }

    pub fn edges(
        mut self,
        value: Vec<BacktestsAvailabilityDatasetSnapshotsValueEdgesItem>,
    ) -> Self {
        self.edges = Some(value);
        self
    }

    pub fn events(
        mut self,
        value: Vec<BacktestsAvailabilityDatasetSnapshotsValueEventsItem>,
    ) -> Self {
        self.events = Some(value);
        self
    }

    pub fn tasks(
        mut self,
        value: Vec<BacktestsAvailabilityDatasetSnapshotsValueTasksItem>,
    ) -> Self {
        self.tasks = Some(value);
        self
    }

    pub fn traces(
        mut self,
        value: Vec<BacktestsAvailabilityDatasetSnapshotsValueTracesItem>,
    ) -> Self {
        self.traces = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BacktestsAvailabilityDatasetSnapshotsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`clusters`](BacktestsAvailabilityDatasetSnapshotsValueBuilder::clusters)
    /// - [`dataset`](BacktestsAvailabilityDatasetSnapshotsValueBuilder::dataset)
    /// - [`edges`](BacktestsAvailabilityDatasetSnapshotsValueBuilder::edges)
    /// - [`traces`](BacktestsAvailabilityDatasetSnapshotsValueBuilder::traces)
    pub fn build(self) -> Result<BacktestsAvailabilityDatasetSnapshotsValue, BuildError> {
        Ok(BacktestsAvailabilityDatasetSnapshotsValue {
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
