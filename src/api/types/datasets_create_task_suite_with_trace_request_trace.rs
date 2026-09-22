pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateTaskSuiteWithTraceRequestTrace {
    /// Accepted for compatibility but never trusted as the authoritative capture. The service re-reads the canonical store by subject.
    #[serde(rename = "eventIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ids: Option<Vec<String>>,
    #[serde(rename = "idempotencyKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Train / validation / test split assignment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split: Option<CreateTaskSuiteWithTraceRequestTraceSplit>,
    /// Optional task fields. Anything left unset is derived from the captured trace (title from the label, instruction from the first message, expected outcome from the events after the cutoff).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<CreateTaskSuiteWithTraceRequestTraceTask>,
    #[serde(rename = "traceId")]
    #[serde(default)]
    pub trace_id: String,
    #[serde(rename = "traceSynthesized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_synthesized: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifiers: Option<Vec<CreateTaskSuiteWithTraceRequestTraceVerifiersItem>>,
}

impl CreateTaskSuiteWithTraceRequestTrace {
    pub fn builder() -> CreateTaskSuiteWithTraceRequestTraceBuilder {
        <CreateTaskSuiteWithTraceRequestTraceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTaskSuiteWithTraceRequestTraceBuilder {
    event_ids: Option<Vec<String>>,
    idempotency_key: Option<String>,
    notes: Option<String>,
    split: Option<CreateTaskSuiteWithTraceRequestTraceSplit>,
    task: Option<CreateTaskSuiteWithTraceRequestTraceTask>,
    trace_id: Option<String>,
    trace_synthesized: Option<bool>,
    verifiers: Option<Vec<CreateTaskSuiteWithTraceRequestTraceVerifiersItem>>,
}

impl CreateTaskSuiteWithTraceRequestTraceBuilder {
    pub fn event_ids(mut self, value: Vec<String>) -> Self {
        self.event_ids = Some(value);
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn split(mut self, value: CreateTaskSuiteWithTraceRequestTraceSplit) -> Self {
        self.split = Some(value);
        self
    }

    pub fn task(mut self, value: CreateTaskSuiteWithTraceRequestTraceTask) -> Self {
        self.task = Some(value);
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn trace_synthesized(mut self, value: bool) -> Self {
        self.trace_synthesized = Some(value);
        self
    }

    pub fn verifiers(
        mut self,
        value: Vec<CreateTaskSuiteWithTraceRequestTraceVerifiersItem>,
    ) -> Self {
        self.verifiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateTaskSuiteWithTraceRequestTrace`].
    /// This method will fail if any of the following fields are not set:
    /// - [`trace_id`](CreateTaskSuiteWithTraceRequestTraceBuilder::trace_id)
    pub fn build(self) -> Result<CreateTaskSuiteWithTraceRequestTrace, BuildError> {
        Ok(CreateTaskSuiteWithTraceRequestTrace {
            event_ids: self.event_ids,
            idempotency_key: self.idempotency_key,
            notes: self.notes,
            split: self.split,
            task: self.task,
            trace_id: self
                .trace_id
                .ok_or_else(|| BuildError::missing_field("trace_id"))?,
            trace_synthesized: self.trace_synthesized,
            verifiers: self.verifiers,
        })
    }
}
