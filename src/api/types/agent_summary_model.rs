pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentSummaryModel {
    #[serde(default)]
    pub label: String,
    #[serde(rename = "modelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}

impl AgentSummaryModel {
    pub fn builder() -> AgentSummaryModelBuilder {
        <AgentSummaryModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSummaryModelBuilder {
    label: Option<String>,
    model_id: Option<String>,
    provider: Option<String>,
}

impl AgentSummaryModelBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentSummaryModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](AgentSummaryModelBuilder::label)
    pub fn build(self) -> Result<AgentSummaryModel, BuildError> {
        Ok(AgentSummaryModel {
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            model_id: self.model_id,
            provider: self.provider,
        })
    }
}
