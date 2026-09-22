pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TaskSuiteSnapshotTracesItem {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "primarySource")]
    #[serde(default)]
    pub primary_source: String,
    #[serde(default)]
    pub sources: Vec<String>,
    /// Train / validation / test split assignment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split: Option<TaskSuiteSnapshotTracesItemSplit>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub started_at: DateTime<FixedOffset>,
    /// Health status of a trace as judged by the dataset owner.
    pub status: TaskSuiteSnapshotTracesItemStatus,
    #[serde(rename = "traceId")]
    #[serde(default)]
    pub trace_id: String,
}

impl TaskSuiteSnapshotTracesItem {
    pub fn builder() -> TaskSuiteSnapshotTracesItemBuilder {
        <TaskSuiteSnapshotTracesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskSuiteSnapshotTracesItemBuilder {
    added_at: Option<DateTime<FixedOffset>>,
    added_by: Option<String>,
    cluster_id: Option<String>,
    duration_ms: Option<i64>,
    embedding: Option<Vec<f64>>,
    event_count: Option<i64>,
    label: Option<String>,
    note: Option<String>,
    primary_source: Option<String>,
    sources: Option<Vec<String>>,
    split: Option<TaskSuiteSnapshotTracesItemSplit>,
    started_at: Option<DateTime<FixedOffset>>,
    status: Option<TaskSuiteSnapshotTracesItemStatus>,
    trace_id: Option<String>,
}

impl TaskSuiteSnapshotTracesItemBuilder {
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

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    pub fn primary_source(mut self, value: impl Into<String>) -> Self {
        self.primary_source = Some(value.into());
        self
    }

    pub fn sources(mut self, value: Vec<String>) -> Self {
        self.sources = Some(value);
        self
    }

    pub fn split(mut self, value: TaskSuiteSnapshotTracesItemSplit) -> Self {
        self.split = Some(value);
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn status(mut self, value: TaskSuiteSnapshotTracesItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskSuiteSnapshotTracesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`duration_ms`](TaskSuiteSnapshotTracesItemBuilder::duration_ms)
    /// - [`event_count`](TaskSuiteSnapshotTracesItemBuilder::event_count)
    /// - [`label`](TaskSuiteSnapshotTracesItemBuilder::label)
    /// - [`primary_source`](TaskSuiteSnapshotTracesItemBuilder::primary_source)
    /// - [`sources`](TaskSuiteSnapshotTracesItemBuilder::sources)
    /// - [`started_at`](TaskSuiteSnapshotTracesItemBuilder::started_at)
    /// - [`status`](TaskSuiteSnapshotTracesItemBuilder::status)
    /// - [`trace_id`](TaskSuiteSnapshotTracesItemBuilder::trace_id)
    pub fn build(self) -> Result<TaskSuiteSnapshotTracesItem, BuildError> {
        Ok(TaskSuiteSnapshotTracesItem {
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
            note: self.note,
            primary_source: self
                .primary_source
                .ok_or_else(|| BuildError::missing_field("primary_source"))?,
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
            trace_id: self
                .trace_id
                .ok_or_else(|| BuildError::missing_field("trace_id"))?,
        })
    }
}
