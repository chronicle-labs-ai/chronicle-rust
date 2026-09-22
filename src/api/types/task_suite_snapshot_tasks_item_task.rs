pub use crate::prelude::*;

/// What a task asks for and how it is judged. Persisted on the membership (mutable working copy) and copied verbatim into every published version item. Verifier bindings live beside it (see `TaskVerifierBinding`).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskSuiteSnapshotTasksItemTask {
    /// Runtime knobs a task carries, mirroring the `[agent]`, `[verifier]` and `[environment]` tables of a Harbor `task.toml`. Every field is optional; the server policy fills defaults at launch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<TaskSuiteSnapshotTasksItemTaskConfig>,
    #[serde(rename = "environmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[serde(rename = "environmentVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_version_id: Option<String>,
    /// What "done correctly" looks like. Passed to graders as the gold reference. For trace-seeded tasks this defaults to the events that followed the seed cutoff.
    #[serde(rename = "expectedOutcome")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_outcome: Option<serde_json::Value>,
    /// The goal, as markdown. Harbor's `instruction.md`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction: Option<String>,
    /// Id of the last seed event. Events up to and including it are the context the agent sees; later events are the recorded outcome.
    #[serde(rename = "seedCutoffEventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_cutoff_event_id: Option<String>,
    /// Optional oracle: how a correct agent would solve this. Harbor's `solution/solve.sh` body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution: Option<String>,
    /// Short human name. Defaults to the trace label for trace-seeded tasks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl TaskSuiteSnapshotTasksItemTask {
    pub fn builder() -> TaskSuiteSnapshotTasksItemTaskBuilder {
        <TaskSuiteSnapshotTasksItemTaskBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskSuiteSnapshotTasksItemTaskBuilder {
    config: Option<TaskSuiteSnapshotTasksItemTaskConfig>,
    environment_id: Option<String>,
    environment_version_id: Option<String>,
    expected_outcome: Option<serde_json::Value>,
    instruction: Option<String>,
    seed_cutoff_event_id: Option<String>,
    solution: Option<String>,
    title: Option<String>,
}

impl TaskSuiteSnapshotTasksItemTaskBuilder {
    pub fn config(mut self, value: TaskSuiteSnapshotTasksItemTaskConfig) -> Self {
        self.config = Some(value);
        self
    }

    pub fn environment_id(mut self, value: impl Into<String>) -> Self {
        self.environment_id = Some(value.into());
        self
    }

    pub fn environment_version_id(mut self, value: impl Into<String>) -> Self {
        self.environment_version_id = Some(value.into());
        self
    }

    pub fn expected_outcome(mut self, value: serde_json::Value) -> Self {
        self.expected_outcome = Some(value);
        self
    }

    pub fn instruction(mut self, value: impl Into<String>) -> Self {
        self.instruction = Some(value.into());
        self
    }

    pub fn seed_cutoff_event_id(mut self, value: impl Into<String>) -> Self {
        self.seed_cutoff_event_id = Some(value.into());
        self
    }

    pub fn solution(mut self, value: impl Into<String>) -> Self {
        self.solution = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskSuiteSnapshotTasksItemTask`].
    pub fn build(self) -> Result<TaskSuiteSnapshotTasksItemTask, BuildError> {
        Ok(TaskSuiteSnapshotTasksItemTask {
            config: self.config,
            environment_id: self.environment_id,
            environment_version_id: self.environment_version_id,
            expected_outcome: self.expected_outcome,
            instruction: self.instruction,
            seed_cutoff_event_id: self.seed_cutoff_event_id,
            solution: self.solution,
            title: self.title,
        })
    }
}
