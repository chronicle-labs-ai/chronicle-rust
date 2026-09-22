pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AddTaskFromTraceRequest {
    /// Accepted for compatibility but never trusted as the authoritative capture. The service re-reads the canonical store by subject.
    #[serde(rename = "eventIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ids: Option<Vec<String>>,
    #[serde(rename = "idempotencyKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_task_from_trace_request_idempotency_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Train / validation / test split assignment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split: Option<AddTaskFromTraceRequestSplit>,
    /// Optional task fields. Anything left unset is derived from the captured trace (title from the label, instruction from the first message, expected outcome from the events after the cutoff).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<AddTaskFromTraceRequestTask>,
    #[serde(rename = "traceId")]
    #[serde(default)]
    pub trace_id: String,
    #[serde(rename = "traceSynthesized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_synthesized: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifiers: Option<Vec<AddTaskFromTraceRequestVerifiersItem>>,
}

impl AddTaskFromTraceRequest {
    pub fn builder() -> AddTaskFromTraceRequestBuilder {
        <AddTaskFromTraceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddTaskFromTraceRequestBuilder {
    event_ids: Option<Vec<String>>,
    add_task_from_trace_request_idempotency_key: Option<String>,
    notes: Option<String>,
    split: Option<AddTaskFromTraceRequestSplit>,
    task: Option<AddTaskFromTraceRequestTask>,
    trace_id: Option<String>,
    trace_synthesized: Option<bool>,
    verifiers: Option<Vec<AddTaskFromTraceRequestVerifiersItem>>,
}

impl AddTaskFromTraceRequestBuilder {
    pub fn event_ids(mut self, value: Vec<String>) -> Self {
        self.event_ids = Some(value);
        self
    }

    pub fn add_task_from_trace_request_idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.add_task_from_trace_request_idempotency_key = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn split(mut self, value: AddTaskFromTraceRequestSplit) -> Self {
        self.split = Some(value);
        self
    }

    pub fn task(mut self, value: AddTaskFromTraceRequestTask) -> Self {
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

    pub fn verifiers(mut self, value: Vec<AddTaskFromTraceRequestVerifiersItem>) -> Self {
        self.verifiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddTaskFromTraceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`trace_id`](AddTaskFromTraceRequestBuilder::trace_id)
    pub fn build(self) -> Result<AddTaskFromTraceRequest, BuildError> {
        Ok(AddTaskFromTraceRequest {
            event_ids: self.event_ids,
            add_task_from_trace_request_idempotency_key: self
                .add_task_from_trace_request_idempotency_key,
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
