pub use crate::prelude::*;

/// Returned by `GET /api/platform/backtests/jobs/:id/trials/:trialId`. Everything the trial detail view renders in one round trip.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BacktestTrialDetailResponse {
    #[serde(default)]
    pub artifacts: Vec<BacktestTrialDetailResponseArtifactsItem>,
    /// Reward name → score for this trial.
    #[serde(default)]
    pub rewards: HashMap<String, f64>,
    /// Scorers bound to this trial's task (frozen verifiers first, then recipe-level graders), in binding order.
    #[serde(default)]
    pub scorers: Vec<BacktestTrialDetailResponseScorersItem>,
    /// Timeline steps ordered by ordinal.
    #[serde(default)]
    pub steps: Vec<BacktestTrialDetailResponseStepsItem>,
    /// Row projection of `"BacktestTrial"`.
    pub trial: BacktestTrialDetailResponseTrial,
}

impl BacktestTrialDetailResponse {
    pub fn builder() -> BacktestTrialDetailResponseBuilder {
        <BacktestTrialDetailResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BacktestTrialDetailResponseBuilder {
    artifacts: Option<Vec<BacktestTrialDetailResponseArtifactsItem>>,
    rewards: Option<HashMap<String, f64>>,
    scorers: Option<Vec<BacktestTrialDetailResponseScorersItem>>,
    steps: Option<Vec<BacktestTrialDetailResponseStepsItem>>,
    trial: Option<BacktestTrialDetailResponseTrial>,
}

impl BacktestTrialDetailResponseBuilder {
    pub fn artifacts(mut self, value: Vec<BacktestTrialDetailResponseArtifactsItem>) -> Self {
        self.artifacts = Some(value);
        self
    }

    pub fn rewards(mut self, value: HashMap<String, f64>) -> Self {
        self.rewards = Some(value);
        self
    }

    pub fn scorers(mut self, value: Vec<BacktestTrialDetailResponseScorersItem>) -> Self {
        self.scorers = Some(value);
        self
    }

    pub fn steps(mut self, value: Vec<BacktestTrialDetailResponseStepsItem>) -> Self {
        self.steps = Some(value);
        self
    }

    pub fn trial(mut self, value: BacktestTrialDetailResponseTrial) -> Self {
        self.trial = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BacktestTrialDetailResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`artifacts`](BacktestTrialDetailResponseBuilder::artifacts)
    /// - [`rewards`](BacktestTrialDetailResponseBuilder::rewards)
    /// - [`scorers`](BacktestTrialDetailResponseBuilder::scorers)
    /// - [`steps`](BacktestTrialDetailResponseBuilder::steps)
    /// - [`trial`](BacktestTrialDetailResponseBuilder::trial)
    pub fn build(self) -> Result<BacktestTrialDetailResponse, BuildError> {
        Ok(BacktestTrialDetailResponse {
            artifacts: self
                .artifacts
                .ok_or_else(|| BuildError::missing_field("artifacts"))?,
            rewards: self
                .rewards
                .ok_or_else(|| BuildError::missing_field("rewards"))?,
            scorers: self
                .scorers
                .ok_or_else(|| BuildError::missing_field("scorers"))?,
            steps: self
                .steps
                .ok_or_else(|| BuildError::missing_field("steps"))?,
            trial: self
                .trial
                .ok_or_else(|| BuildError::missing_field("trial"))?,
        })
    }
}
