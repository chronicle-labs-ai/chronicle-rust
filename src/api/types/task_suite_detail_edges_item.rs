pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskSuiteDetailEdgesItem {
    #[serde(rename = "fromTraceId")]
    #[serde(default)]
    pub from_trace_id: String,
    #[serde(rename = "toTraceId")]
    #[serde(default)]
    pub to_trace_id: String,
    /// Similarity weight in [0..1].
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub weight: f64,
}

impl TaskSuiteDetailEdgesItem {
    pub fn builder() -> TaskSuiteDetailEdgesItemBuilder {
        <TaskSuiteDetailEdgesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskSuiteDetailEdgesItemBuilder {
    from_trace_id: Option<String>,
    to_trace_id: Option<String>,
    weight: Option<f64>,
}

impl TaskSuiteDetailEdgesItemBuilder {
    pub fn from_trace_id(mut self, value: impl Into<String>) -> Self {
        self.from_trace_id = Some(value.into());
        self
    }

    pub fn to_trace_id(mut self, value: impl Into<String>) -> Self {
        self.to_trace_id = Some(value.into());
        self
    }

    pub fn weight(mut self, value: f64) -> Self {
        self.weight = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskSuiteDetailEdgesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_trace_id`](TaskSuiteDetailEdgesItemBuilder::from_trace_id)
    /// - [`to_trace_id`](TaskSuiteDetailEdgesItemBuilder::to_trace_id)
    /// - [`weight`](TaskSuiteDetailEdgesItemBuilder::weight)
    pub fn build(self) -> Result<TaskSuiteDetailEdgesItem, BuildError> {
        Ok(TaskSuiteDetailEdgesItem {
            from_trace_id: self
                .from_trace_id
                .ok_or_else(|| BuildError::missing_field("from_trace_id"))?,
            to_trace_id: self
                .to_trace_id
                .ok_or_else(|| BuildError::missing_field("to_trace_id"))?,
            weight: self
                .weight
                .ok_or_else(|| BuildError::missing_field("weight"))?,
        })
    }
}
