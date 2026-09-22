pub use crate::prelude::*;

/// One task as frozen in a snapshot: the definition plus its verifiers with scorer content embedded. `trace_id` matches the `TraceSummary` in the same snapshot.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TaskSuiteSnapshotTasksItem {
    #[serde(rename = "membershipId")]
    #[serde(default)]
    pub membership_id: String,
    /// The canonical event-store subject curated into a Dataset. Providers that cannot form a real trace keep a single event subject rather than inventing a trace identifier. `Task` marks a hand-authored task with no captured subject; its revision holds zero events and its subject id is minted by the service.
    #[serde(rename = "subjectKind")]
    pub subject_kind: TaskSuiteSnapshotTasksItemSubjectKind,
    /// What a task asks for and how it is judged. Persisted on the membership (mutable working copy) and copied verbatim into every published version item. Verifier bindings live beside it (see `TaskVerifierBinding`).
    #[serde(default)]
    pub task: TaskSuiteSnapshotTasksItemTask,
    #[serde(rename = "traceId")]
    #[serde(default)]
    pub trace_id: String,
    #[serde(default)]
    pub verifiers: Vec<TaskSuiteSnapshotTasksItemVerifiersItem>,
}

impl TaskSuiteSnapshotTasksItem {
    pub fn builder() -> TaskSuiteSnapshotTasksItemBuilder {
        <TaskSuiteSnapshotTasksItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskSuiteSnapshotTasksItemBuilder {
    membership_id: Option<String>,
    subject_kind: Option<TaskSuiteSnapshotTasksItemSubjectKind>,
    task: Option<TaskSuiteSnapshotTasksItemTask>,
    trace_id: Option<String>,
    verifiers: Option<Vec<TaskSuiteSnapshotTasksItemVerifiersItem>>,
}

impl TaskSuiteSnapshotTasksItemBuilder {
    pub fn membership_id(mut self, value: impl Into<String>) -> Self {
        self.membership_id = Some(value.into());
        self
    }

    pub fn subject_kind(mut self, value: TaskSuiteSnapshotTasksItemSubjectKind) -> Self {
        self.subject_kind = Some(value);
        self
    }

    pub fn task(mut self, value: TaskSuiteSnapshotTasksItemTask) -> Self {
        self.task = Some(value);
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn verifiers(mut self, value: Vec<TaskSuiteSnapshotTasksItemVerifiersItem>) -> Self {
        self.verifiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskSuiteSnapshotTasksItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`membership_id`](TaskSuiteSnapshotTasksItemBuilder::membership_id)
    /// - [`subject_kind`](TaskSuiteSnapshotTasksItemBuilder::subject_kind)
    /// - [`task`](TaskSuiteSnapshotTasksItemBuilder::task)
    /// - [`trace_id`](TaskSuiteSnapshotTasksItemBuilder::trace_id)
    /// - [`verifiers`](TaskSuiteSnapshotTasksItemBuilder::verifiers)
    pub fn build(self) -> Result<TaskSuiteSnapshotTasksItem, BuildError> {
        Ok(TaskSuiteSnapshotTasksItem {
            membership_id: self
                .membership_id
                .ok_or_else(|| BuildError::missing_field("membership_id"))?,
            subject_kind: self
                .subject_kind
                .ok_or_else(|| BuildError::missing_field("subject_kind"))?,
            task: self.task.ok_or_else(|| BuildError::missing_field("task"))?,
            trace_id: self
                .trace_id
                .ok_or_else(|| BuildError::missing_field("trace_id"))?,
            verifiers: self
                .verifiers
                .ok_or_else(|| BuildError::missing_field("verifiers"))?,
        })
    }
}
