pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BacktestsAvailabilityAgentsItemModel {
    #[serde(default)]
    pub label: String,
    #[serde(rename = "modelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}

impl BacktestsAvailabilityAgentsItemModel {
    pub fn builder() -> BacktestsAvailabilityAgentsItemModelBuilder {
        <BacktestsAvailabilityAgentsItemModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BacktestsAvailabilityAgentsItemModelBuilder {
    label: Option<String>,
    model_id: Option<String>,
    provider: Option<String>,
}

impl BacktestsAvailabilityAgentsItemModelBuilder {
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

    /// Consumes the builder and constructs a [`BacktestsAvailabilityAgentsItemModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](BacktestsAvailabilityAgentsItemModelBuilder::label)
    pub fn build(self) -> Result<BacktestsAvailabilityAgentsItemModel, BuildError> {
        Ok(BacktestsAvailabilityAgentsItemModel {
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            model_id: self.model_id,
            provider: self.provider,
        })
    }
}
