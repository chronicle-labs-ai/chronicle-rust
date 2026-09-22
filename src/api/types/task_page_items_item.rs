pub use crate::prelude::*;

/// One task in a dataset: the trace summary (its seed, when it has one), the stable membership identity used for lazy event reads and explicit refreshes, and the task definition with its verifier bindings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TaskPageItemsItem {
    #[serde(rename = "addedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "addedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_by: Option<String>,
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "durationMs")]
    #[serde(default)]
    pub duration_ms: i64,
    /// Pre-computed 2D embedding in normalized `[-1, 1]` space.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding: Option<Vec<f64>>,
    #[serde(rename = "eventCount")]
    #[serde(default)]
    pub event_count: i64,
    #[serde(default)]
    pub label: String,
    #[serde(rename = "membershipId")]
    #[serde(default)]
    pub membership_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "primarySource")]
    #[serde(default)]
    pub primary_source: String,
    #[serde(rename = "refreshAvailable")]
    #[serde(default)]
    pub refresh_available: bool,
    #[serde(default)]
    pub revision: i64,
    #[serde(default)]
    pub sources: Vec<String>,
    /// Train / validation / test split assignment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split: Option<TaskPageItemsItemSplit>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub started_at: DateTime<FixedOffset>,
    /// Health status of a trace as judged by the dataset owner.
    pub status: TaskPageItemsItemStatus,
    #[serde(rename = "subjectId")]
    #[serde(default)]
    pub subject_id: String,
    /// The canonical event-store subject curated into a Dataset. Providers that cannot form a real trace keep a single event subject rather than inventing a trace identifier. `Task` marks a hand-authored task with no captured subject; its revision holds zero events and its subject id is minted by the service.
    #[serde(rename = "subjectKind")]
    pub subject_kind: TaskPageItemsItemSubjectKind,
    /// What a task asks for and how it is judged. Persisted on the membership (mutable working copy) and copied verbatim into every published version item. Verifier bindings live beside it (see `TaskVerifierBinding`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<TaskPageItemsItemTask>,
    #[serde(rename = "traceId")]
    #[serde(default)]
    pub trace_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifiers: Option<Vec<TaskPageItemsItemVerifiersItem>>,
}

