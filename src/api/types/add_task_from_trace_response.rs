pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AddTaskFromTraceResponse {
    #[serde(default)]
    pub dataset: AddTaskFromTraceResponseDataset,
    /// The compact membership projection returned to Dataset and Timeline clients.
    pub membership: AddTaskFromTraceResponseMembership,
}

impl AddTaskFromTraceResponse {
    pub fn builder() -> AddTaskFromTraceResponseBuilder {
        <AddTaskFromTraceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddTaskFromTraceResponseBuilder {
    dataset: Option<AddTaskFromTraceResponseDataset>,
    membership: Option<AddTaskFromTraceResponseMembership>,
}

impl AddTaskFromTraceResponseBuilder {
    pub fn dataset(mut self, value: AddTaskFromTraceResponseDataset) -> Self {
        self.dataset = Some(value);
        self
    }

    pub fn membership(mut self, value: AddTaskFromTraceResponseMembership) -> Self {
        self.membership = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddTaskFromTraceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dataset`](AddTaskFromTraceResponseBuilder::dataset)
    /// - [`membership`](AddTaskFromTraceResponseBuilder::membership)
    pub fn build(self) -> Result<AddTaskFromTraceResponse, BuildError> {
        Ok(AddTaskFromTraceResponse {
            dataset: self
                .dataset
                .ok_or_else(|| BuildError::missing_field("dataset"))?,
            membership: self
                .membership
                .ok_or_else(|| BuildError::missing_field("membership"))?,
        })
    }
}
