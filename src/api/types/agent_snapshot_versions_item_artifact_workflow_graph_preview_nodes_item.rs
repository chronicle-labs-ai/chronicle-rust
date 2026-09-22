pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewNodesItem {
    #[serde(default)]
    pub id: String,
    pub kind: AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewNodesItemKind,
    #[serde(default)]
    pub label: String,
    /// Optional tool name reference for tool nodes.
    #[serde(rename = "toolName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
}

impl AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewNodesItem {
    pub fn builder() -> AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewNodesItemBuilder {
        <AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewNodesItemBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewNodesItemBuilder {
    id: Option<String>,
    kind: Option<AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewNodesItemKind>,
    label: Option<String>,
    tool_name: Option<String>,
}

impl AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewNodesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(
        mut self,
        value: AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewNodesItemKind,
    ) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn tool_name(mut self, value: impl Into<String>) -> Self {
        self.tool_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewNodesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewNodesItemBuilder::id)
    /// - [`kind`](AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewNodesItemBuilder::kind)
    /// - [`label`](AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewNodesItemBuilder::label)
    pub fn build(
        self,
    ) -> Result<AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewNodesItem, BuildError> {
        Ok(
            AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewNodesItem {
                id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
                kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
                label: self
                    .label
                    .ok_or_else(|| BuildError::missing_field("label"))?,
                tool_name: self.tool_name,
            },
        )
    }
}
