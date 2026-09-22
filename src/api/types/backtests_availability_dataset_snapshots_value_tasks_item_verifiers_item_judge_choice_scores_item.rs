pub use crate::prelude::*;

/// One judge choice mapped to a numeric score, e.g. `A → 1.0`, `B → 0.5`, `C → 0.0`. When a judge scorer declares choices, the model must pick exactly one and the mapped score is the result.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudgeChoiceScoresItem {
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub score: f64,
}

impl BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudgeChoiceScoresItem {
    pub fn builder(
    ) -> BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudgeChoiceScoresItemBuilder
    {
        <BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudgeChoiceScoresItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudgeChoiceScoresItemBuilder
{
    label: Option<String>,
    score: Option<f64>,
}

impl BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudgeChoiceScoresItemBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn score(mut self, value: f64) -> Self {
        self.score = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudgeChoiceScoresItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudgeChoiceScoresItemBuilder::label)
    /// - [`score`](BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudgeChoiceScoresItemBuilder::score)
    pub fn build(
        self,
    ) -> Result<
        BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudgeChoiceScoresItem,
        BuildError,
    > {
        Ok(
            BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudgeChoiceScoresItem {
                label: self
                    .label
                    .ok_or_else(|| BuildError::missing_field("label"))?,
                score: self
                    .score
                    .ok_or_else(|| BuildError::missing_field("score"))?,
            },
        )
    }
}
