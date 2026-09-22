pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[non_exhaustive]
pub enum TrialEvent {
    #[serde(rename = "job-started")]
    #[non_exhaustive]
    JobStarted {
        #[serde(default)]
        job_id: String,
    },

    #[serde(rename = "trial-phase-changed")]
    #[non_exhaustive]
    TrialPhaseChanged {
        #[serde(default)]
        job_id: String,
        phase: TrialEventTrialPhaseChangedPhase,
        #[serde(default)]
        trial_id: String,
    },

    #[serde(rename = "trial-rewards-recorded")]
    #[non_exhaustive]
    TrialRewardsRecorded {
        #[serde(default)]
        job_id: String,
        #[serde(default)]
        rewards: HashMap<String, f64>,
        #[serde(default)]
        trial_id: String,
    },

    #[serde(rename = "trial-finished")]
    #[non_exhaustive]
    TrialFinished {
        #[serde(skip_serializing_if = "Option::is_none")]
        exception: Option<TrialEventTrialFinishedException>,
        #[serde(default)]
        job_id: String,
        status: TrialEventTrialFinishedStatus,
        #[serde(default)]
        trial_id: String,
    },

    #[serde(rename = "job-finished")]
    #[non_exhaustive]
    JobFinished {
        #[serde(default)]
        job_id: String,
        status: TrialEventJobFinishedStatus,
        #[serde(skip_serializing_if = "Option::is_none")]
        verdict: Option<String>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl TrialEvent {
    pub fn job_started(job_id: String) -> Self {
        Self::JobStarted { job_id }
    }

    pub fn trial_phase_changed(
        job_id: String,
        phase: TrialEventTrialPhaseChangedPhase,
        trial_id: String,
    ) -> Self {
        Self::TrialPhaseChanged {
            job_id,
            phase,
            trial_id,
        }
    }

    pub fn trial_rewards_recorded(
        job_id: String,
        rewards: HashMap<String, f64>,
        trial_id: String,
    ) -> Self {
        Self::TrialRewardsRecorded {
            job_id,
            rewards,
            trial_id,
        }
    }

    pub fn trial_finished(
        job_id: String,
        status: TrialEventTrialFinishedStatus,
        trial_id: String,
    ) -> Self {
        Self::TrialFinished {
            exception: None,
            job_id,
            status,
            trial_id,
        }
    }

    pub fn job_finished(job_id: String, status: TrialEventJobFinishedStatus) -> Self {
        Self::JobFinished {
            job_id,
            status,
            verdict: None,
        }
    }

    pub fn trial_finished_with_exception(
        exception: TrialEventTrialFinishedException,
        job_id: String,
        status: TrialEventTrialFinishedStatus,
        trial_id: String,
    ) -> Self {
        Self::TrialFinished {
            exception: Some(exception),
            job_id,
            status,
            trial_id,
        }
    }

    pub fn job_finished_with_verdict(
        job_id: String,
        status: TrialEventJobFinishedStatus,
        verdict: String,
    ) -> Self {
        Self::JobFinished {
            job_id,
            status,
            verdict: Some(verdict),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
