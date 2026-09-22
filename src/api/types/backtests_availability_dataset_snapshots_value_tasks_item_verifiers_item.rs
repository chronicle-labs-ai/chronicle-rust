pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItem {
    /// Code handler snapshot — required when `kind` is `code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemCode>,
    /// Optional human-readable explanation of why this grader was proposed. For rubric graders this doubles as the judge prompt; library scorers inline their prompt here at launch so the recipe stays self-describing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<String>,
    #[serde(default)]
    pub id: String,
    /// LLM-judge configuration snapshot (rubric graders). `None` falls back to the default free-score judge behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub judge: Option<BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudge>,
    pub kind: BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemKind,
    #[serde(default)]
    pub label: String,
    /// Scores at or above this value render as passing in results.
    #[serde(rename = "passThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pass_threshold: Option<f64>,
    /// Provenance: the library `Scorer.id` this grader was created from (when `source` is `library`). The prompt is snapshotted into `evidence` at launch, so editing the scorer later never mutates a past run.
    #[serde(rename = "scorerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scorer_id: Option<String>,
    /// Grader source — where this grader came from when it was added to the recipe. Determines the chip copy ("proposed" vs "library" vs "custom" vs "dataset").
    pub source: BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemSource,
    /// Grader weight bucket — `low | med | high` matches the segmented control in the GraderBuilder tray.
    pub weight: BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemWeight,
}

impl BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItem {
    pub fn builder() -> BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemBuilder {
        <BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemBuilder {
    code: Option<BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemCode>,
    evidence: Option<String>,
    id: Option<String>,
    judge: Option<BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudge>,
    kind: Option<BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemKind>,
    label: Option<String>,
    pass_threshold: Option<f64>,
    scorer_id: Option<String>,
    source: Option<BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemSource>,
    weight: Option<BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemWeight>,
}

impl BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemBuilder {
    pub fn code(
        mut self,
        value: BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemCode,
    ) -> Self {
        self.code = Some(value);
        self
    }

    pub fn evidence(mut self, value: impl Into<String>) -> Self {
        self.evidence = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn judge(
        mut self,
        value: BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemJudge,
    ) -> Self {
        self.judge = Some(value);
        self
    }

    pub fn kind(
        mut self,
        value: BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemKind,
    ) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn pass_threshold(mut self, value: f64) -> Self {
        self.pass_threshold = Some(value);
        self
    }

    pub fn scorer_id(mut self, value: impl Into<String>) -> Self {
        self.scorer_id = Some(value.into());
        self
    }

    pub fn source(
        mut self,
        value: BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemSource,
    ) -> Self {
        self.source = Some(value);
        self
    }

    pub fn weight(
        mut self,
        value: BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemWeight,
    ) -> Self {
        self.weight = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemBuilder::id)
    /// - [`kind`](BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemBuilder::kind)
    /// - [`label`](BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemBuilder::label)
    /// - [`source`](BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemBuilder::source)
    /// - [`weight`](BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemBuilder::weight)
    pub fn build(
        self,
    ) -> Result<BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItem, BuildError> {
        Ok(
            BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItem {
                code: self.code,
                evidence: self.evidence,
                id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
                judge: self.judge,
                kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
                label: self
                    .label
                    .ok_or_else(|| BuildError::missing_field("label"))?,
                pass_threshold: self.pass_threshold,
                scorer_id: self.scorer_id,
                source: self
                    .source
                    .ok_or_else(|| BuildError::missing_field("source"))?,
                weight: self
                    .weight
                    .ok_or_else(|| BuildError::missing_field("weight"))?,
            },
        )
    }
}
