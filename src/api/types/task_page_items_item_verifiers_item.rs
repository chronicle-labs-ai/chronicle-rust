pub use crate::prelude::*;

/// One verifier bound to a task: a scorer from the tenant library plus the weight and pass threshold it carries for this task. Order is the position in the task's verifier list.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskPageItemsItemVerifiersItem {
    #[serde(rename = "passThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pass_threshold: Option<f64>,
    #[serde(rename = "scorerId")]
    #[serde(default)]
    pub scorer_id: String,
    /// Grader weight bucket — `low | med | high` matches the segmented control in the GraderBuilder tray.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<TaskPageItemsItemVerifiersItemWeight>,
}

impl TaskPageItemsItemVerifiersItem {
    pub fn builder() -> TaskPageItemsItemVerifiersItemBuilder {
        <TaskPageItemsItemVerifiersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskPageItemsItemVerifiersItemBuilder {
    pass_threshold: Option<f64>,
    scorer_id: Option<String>,
    weight: Option<TaskPageItemsItemVerifiersItemWeight>,
}

impl TaskPageItemsItemVerifiersItemBuilder {
    pub fn pass_threshold(mut self, value: f64) -> Self {
        self.pass_threshold = Some(value);
        self
    }

    pub fn scorer_id(mut self, value: impl Into<String>) -> Self {
        self.scorer_id = Some(value.into());
        self
    }

    pub fn weight(mut self, value: TaskPageItemsItemVerifiersItemWeight) -> Self {
        self.weight = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskPageItemsItemVerifiersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scorer_id`](TaskPageItemsItemVerifiersItemBuilder::scorer_id)
    pub fn build(self) -> Result<TaskPageItemsItemVerifiersItem, BuildError> {
        Ok(TaskPageItemsItemVerifiersItem {
            pass_threshold: self.pass_threshold,
            scorer_id: self
                .scorer_id
                .ok_or_else(|| BuildError::missing_field("scorer_id"))?,
            weight: self.weight,
        })
    }
}
