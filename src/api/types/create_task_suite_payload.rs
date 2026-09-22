pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateTaskSuitePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    /// Intended use of a dataset — drives the colored badge on the picker and lets apps route additions to the right backend (eval suite, training set, replay corpus, manual review queue).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<CreateTaskSuitePayloadPurpose>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl CreateTaskSuitePayload {
    pub fn builder() -> CreateTaskSuitePayloadBuilder {
        <CreateTaskSuitePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTaskSuitePayloadBuilder {
    description: Option<String>,
    name: Option<String>,
    purpose: Option<CreateTaskSuitePayloadPurpose>,
    tags: Option<Vec<String>>,
}

impl CreateTaskSuitePayloadBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn purpose(mut self, value: CreateTaskSuitePayloadPurpose) -> Self {
        self.purpose = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateTaskSuitePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateTaskSuitePayloadBuilder::name)
    pub fn build(self) -> Result<CreateTaskSuitePayload, BuildError> {
        Ok(CreateTaskSuitePayload {
            description: self.description,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            purpose: self.purpose,
            tags: self.tags,
        })
    }
}
