pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TaskSuiteEvalRun {
    /// Display label — usually `agent.name@version` or a build hash.
    #[serde(rename = "agentLabel")]
    #[serde(default)]
    pub agent_label: String,
    /// Tasks that did not pass (see `task_results`).
    #[serde(rename = "failedTraceIds")]
    #[serde(default)]
    pub failed_trace_ids: Vec<String>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// 0–1; null while running.
    #[serde(rename = "passRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pass_rate: Option<f64>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub started_at: DateTime<FixedOffset>,
    /// Status badge tone for an eval run.
    pub status: TaskSuiteEvalRunStatus,
    /// One entry per task whose trials have all finished.
    #[serde(rename = "taskResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_results: Option<Vec<TaskSuiteEvalRunTaskResultsItem>>,
    #[serde(rename = "totalCount")]
    #[serde(default)]
    pub total_count: i64,
}

impl TaskSuiteEvalRun {
    pub fn builder() -> TaskSuiteEvalRunBuilder {
        <TaskSuiteEvalRunBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskSuiteEvalRunBuilder {
    agent_label: Option<String>,
    failed_trace_ids: Option<Vec<String>>,
    id: Option<String>,
    note: Option<String>,
    pass_rate: Option<f64>,
    started_at: Option<DateTime<FixedOffset>>,
    status: Option<TaskSuiteEvalRunStatus>,
    task_results: Option<Vec<TaskSuiteEvalRunTaskResultsItem>>,
    total_count: Option<i64>,
}

impl TaskSuiteEvalRunBuilder {
    pub fn agent_label(mut self, value: impl Into<String>) -> Self {
        self.agent_label = Some(value.into());
        self
    }

    pub fn failed_trace_ids(mut self, value: Vec<String>) -> Self {
        self.failed_trace_ids = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    pub fn pass_rate(mut self, value: f64) -> Self {
        self.pass_rate = Some(value);
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn status(mut self, value: TaskSuiteEvalRunStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn task_results(mut self, value: Vec<TaskSuiteEvalRunTaskResultsItem>) -> Self {
        self.task_results = Some(value);
        self
    }

    pub fn total_count(mut self, value: i64) -> Self {
        self.total_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskSuiteEvalRun`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_label`](TaskSuiteEvalRunBuilder::agent_label)
    /// - [`failed_trace_ids`](TaskSuiteEvalRunBuilder::failed_trace_ids)
    /// - [`id`](TaskSuiteEvalRunBuilder::id)
    /// - [`started_at`](TaskSuiteEvalRunBuilder::started_at)
    /// - [`status`](TaskSuiteEvalRunBuilder::status)
    /// - [`total_count`](TaskSuiteEvalRunBuilder::total_count)
    pub fn build(self) -> Result<TaskSuiteEvalRun, BuildError> {
        Ok(TaskSuiteEvalRun {
            agent_label: self
                .agent_label
                .ok_or_else(|| BuildError::missing_field("agent_label"))?,
            failed_trace_ids: self
                .failed_trace_ids
                .ok_or_else(|| BuildError::missing_field("failed_trace_ids"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            note: self.note,
            pass_rate: self.pass_rate,
            started_at: self
                .started_at
                .ok_or_else(|| BuildError::missing_field("started_at"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            task_results: self.task_results,
            total_count: self
                .total_count
                .ok_or_else(|| BuildError::missing_field("total_count"))?,
        })
    }
}
