pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateTaskSuiteWithTraceRequestDataset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    /// Intended use of a dataset — drives the colored badge on the picker and lets apps route additions to the right backend (eval suite, training set, replay corpus, manual review queue).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<CreateTaskSuiteWithTraceRequestDatasetPurpose>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl CreateTaskSuiteWithTraceRequestDataset {
    pub fn builder() -> CreateTaskSuiteWithTraceRequestDatasetBuilder {
        <CreateTaskSuiteWithTraceRequestDatasetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTaskSuiteWithTraceRequestDatasetBuilder {
    description: Option<String>,
    name: Option<String>,
    purpose: Option<CreateTaskSuiteWithTraceRequestDatasetPurpose>,
    tags: Option<Vec<String>>,
}

impl CreateTaskSuiteWithTraceRequestDatasetBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn purpose(mut self, value: CreateTaskSuiteWithTraceRequestDatasetPurpose) -> Self {
        self.purpose = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateTaskSuiteWithTraceRequestDataset`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateTaskSuiteWithTraceRequestDatasetBuilder::name)
    pub fn build(self) -> Result<CreateTaskSuiteWithTraceRequestDataset, BuildError> {
        Ok(CreateTaskSuiteWithTraceRequestDataset {
            description: self.description,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            purpose: self.purpose,
            tags: self.tags,
        })
    }
}
