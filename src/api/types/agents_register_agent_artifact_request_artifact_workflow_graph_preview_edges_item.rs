pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewEdgesItem {
    #[serde(default)]
    pub from: String,
    /// Optional short label rendered along the edge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default)]
    pub to: String,
}

impl RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewEdgesItem {
    pub fn builder() -> RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewEdgesItemBuilder {
        <RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewEdgesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewEdgesItemBuilder {
    from: Option<String>,
    label: Option<String>,
    to: Option<String>,
}

impl RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewEdgesItemBuilder {
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewEdgesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from`](RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewEdgesItemBuilder::from)
    /// - [`to`](RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewEdgesItemBuilder::to)
    pub fn build(
        self,
    ) -> Result<RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewEdgesItem, BuildError> {
        Ok(
            RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewEdgesItem {
                from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
                label: self.label,
                to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
            },
        )
    }
}
