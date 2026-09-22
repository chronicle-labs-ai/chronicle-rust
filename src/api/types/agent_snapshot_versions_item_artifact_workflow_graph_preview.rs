pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentSnapshotVersionsItemArtifactWorkflowGraphPreview {
    #[serde(default)]
    pub edges: Vec<AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewEdgesItem>,
    #[serde(default)]
    pub nodes: Vec<AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewNodesItem>,
}

impl AgentSnapshotVersionsItemArtifactWorkflowGraphPreview {
    pub fn builder() -> AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewBuilder {
        <AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewBuilder {
    edges: Option<Vec<AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewEdgesItem>>,
    nodes: Option<Vec<AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewNodesItem>>,
}

impl AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewBuilder {
    pub fn edges(
        mut self,
        value: Vec<AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewEdgesItem>,
    ) -> Self {
        self.edges = Some(value);
        self
    }

    pub fn nodes(
        mut self,
        value: Vec<AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewNodesItem>,
    ) -> Self {
        self.nodes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentSnapshotVersionsItemArtifactWorkflowGraphPreview`].
    /// This method will fail if any of the following fields are not set:
    /// - [`edges`](AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewBuilder::edges)
    /// - [`nodes`](AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewBuilder::nodes)
    pub fn build(
        self,
    ) -> Result<AgentSnapshotVersionsItemArtifactWorkflowGraphPreview, BuildError> {
        Ok(AgentSnapshotVersionsItemArtifactWorkflowGraphPreview {
            edges: self
                .edges
                .ok_or_else(|| BuildError::missing_field("edges"))?,
            nodes: self
                .nodes
                .ok_or_else(|| BuildError::missing_field("nodes"))?,
        })
    }
}
