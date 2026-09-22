pub use crate::prelude::*;

/// The compact membership projection returned to Dataset and Timeline clients.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AddTaskFromTraceResponseMembership {
    #[serde(rename = "addedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_at: Option<String>,
    #[serde(rename = "datasetId")]
    #[serde(default)]
    pub dataset_id: String,
    #[serde(rename = "datasetName")]
    #[serde(default)]
    pub dataset_name: String,
    #[serde(rename = "eventCount")]
    #[serde(default)]
    pub event_count: i64,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Intended use of a dataset — drives the colored badge on the picker and lets apps route additions to the right backend (eval suite, training set, replay corpus, manual review queue).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<AddTaskFromTraceResponseMembershipPurpose>,
    #[serde(rename = "refreshAvailable")]
    #[serde(default)]
    pub refresh_available: bool,
    #[serde(default)]
    pub revision: i64,
    /// Train / validation / test split assignment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split: Option<AddTaskFromTraceResponseMembershipSplit>,
    #[serde(rename = "subjectId")]
    #[serde(default)]
    pub subject_id: String,
    /// The canonical event-store subject curated into a Dataset. Providers that cannot form a real trace keep a single event subject rather than inventing a trace identifier. `Task` marks a hand-authored task with no captured subject; its revision holds zero events and its subject id is minted by the service.
    #[serde(rename = "subjectKind")]
    pub subject_kind: AddTaskFromTraceResponseMembershipSubjectKind,
    /// Compatibility identity consumed by the Timeline picker. For an event membership this is the event id.
    #[serde(rename = "traceId")]
    #[serde(default)]
    pub trace_id: String,
}

impl AddTaskFromTraceResponseMembership {
    pub fn builder() -> AddTaskFromTraceResponseMembershipBuilder {
        <AddTaskFromTraceResponseMembershipBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddTaskFromTraceResponseMembershipBuilder {
    added_at: Option<String>,
    dataset_id: Option<String>,
    dataset_name: Option<String>,
    event_count: Option<i64>,
    id: Option<String>,
    note: Option<String>,
    purpose: Option<AddTaskFromTraceResponseMembershipPurpose>,
    refresh_available: Option<bool>,
    revision: Option<i64>,
    split: Option<AddTaskFromTraceResponseMembershipSplit>,
    subject_id: Option<String>,
    subject_kind: Option<AddTaskFromTraceResponseMembershipSubjectKind>,
    trace_id: Option<String>,
}

impl AddTaskFromTraceResponseMembershipBuilder {
    pub fn added_at(mut self, value: impl Into<String>) -> Self {
        self.added_at = Some(value.into());
        self
    }

    pub fn dataset_id(mut self, value: impl Into<String>) -> Self {
        self.dataset_id = Some(value.into());
        self
    }

    pub fn dataset_name(mut self, value: impl Into<String>) -> Self {
        self.dataset_name = Some(value.into());
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

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    pub fn purpose(mut self, value: AddTaskFromTraceResponseMembershipPurpose) -> Self {
        self.purpose = Some(value);
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

    pub fn split(mut self, value: AddTaskFromTraceResponseMembershipSplit) -> Self {
        self.split = Some(value);
        self
    }

    pub fn subject_id(mut self, value: impl Into<String>) -> Self {
        self.subject_id = Some(value.into());
        self
    }

    pub fn subject_kind(mut self, value: AddTaskFromTraceResponseMembershipSubjectKind) -> Self {
        self.subject_kind = Some(value);
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AddTaskFromTraceResponseMembership`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dataset_id`](AddTaskFromTraceResponseMembershipBuilder::dataset_id)
    /// - [`dataset_name`](AddTaskFromTraceResponseMembershipBuilder::dataset_name)
    /// - [`event_count`](AddTaskFromTraceResponseMembershipBuilder::event_count)
    /// - [`id`](AddTaskFromTraceResponseMembershipBuilder::id)
    /// - [`refresh_available`](AddTaskFromTraceResponseMembershipBuilder::refresh_available)
    /// - [`revision`](AddTaskFromTraceResponseMembershipBuilder::revision)
    /// - [`subject_id`](AddTaskFromTraceResponseMembershipBuilder::subject_id)
    /// - [`subject_kind`](AddTaskFromTraceResponseMembershipBuilder::subject_kind)
    /// - [`trace_id`](AddTaskFromTraceResponseMembershipBuilder::trace_id)
    pub fn build(self) -> Result<AddTaskFromTraceResponseMembership, BuildError> {
        Ok(AddTaskFromTraceResponseMembership {
            added_at: self.added_at,
            dataset_id: self
                .dataset_id
                .ok_or_else(|| BuildError::missing_field("dataset_id"))?,
            dataset_name: self
                .dataset_name
                .ok_or_else(|| BuildError::missing_field("dataset_name"))?,
            event_count: self
                .event_count
                .ok_or_else(|| BuildError::missing_field("event_count"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            note: self.note,
            purpose: self.purpose,
            refresh_available: self
                .refresh_available
                .ok_or_else(|| BuildError::missing_field("refresh_available"))?,
            revision: self
                .revision
                .ok_or_else(|| BuildError::missing_field("revision"))?,
            split: self.split,
            subject_id: self
                .subject_id
                .ok_or_else(|| BuildError::missing_field("subject_id"))?,
            subject_kind: self
                .subject_kind
                .ok_or_else(|| BuildError::missing_field("subject_kind"))?,
            trace_id: self
                .trace_id
                .ok_or_else(|| BuildError::missing_field("trace_id"))?,
        })
    }
}
