pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentVersionSummaryArtifactWorkflowGraphPreviewEdgesItem {
    #[serde(default)]
    pub from: String,
    /// Optional short label rendered along the edge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default)]
    pub to: String,
}

impl AgentVersionSummaryArtifactWorkflowGraphPreviewEdgesItem {
    pub fn builder() -> AgentVersionSummaryArtifactWorkflowGraphPreviewEdgesItemBuilder {
        <AgentVersionSummaryArtifactWorkflowGraphPreviewEdgesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentVersionSummaryArtifactWorkflowGraphPreviewEdgesItemBuilder {
    from: Option<String>,
    label: Option<String>,
    to: Option<String>,
}

impl AgentVersionSummaryArtifactWorkflowGraphPreviewEdgesItemBuilder {
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

    /// Consumes the builder and constructs a [`AgentVersionSummaryArtifactWorkflowGraphPreviewEdgesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from`](AgentVersionSummaryArtifactWorkflowGraphPreviewEdgesItemBuilder::from)
    /// - [`to`](AgentVersionSummaryArtifactWorkflowGraphPreviewEdgesItemBuilder::to)
    pub fn build(
        self,
    ) -> Result<AgentVersionSummaryArtifactWorkflowGraphPreviewEdgesItem, BuildError> {
        Ok(AgentVersionSummaryArtifactWorkflowGraphPreviewEdgesItem {
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            label: self.label,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
        })
    }
}
