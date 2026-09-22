pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentVersionSummaryArtifactWorkflowGraphPreview {
    #[serde(default)]
    pub edges: Vec<AgentVersionSummaryArtifactWorkflowGraphPreviewEdgesItem>,
    #[serde(default)]
    pub nodes: Vec<AgentVersionSummaryArtifactWorkflowGraphPreviewNodesItem>,
}

impl AgentVersionSummaryArtifactWorkflowGraphPreview {
    pub fn builder() -> AgentVersionSummaryArtifactWorkflowGraphPreviewBuilder {
        <AgentVersionSummaryArtifactWorkflowGraphPreviewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentVersionSummaryArtifactWorkflowGraphPreviewBuilder {
    edges: Option<Vec<AgentVersionSummaryArtifactWorkflowGraphPreviewEdgesItem>>,
    nodes: Option<Vec<AgentVersionSummaryArtifactWorkflowGraphPreviewNodesItem>>,
}

impl AgentVersionSummaryArtifactWorkflowGraphPreviewBuilder {
    pub fn edges(
        mut self,
        value: Vec<AgentVersionSummaryArtifactWorkflowGraphPreviewEdgesItem>,
    ) -> Self {
        self.edges = Some(value);
        self
    }

    pub fn nodes(
        mut self,
        value: Vec<AgentVersionSummaryArtifactWorkflowGraphPreviewNodesItem>,
    ) -> Self {
        self.nodes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentVersionSummaryArtifactWorkflowGraphPreview`].
    /// This method will fail if any of the following fields are not set:
    /// - [`edges`](AgentVersionSummaryArtifactWorkflowGraphPreviewBuilder::edges)
    /// - [`nodes`](AgentVersionSummaryArtifactWorkflowGraphPreviewBuilder::nodes)
    pub fn build(self) -> Result<AgentVersionSummaryArtifactWorkflowGraphPreview, BuildError> {
        Ok(AgentVersionSummaryArtifactWorkflowGraphPreview {
            edges: self
                .edges
                .ok_or_else(|| BuildError::missing_field("edges"))?,
            nodes: self
                .nodes
                .ok_or_else(|| BuildError::missing_field("nodes"))?,
        })
    }
}
