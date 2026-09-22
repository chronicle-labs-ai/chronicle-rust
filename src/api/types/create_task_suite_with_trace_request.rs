pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateTaskSuiteWithTraceRequest {
    #[serde(default)]
    pub dataset: CreateTaskSuiteWithTraceRequestDataset,
    #[serde(default)]
    pub trace: CreateTaskSuiteWithTraceRequestTrace,
}

impl CreateTaskSuiteWithTraceRequest {
    pub fn builder() -> CreateTaskSuiteWithTraceRequestBuilder {
        <CreateTaskSuiteWithTraceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTaskSuiteWithTraceRequestBuilder {
    dataset: Option<CreateTaskSuiteWithTraceRequestDataset>,
    trace: Option<CreateTaskSuiteWithTraceRequestTrace>,
}

impl CreateTaskSuiteWithTraceRequestBuilder {
    pub fn dataset(mut self, value: CreateTaskSuiteWithTraceRequestDataset) -> Self {
        self.dataset = Some(value);
        self
    }

    pub fn trace(mut self, value: CreateTaskSuiteWithTraceRequestTrace) -> Self {
        self.trace = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateTaskSuiteWithTraceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dataset`](CreateTaskSuiteWithTraceRequestBuilder::dataset)
    /// - [`trace`](CreateTaskSuiteWithTraceRequestBuilder::trace)
    pub fn build(self) -> Result<CreateTaskSuiteWithTraceRequest, BuildError> {
        Ok(CreateTaskSuiteWithTraceRequest {
            dataset: self
                .dataset
                .ok_or_else(|| BuildError::missing_field("dataset"))?,
            trace: self
                .trace
                .ok_or_else(|| BuildError::missing_field("trace"))?,
        })
    }
}
