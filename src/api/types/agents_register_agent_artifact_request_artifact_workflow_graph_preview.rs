pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RegisterAgentArtifactRequestArtifactWorkflowGraphPreview {
    #[serde(default)]
    pub edges: Vec<RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewEdgesItem>,
    #[serde(default)]
    pub nodes: Vec<RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewNodesItem>,
}

impl RegisterAgentArtifactRequestArtifactWorkflowGraphPreview {
    pub fn builder() -> RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewBuilder {
        <RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewBuilder {
    edges: Option<Vec<RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewEdgesItem>>,
    nodes: Option<Vec<RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewNodesItem>>,
}

impl RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewBuilder {
    pub fn edges(
        mut self,
        value: Vec<RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewEdgesItem>,
    ) -> Self {
        self.edges = Some(value);
        self
    }

    pub fn nodes(
        mut self,
        value: Vec<RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewNodesItem>,
    ) -> Self {
        self.nodes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RegisterAgentArtifactRequestArtifactWorkflowGraphPreview`].
    /// This method will fail if any of the following fields are not set:
    /// - [`edges`](RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewBuilder::edges)
    /// - [`nodes`](RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewBuilder::nodes)
    pub fn build(
        self,
    ) -> Result<RegisterAgentArtifactRequestArtifactWorkflowGraphPreview, BuildError> {
        Ok(RegisterAgentArtifactRequestArtifactWorkflowGraphPreview {
            edges: self
                .edges
                .ok_or_else(|| BuildError::missing_field("edges"))?,
            nodes: self
                .nodes
                .ok_or_else(|| BuildError::missing_field("nodes"))?,
        })
    }
}
