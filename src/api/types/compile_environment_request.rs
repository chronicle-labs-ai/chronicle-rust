pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompileEnvironmentRequest {
    #[serde(rename = "datasetSnapshotId")]
    #[serde(default)]
    pub dataset_snapshot_id: String,
    #[serde(rename = "scenarioId")]
    #[serde(default)]
    pub scenario_id: String,
}

impl CompileEnvironmentRequest {
    pub fn builder() -> CompileEnvironmentRequestBuilder {
        <CompileEnvironmentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompileEnvironmentRequestBuilder {
    dataset_snapshot_id: Option<String>,
    scenario_id: Option<String>,
}

impl CompileEnvironmentRequestBuilder {
    pub fn dataset_snapshot_id(mut self, value: impl Into<String>) -> Self {
        self.dataset_snapshot_id = Some(value.into());
        self
    }

    pub fn scenario_id(mut self, value: impl Into<String>) -> Self {
        self.scenario_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompileEnvironmentRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dataset_snapshot_id`](CompileEnvironmentRequestBuilder::dataset_snapshot_id)
    /// - [`scenario_id`](CompileEnvironmentRequestBuilder::scenario_id)
    pub fn build(self) -> Result<CompileEnvironmentRequest, BuildError> {
        Ok(CompileEnvironmentRequest {
            dataset_snapshot_id: self
                .dataset_snapshot_id
                .ok_or_else(|| BuildError::missing_field("dataset_snapshot_id"))?,
            scenario_id: self
                .scenario_id
                .ok_or_else(|| BuildError::missing_field("scenario_id"))?,
        })
    }
}
