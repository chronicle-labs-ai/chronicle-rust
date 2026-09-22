pub use crate::prelude::*;

/// Returned by `GET /api/platform/backtests/availability`. Matches the frontend's `BacktestsAvailability`. Datasets, environments, and agents come from their respective domain crates' types — the IDs in these slices are what the recipe references.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BacktestsAvailability {
    #[serde(default)]
    pub agents: Vec<BacktestsAvailabilityAgentsItem>,
    #[serde(rename = "datasetSnapshots")]
    #[serde(default)]
    pub dataset_snapshots: HashMap<String, BacktestsAvailabilityDatasetSnapshotsValue>,
    #[serde(default)]
    pub datasets: Vec<BacktestsAvailabilityDatasetsItem>,
    /// Environment row identities + status. Detailed snapshot lives behind `GET /api/platform/environments/:id`.
    #[serde(default)]
    pub environments: Vec<BacktestsAvailabilityEnvironmentsItem>,
}

impl BacktestsAvailability {
    pub fn builder() -> BacktestsAvailabilityBuilder {
        <BacktestsAvailabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BacktestsAvailabilityBuilder {
    agents: Option<Vec<BacktestsAvailabilityAgentsItem>>,
    dataset_snapshots: Option<HashMap<String, BacktestsAvailabilityDatasetSnapshotsValue>>,
    datasets: Option<Vec<BacktestsAvailabilityDatasetsItem>>,
    environments: Option<Vec<BacktestsAvailabilityEnvironmentsItem>>,
}

impl BacktestsAvailabilityBuilder {
    pub fn agents(mut self, value: Vec<BacktestsAvailabilityAgentsItem>) -> Self {
        self.agents = Some(value);
        self
    }

    pub fn dataset_snapshots(
        mut self,
        value: HashMap<String, BacktestsAvailabilityDatasetSnapshotsValue>,
    ) -> Self {
        self.dataset_snapshots = Some(value);
        self
    }

    pub fn datasets(mut self, value: Vec<BacktestsAvailabilityDatasetsItem>) -> Self {
        self.datasets = Some(value);
        self
    }

    pub fn environments(mut self, value: Vec<BacktestsAvailabilityEnvironmentsItem>) -> Self {
        self.environments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BacktestsAvailability`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agents`](BacktestsAvailabilityBuilder::agents)
    /// - [`dataset_snapshots`](BacktestsAvailabilityBuilder::dataset_snapshots)
    /// - [`datasets`](BacktestsAvailabilityBuilder::datasets)
    /// - [`environments`](BacktestsAvailabilityBuilder::environments)
    pub fn build(self) -> Result<BacktestsAvailability, BuildError> {
        Ok(BacktestsAvailability {
            agents: self
                .agents
                .ok_or_else(|| BuildError::missing_field("agents"))?,
            dataset_snapshots: self
                .dataset_snapshots
                .ok_or_else(|| BuildError::missing_field("dataset_snapshots"))?,
            datasets: self
                .datasets
                .ok_or_else(|| BuildError::missing_field("datasets"))?,
            environments: self
                .environments
                .ok_or_else(|| BuildError::missing_field("environments"))?,
        })
    }
}
