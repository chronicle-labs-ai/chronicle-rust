pub use crate::prelude::*;

/// A scorer as the trial view needs it: the frozen grader snapshot the reward key points at, plus the reward key itself so the UI can join scores to names without reconstructing `grader_<id>`.
///
/// Resolved from the job's pinned Dataset Version items (task verifiers, `source: dataset`) and the recipe's own grader list.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BacktestTrialDetailResponseScorersItem {
    pub grader: BacktestTrialDetailResponseScorersItemGrader,
    /// Reward key this scorer's score is stored under (`grader_<id>`).
    #[serde(rename = "rewardKey")]
    #[serde(default)]
    pub reward_key: String,
}

impl BacktestTrialDetailResponseScorersItem {
    pub fn builder() -> BacktestTrialDetailResponseScorersItemBuilder {
        <BacktestTrialDetailResponseScorersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BacktestTrialDetailResponseScorersItemBuilder {
    grader: Option<BacktestTrialDetailResponseScorersItemGrader>,
    reward_key: Option<String>,
}

impl BacktestTrialDetailResponseScorersItemBuilder {
    pub fn grader(mut self, value: BacktestTrialDetailResponseScorersItemGrader) -> Self {
        self.grader = Some(value);
        self
    }

    pub fn reward_key(mut self, value: impl Into<String>) -> Self {
        self.reward_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BacktestTrialDetailResponseScorersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`grader`](BacktestTrialDetailResponseScorersItemBuilder::grader)
    /// - [`reward_key`](BacktestTrialDetailResponseScorersItemBuilder::reward_key)
    pub fn build(self) -> Result<BacktestTrialDetailResponseScorersItem, BuildError> {
        Ok(BacktestTrialDetailResponseScorersItem {
            grader: self
                .grader
                .ok_or_else(|| BuildError::missing_field("grader"))?,
            reward_key: self
                .reward_key
                .ok_or_else(|| BuildError::missing_field("reward_key"))?,
        })
    }
}