impl TaskPageItemsItem {
    pub fn builder() -> TaskPageItemsItemBuilder {
        <TaskPageItemsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskPageItemsItemBuilder {
    added_at: Option<DateTime<FixedOffset>>,
    added_by: Option<String>,
    cluster_id: Option<String>,
    duration_ms: Option<i64>,
    embedding: Option<Vec<f64>>,
    event_count: Option<i64>,
    label: Option<String>,
    membership_id: Option<String>,
    note: Option<String>,
    primary_source: Option<String>,
    refresh_available: Option<bool>,
    revision: Option<i64>,
    sources: Option<Vec<String>>,
    split: Option<TaskPageItemsItemSplit>,
    started_at: Option<DateTime<FixedOffset>>,
    status: Option<TaskPageItemsItemStatus>,
    subject_id: Option<String>,
    subject_kind: Option<TaskPageItemsItemSubjectKind>,
    task: Option<TaskPageItemsItemTask>,
    trace_id: Option<String>,
    verifiers: Option<Vec<TaskPageItemsItemVerifiersItem>>,
}

impl TaskPageItemsItemBuilder {
    pub fn added_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.added_at = Some(value);
        self
    }

    pub fn added_by(mut self, value: impl Into<String>) -> Self {
        self.added_by = Some(value.into());
        self
    }

    pub fn cluster_id(mut self, value: impl Into<String>) -> Self {
        self.cluster_id = Some(value.into());
        self
    }

    pub fn duration_ms(mut self, value: i64) -> Self {
        self.duration_ms = Some(value);
        self
    }

    pub fn embedding(mut self, value: Vec<f64>) -> Self {
        self.embedding = Some(value);
        self
    }

    pub fn event_count(mut self, value: i64) -> Self {
        self.event_count = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn membership_id(mut self, value: impl Into<String>) -> Self {
        self.membership_id = Some(value.into());
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    pub fn primary_source(mut self, value: impl Into<String>) -> Self {
        self.primary_source = Some(value.into());
        self
    }

    pub fn refresh_available(mut self, value: bool) -> Self {
        self.refresh_available = Some(value);
        self
    }

    pub fn revision(mut self, value: i64) -> Self {
        self.revision = Some(value);
        self
    }

    pub fn sources(mut self, value: Vec<String>) -> Self {
        self.sources = Some(value);
        self
    }

    pub fn split(mut self, value: TaskPageItemsItemSplit) -> Self {
        self.split = Some(value);
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn status(mut self, value: TaskPageItemsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn subject_id(mut self, value: impl Into<String>) -> Self {
        self.subject_id = Some(value.into());
        self
    }

    pub fn subject_kind(mut self, value: TaskPageItemsItemSubjectKind) -> Self {
        self.subject_kind = Some(value);
        self
    }

    pub fn task(mut self, value: TaskPageItemsItemTask) -> Self {
        self.task = Some(value);
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn verifiers(mut self, value: Vec<TaskPageItemsItemVerifiersItem>) -> Self {
        self.verifiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskPageItemsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`duration_ms`](TaskPageItemsItemBuilder::duration_ms)
    /// - [`event_count`](TaskPageItemsItemBuilder::event_count)
    /// - [`label`](TaskPageItemsItemBuilder::label)
    /// - [`membership_id`](TaskPageItemsItemBuilder::membership_id)
    /// - [`primary_source`](TaskPageItemsItemBuilder::primary_source)
    /// - [`refresh_available`](TaskPageItemsItemBuilder::refresh_available)
    /// - [`revision`](TaskPageItemsItemBuilder::revision)
    /// - [`sources`](TaskPageItemsItemBuilder::sources)
    /// - [`started_at`](TaskPageItemsItemBuilder::started_at)
    /// - [`status`](TaskPageItemsItemBuilder::status)
    /// - [`subject_id`](TaskPageItemsItemBuilder::subject_id)
    /// - [`subject_kind`](TaskPageItemsItemBuilder::subject_kind)
    /// - [`trace_id`](TaskPageItemsItemBuilder::trace_id)
    pub fn build(self) -> Result<TaskPageItemsItem, BuildError> {
        Ok(TaskPageItemsItem {
            added_at: self.added_at,
            added_by: self.added_by,
            cluster_id: self.cluster_id,
            duration_ms: self
                .duration_ms
                .ok_or_else(|| BuildError::missing_field("duration_ms"))?,
            embedding: self.embedding,
            event_count: self
                .event_count
                .ok_or_else(|| BuildError::missing_field("event_count"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            membership_id: self
                .membership_id
                .ok_or_else(|| BuildError::missing_field("membership_id"))?,
            note: self.note,
            primary_source: self
                .primary_source
                .ok_or_else(|| BuildError::missing_field("primary_source"))?,
            refresh_available: self
                .refresh_available
                .ok_or_else(|| BuildError::missing_field("refresh_available"))?,
            revision: self
                .revision
                .ok_or_else(|| BuildError::missing_field("revision"))?,
            sources: self
                .sources
                .ok_or_else(|| BuildError::missing_field("sources"))?,
            split: self.split,
            started_at: self
                .started_at
                .ok_or_else(|| BuildError::missing_field("started_at"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            subject_id: self
                .subject_id
                .ok_or_else(|| BuildError::missing_field("subject_id"))?,
            subject_kind: self
                .subject_kind
                .ok_or_else(|| BuildError::missing_field("subject_kind"))?,
            task: self.task,
            trace_id: self
                .trace_id
                .ok_or_else(|| BuildError::missing_field("trace_id"))?,
            verifiers: self.verifiers,
        })
    }
}
