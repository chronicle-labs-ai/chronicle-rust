pub use crate::prelude::*;

/// Runtime knobs a task carries, mirroring the `[agent]`, `[verifier]` and `[environment]` tables of a Harbor `task.toml`. Every field is optional; the server policy fills defaults at launch.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskTaskConfig {
    #[serde(rename = "agentTimeoutSec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_timeout_sec: Option<f64>,
    /// Network policy for the agent sandbox, mirroring Harbor's `[environment].network_mode`.
    #[serde(rename = "networkMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<TaskTaskConfigNetworkMode>,
    #[serde(rename = "verifierTimeoutSec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifier_timeout_sec: Option<f64>,
}

impl TaskTaskConfig {
    pub fn builder() -> TaskTaskConfigBuilder {
        <TaskTaskConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskTaskConfigBuilder {
    agent_timeout_sec: Option<f64>,
    network_mode: Option<TaskTaskConfigNetworkMode>,
    verifier_timeout_sec: Option<f64>,
}

impl TaskTaskConfigBuilder {
    pub fn agent_timeout_sec(mut self, value: f64) -> Self {
        self.agent_timeout_sec = Some(value);
        self
    }

    pub fn network_mode(mut self, value: TaskTaskConfigNetworkMode) -> Self {
        self.network_mode = Some(value);
        self
    }

    pub fn verifier_timeout_sec(mut self, value: f64) -> Self {
        self.verifier_timeout_sec = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskTaskConfig`].
    pub fn build(self) -> Result<TaskTaskConfig, BuildError> {
        Ok(TaskTaskConfig {
            agent_timeout_sec: self.agent_timeout_sec,
            network_mode: self.network_mode,
            verifier_timeout_sec: self.verifier_timeout_sec,
        })
    }
}
