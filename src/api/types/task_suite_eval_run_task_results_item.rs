pub use crate::prelude::*;

/// How one task fared in an eval run, aggregated over its trials.
///
/// A task passes when every trial reached a terminal success and each verifier that declares a `passThreshold` scored at or above it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskSuiteEvalRunTaskResultsItem {
    /// Verifiers whose reward fell below their pass threshold (or was missing) in at least one trial.
    #[serde(rename = "failedVerifierIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_verifier_ids: Option<Vec<String>>,
    #[serde(default)]
    pub passed: bool,
    /// Mean of the top-level rewards across the task's trials; null when no trial recorded a reward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    /// The task id (the membership subject id, which is also the case id).
    #[serde(rename = "traceId")]
    #[serde(default)]
    pub trace_id: String,
}

impl TaskSuiteEvalRunTaskResultsItem {
    pub fn builder() -> TaskSuiteEvalRunTaskResultsItemBuilder {
        <TaskSuiteEvalRunTaskResultsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskSuiteEvalRunTaskResultsItemBuilder {
    failed_verifier_ids: Option<Vec<String>>,
    passed: Option<bool>,
    score: Option<f64>,
    trace_id: Option<String>,
}

impl TaskSuiteEvalRunTaskResultsItemBuilder {
    pub fn failed_verifier_ids(mut self, value: Vec<String>) -> Self {
        self.failed_verifier_ids = Some(value);
        self
    }

    pub fn passed(mut self, value: bool) -> Self {
        self.passed = Some(value);
        self
    }

    pub fn score(mut self, value: f64) -> Self {
        self.score = Some(value);
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskSuiteEvalRunTaskResultsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`passed`](TaskSuiteEvalRunTaskResultsItemBuilder::passed)
    /// - [`trace_id`](TaskSuiteEvalRunTaskResultsItemBuilder::trace_id)
    pub fn build(self) -> Result<TaskSuiteEvalRunTaskResultsItem, BuildError> {
        Ok(TaskSuiteEvalRunTaskResultsItem {
            failed_verifier_ids: self.failed_verifier_ids,
            passed: self
                .passed
                .ok_or_else(|| BuildError::missing_field("passed"))?,
            score: self.score,
            trace_id: self
                .trace_id
                .ok_or_else(|| BuildError::missing_field("trace_id"))?,
        })
    }
}
