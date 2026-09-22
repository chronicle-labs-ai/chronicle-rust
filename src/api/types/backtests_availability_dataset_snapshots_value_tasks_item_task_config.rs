pub use crate::prelude::*;

/// Runtime knobs a task carries, mirroring the `[agent]`, `[verifier]` and `[environment]` tables of a Harbor `task.toml`. Every field is optional; the server policy fills defaults at launch.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BacktestsAvailabilityDatasetSnapshotsValueTasksItemTaskConfig {
    #[serde(rename = "agentTimeoutSec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_timeout_sec: Option<f64>,
    /// Network policy for the agent sandbox, mirroring Harbor's `[environment].network_mode`.
    #[serde(rename = "networkMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode:
        Option<BacktestsAvailabilityDatasetSnapshotsValueTasksItemTaskConfigNetworkMode>,
    #[serde(rename = "verifierTimeoutSec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifier_timeout_sec: Option<f64>,
}

impl BacktestsAvailabilityDatasetSnapshotsValueTasksItemTaskConfig {
    pub fn builder() -> BacktestsAvailabilityDatasetSnapshotsValueTasksItemTaskConfigBuilder {
        <BacktestsAvailabilityDatasetSnapshotsValueTasksItemTaskConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BacktestsAvailabilityDatasetSnapshotsValueTasksItemTaskConfigBuilder {
    agent_timeout_sec: Option<f64>,
    network_mode: Option<BacktestsAvailabilityDatasetSnapshotsValueTasksItemTaskConfigNetworkMode>,
    verifier_timeout_sec: Option<f64>,
}

impl BacktestsAvailabilityDatasetSnapshotsValueTasksItemTaskConfigBuilder {
    pub fn agent_timeout_sec(mut self, value: f64) -> Self {
        self.agent_timeout_sec = Some(value);
        self
    }

    pub fn network_mode(
        mut self,
        value: BacktestsAvailabilityDatasetSnapshotsValueTasksItemTaskConfigNetworkMode,
    ) -> Self {
        self.network_mode = Some(value);
        self
    }

    pub fn verifier_timeout_sec(mut self, value: f64) -> Self {
        self.verifier_timeout_sec = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BacktestsAvailabilityDatasetSnapshotsValueTasksItemTaskConfig`].
    pub fn build(
        self,
    ) -> Result<BacktestsAvailabilityDatasetSnapshotsValueTasksItemTaskConfig, BuildError> {
        Ok(
            BacktestsAvailabilityDatasetSnapshotsValueTasksItemTaskConfig {
                agent_timeout_sec: self.agent_timeout_sec,
                network_mode: self.network_mode,
                verifier_timeout_sec: self.verifier_timeout_sec,
            },
        )
    }
}
