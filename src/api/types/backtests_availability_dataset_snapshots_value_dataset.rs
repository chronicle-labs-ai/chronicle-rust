pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BacktestsAvailabilityDatasetSnapshotsValueDataset {
    /// Soft-archive timestamp. Present only when `includeArchived=true` or an archived Dataset is read directly.
    #[serde(rename = "archivedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<DateTime<FixedOffset>>,
    /// Display name of the dataset owner / creator.
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional total event count across all traces.
    #[serde(rename = "eventCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_count: Option<i64>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    /// Intended use of a dataset — drives the colored badge on the picker and lets apps route additions to the right backend (eval suite, training set, replay corpus, manual review queue).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<BacktestsAvailabilityDatasetSnapshotsValueDatasetPurpose>,
    /// Free-form pinned tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Number of traces currently in the dataset.
    #[serde(rename = "traceCount")]
    #[serde(default)]
    pub trace_count: i64,
    /// ISO timestamp of the most recent addition.
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl BacktestsAvailabilityDatasetSnapshotsValueDataset {
    pub fn builder() -> BacktestsAvailabilityDatasetSnapshotsValueDatasetBuilder {
        <BacktestsAvailabilityDatasetSnapshotsValueDatasetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BacktestsAvailabilityDatasetSnapshotsValueDatasetBuilder {
    archived_at: Option<DateTime<FixedOffset>>,
    created_by: Option<String>,
    description: Option<String>,
    event_count: Option<i64>,
    id: Option<String>,
    name: Option<String>,
    purpose: Option<BacktestsAvailabilityDatasetSnapshotsValueDatasetPurpose>,
    tags: Option<Vec<String>>,
    trace_count: Option<i64>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl BacktestsAvailabilityDatasetSnapshotsValueDatasetBuilder {
    pub fn archived_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.archived_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn event_count(mut self, value: i64) -> Self {
        self.event_count = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn purpose(
        mut self,
        value: BacktestsAvailabilityDatasetSnapshotsValueDatasetPurpose,
    ) -> Self {
        self.purpose = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn trace_count(mut self, value: i64) -> Self {
        self.trace_count = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BacktestsAvailabilityDatasetSnapshotsValueDataset`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BacktestsAvailabilityDatasetSnapshotsValueDatasetBuilder::id)
    /// - [`name`](BacktestsAvailabilityDatasetSnapshotsValueDatasetBuilder::name)
    /// - [`trace_count`](BacktestsAvailabilityDatasetSnapshotsValueDatasetBuilder::trace_count)
    pub fn build(self) -> Result<BacktestsAvailabilityDatasetSnapshotsValueDataset, BuildError> {
        Ok(BacktestsAvailabilityDatasetSnapshotsValueDataset {
            archived_at: self.archived_at,
            created_by: self.created_by,
            description: self.description,
            event_count: self.event_count,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            purpose: self.purpose,
            tags: self.tags,
            trace_count: self
                .trace_count
                .ok_or_else(|| BuildError::missing_field("trace_count"))?,
            updated_at: self.updated_at,
        })
    }
}
