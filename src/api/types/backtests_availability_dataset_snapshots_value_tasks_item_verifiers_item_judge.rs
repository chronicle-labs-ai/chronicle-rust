pub use crate::prelude::*;

/// LLM-judge configuration snapshot (rubric graders). `None` falls back to the default free-score judge behavior.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudge {
    /// Allow the judge to decline non-applicable cases; skipped cases emit no reward key instead of scoring 0.
    #[serde(rename = "allowSkip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_skip: Option<bool>,
    /// Choice→score mapping. Non-empty forces the judge to pick one choice; the mapped score is the result.
    #[serde(rename = "choiceScores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choice_scores: Option<
        Vec<BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudgeChoiceScoresItem>,
    >,
    /// Judge model override; `None` uses the server default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Ask the judge to reason step-by-step before answering.
    #[serde(rename = "useCot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_cot: Option<bool>,
}

impl BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudge {
    pub fn builder() -> BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudgeBuilder
    {
        <BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudgeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudgeBuilder {
    allow_skip: Option<bool>,
    choice_scores: Option<
        Vec<BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudgeChoiceScoresItem>,
    >,
    model: Option<String>,
    use_cot: Option<bool>,
}

impl BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudgeBuilder {
    pub fn allow_skip(mut self, value: bool) -> Self {
        self.allow_skip = Some(value);
        self
    }

    pub fn choice_scores(
        mut self,
        value: Vec<
            BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudgeChoiceScoresItem,
        >,
    ) -> Self {
        self.choice_scores = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn use_cot(mut self, value: bool) -> Self {
        self.use_cot = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudge`].
    pub fn build(
        self,
    ) -> Result<BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudge, BuildError>
    {
        Ok(
            BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudge {
                allow_skip: self.allow_skip,
                choice_scores: self.choice_scores,
                model: self.model,
                use_cot: self.use_cot,
            },
        )
    }
}
