pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateTaskSuiteWithTraceResponse {
    #[serde(default)]
    pub dataset: CreateTaskSuiteWithTraceResponseDataset,
    /// The compact membership projection returned to Dataset and Timeline clients.
    pub membership: CreateTaskSuiteWithTraceResponseMembership,
}

impl CreateTaskSuiteWithTraceResponse {
    pub fn builder() -> CreateTaskSuiteWithTraceResponseBuilder {
        <CreateTaskSuiteWithTraceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTaskSuiteWithTraceResponseBuilder {
    dataset: Option<CreateTaskSuiteWithTraceResponseDataset>,
    membership: Option<CreateTaskSuiteWithTraceResponseMembership>,
}

impl CreateTaskSuiteWithTraceResponseBuilder {
    pub fn dataset(mut self, value: CreateTaskSuiteWithTraceResponseDataset) -> Self {
        self.dataset = Some(value);
        self
    }

    pub fn membership(mut self, value: CreateTaskSuiteWithTraceResponseMembership) -> Self {
        self.membership = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateTaskSuiteWithTraceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dataset`](CreateTaskSuiteWithTraceResponseBuilder::dataset)
    /// - [`membership`](CreateTaskSuiteWithTraceResponseBuilder::membership)
    pub fn build(self) -> Result<CreateTaskSuiteWithTraceResponse, BuildError> {
        Ok(CreateTaskSuiteWithTraceResponse {
            dataset: self
                .dataset
                .ok_or_else(|| BuildError::missing_field("dataset"))?,
            membership: self
                .membership
                .ok_or_else(|| BuildError::missing_field("membership"))?,
        })
    }
}
