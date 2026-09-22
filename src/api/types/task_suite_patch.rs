pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskSuitePatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Intended use of a dataset — drives the colored badge on the picker and lets apps route additions to the right backend (eval suite, training set, replay corpus, manual review queue).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<TaskSuitePatchPurpose>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl TaskSuitePatch {
    pub fn builder() -> TaskSuitePatchBuilder {
        <TaskSuitePatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskSuitePatchBuilder {
    description: Option<String>,
    name: Option<String>,
    purpose: Option<TaskSuitePatchPurpose>,
    tags: Option<Vec<String>>,
}

impl TaskSuitePatchBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn purpose(mut self, value: TaskSuitePatchPurpose) -> Self {
        self.purpose = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskSuitePatch`].
    pub fn build(self) -> Result<TaskSuitePatch, BuildError> {
        Ok(TaskSuitePatch {
            description: self.description,
            name: self.name,
            purpose: self.purpose,
            tags: self.tags,
        })
    }
}
