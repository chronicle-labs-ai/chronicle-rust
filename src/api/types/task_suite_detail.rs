pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskSuiteDetail {
    #[serde(default)]
    pub clusters: Vec<TaskSuiteDetailClustersItem>,
    #[serde(default)]
    pub dataset: TaskSuiteDetailDataset,
    #[serde(default)]
    pub edges: Vec<TaskSuiteDetailEdgesItem>,
}

impl TaskSuiteDetail {
    pub fn builder() -> TaskSuiteDetailBuilder {
        <TaskSuiteDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskSuiteDetailBuilder {
    clusters: Option<Vec<TaskSuiteDetailClustersItem>>,
    dataset: Option<TaskSuiteDetailDataset>,
    edges: Option<Vec<TaskSuiteDetailEdgesItem>>,
}

impl TaskSuiteDetailBuilder {
    pub fn clusters(mut self, value: Vec<TaskSuiteDetailClustersItem>) -> Self {
        self.clusters = Some(value);
        self
    }

    pub fn dataset(mut self, value: TaskSuiteDetailDataset) -> Self {
        self.dataset = Some(value);
        self
    }

    pub fn edges(mut self, value: Vec<TaskSuiteDetailEdgesItem>) -> Self {
        self.edges = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskSuiteDetail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`clusters`](TaskSuiteDetailBuilder::clusters)
    /// - [`dataset`](TaskSuiteDetailBuilder::dataset)
    /// - [`edges`](TaskSuiteDetailBuilder::edges)
    pub fn build(self) -> Result<TaskSuiteDetail, BuildError> {
        Ok(TaskSuiteDetail {
            clusters: self
                .clusters
                .ok_or_else(|| BuildError::missing_field("clusters"))?,
            dataset: self
                .dataset
                .ok_or_else(|| BuildError::missing_field("dataset"))?,
            edges: self
                .edges
                .ok_or_else(|| BuildError::missing_field("edges"))?,
        })
    }
}
