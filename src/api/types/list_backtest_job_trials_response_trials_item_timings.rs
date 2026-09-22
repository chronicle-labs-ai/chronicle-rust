pub use crate::prelude::*;

/// Per-phase timing bookkeeping. Each pair is `(started_at, finished_at)`; `None` until that phase starts/ends.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListBacktestJobTrialsResponseTrialsItemTimings {
    #[serde(rename = "agentRunFinishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_run_finished_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "agentRunStartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_run_started_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "agentSetupFinishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_setup_finished_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "agentSetupStartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_setup_started_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "envSetupFinishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_setup_finished_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "envSetupStartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_setup_started_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "verifierFinishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifier_finished_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "verifierStartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifier_started_at: Option<DateTime<FixedOffset>>,
}

impl ListBacktestJobTrialsResponseTrialsItemTimings {
    pub fn builder() -> ListBacktestJobTrialsResponseTrialsItemTimingsBuilder {
        <ListBacktestJobTrialsResponseTrialsItemTimingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBacktestJobTrialsResponseTrialsItemTimingsBuilder {
    agent_run_finished_at: Option<DateTime<FixedOffset>>,
    agent_run_started_at: Option<DateTime<FixedOffset>>,
    agent_setup_finished_at: Option<DateTime<FixedOffset>>,
    agent_setup_started_at: Option<DateTime<FixedOffset>>,
    env_setup_finished_at: Option<DateTime<FixedOffset>>,
    env_setup_started_at: Option<DateTime<FixedOffset>>,
    verifier_finished_at: Option<DateTime<FixedOffset>>,
    verifier_started_at: Option<DateTime<FixedOffset>>,
}

impl ListBacktestJobTrialsResponseTrialsItemTimingsBuilder {
    pub fn agent_run_finished_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.agent_run_finished_at = Some(value);
        self
    }

    pub fn agent_run_started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.agent_run_started_at = Some(value);
        self
    }

    pub fn agent_setup_finished_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.agent_setup_finished_at = Some(value);
        self
    }

    pub fn agent_setup_started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.agent_setup_started_at = Some(value);
        self
    }

    pub fn env_setup_finished_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.env_setup_finished_at = Some(value);
        self
    }

    pub fn env_setup_started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.env_setup_started_at = Some(value);
        self
    }

    pub fn verifier_finished_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.verifier_finished_at = Some(value);
        self
    }

    pub fn verifier_started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.verifier_started_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListBacktestJobTrialsResponseTrialsItemTimings`].
    pub fn build(self) -> Result<ListBacktestJobTrialsResponseTrialsItemTimings, BuildError> {
        Ok(ListBacktestJobTrialsResponseTrialsItemTimings {
            agent_run_finished_at: self.agent_run_finished_at,
            agent_run_started_at: self.agent_run_started_at,
            agent_setup_finished_at: self.agent_setup_finished_at,
            agent_setup_started_at: self.agent_setup_started_at,
            env_setup_finished_at: self.env_setup_finished_at,
            env_setup_started_at: self.env_setup_started_at,
            verifier_finished_at: self.verifier_finished_at,
            verifier_started_at: self.verifier_started_at,
        })
    }
}
