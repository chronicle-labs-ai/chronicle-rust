pub use crate::prelude::*;

/// One verifier bound to a task: a scorer from the tenant library plus the weight and pass threshold it carries for this task. Order is the position in the task's verifier list.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SetTaskVerifiersRequestVerifiersItem {
    #[serde(rename = "passThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pass_threshold: Option<f64>,
    #[serde(rename = "scorerId")]
    #[serde(default)]
    pub scorer_id: String,
    /// Grader weight bucket — `low | med | high` matches the segmented control in the GraderBuilder tray.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<SetTaskVerifiersRequestVerifiersItemWeight>,
}

impl SetTaskVerifiersRequestVerifiersItem {
    pub fn builder() -> SetTaskVerifiersRequestVerifiersItemBuilder {
        <SetTaskVerifiersRequestVerifiersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetTaskVerifiersRequestVerifiersItemBuilder {
    pass_threshold: Option<f64>,
    scorer_id: Option<String>,
    weight: Option<SetTaskVerifiersRequestVerifiersItemWeight>,
}

impl SetTaskVerifiersRequestVerifiersItemBuilder {
    pub fn pass_threshold(mut self, value: f64) -> Self {
        self.pass_threshold = Some(value);
        self
    }

    pub fn scorer_id(mut self, value: impl Into<String>) -> Self {
        self.scorer_id = Some(value.into());
        self
    }

    pub fn weight(mut self, value: SetTaskVerifiersRequestVerifiersItemWeight) -> Self {
        self.weight = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetTaskVerifiersRequestVerifiersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scorer_id`](SetTaskVerifiersRequestVerifiersItemBuilder::scorer_id)
    pub fn build(self) -> Result<SetTaskVerifiersRequestVerifiersItem, BuildError> {
        Ok(SetTaskVerifiersRequestVerifiersItem {
            pass_threshold: self.pass_threshold,
            scorer_id: self
                .scorer_id
                .ok_or_else(|| BuildError::missing_field("scorer_id"))?,
            weight: self.weight,
        })
    }
}
