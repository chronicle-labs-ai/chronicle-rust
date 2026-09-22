pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentSnapshotVersionsItemArtifactModel {
    #[serde(default)]
    pub label: String,
    #[serde(rename = "modelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}

impl AgentSnapshotVersionsItemArtifactModel {
    pub fn builder() -> AgentSnapshotVersionsItemArtifactModelBuilder {
        <AgentSnapshotVersionsItemArtifactModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSnapshotVersionsItemArtifactModelBuilder {
    label: Option<String>,
    model_id: Option<String>,
    provider: Option<String>,
}

impl AgentSnapshotVersionsItemArtifactModelBuilder {
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

    /// Consumes the builder and constructs a [`AgentSnapshotVersionsItemArtifactModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](AgentSnapshotVersionsItemArtifactModelBuilder::label)
    pub fn build(self) -> Result<AgentSnapshotVersionsItemArtifactModel, BuildError> {
        Ok(AgentSnapshotVersionsItemArtifactModel {
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            model_id: self.model_id,
            provider: self.provider,
        })
    }
}
