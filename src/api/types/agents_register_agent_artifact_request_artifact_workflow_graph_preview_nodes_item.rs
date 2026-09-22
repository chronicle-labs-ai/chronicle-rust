pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewNodesItem {
    #[serde(default)]
    pub id: String,
    pub kind: RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewNodesItemKind,
    #[serde(default)]
    pub label: String,
    /// Optional tool name reference for tool nodes.
    #[serde(rename = "toolName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
}

impl RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewNodesItem {
    pub fn builder() -> RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewNodesItemBuilder {
        <RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewNodesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewNodesItemBuilder {
    id: Option<String>,
    kind: Option<RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewNodesItemKind>,
    label: Option<String>,
    tool_name: Option<String>,
}

impl RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewNodesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(
        mut self,
        value: RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewNodesItemKind,
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

    /// Consumes the builder and constructs a [`RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewNodesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewNodesItemBuilder::id)
    /// - [`kind`](RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewNodesItemBuilder::kind)
    /// - [`label`](RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewNodesItemBuilder::label)
    pub fn build(
        self,
    ) -> Result<RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewNodesItem, BuildError> {
        Ok(
            RegisterAgentArtifactRequestArtifactWorkflowGraphPreviewNodesItem {
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
