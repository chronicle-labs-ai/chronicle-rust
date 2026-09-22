pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentSnapshotVersionsItemArtifactKnowledgeSourcesItem {
    /// Optional jump-out link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(default)]
    pub id: String,
    pub kind: AgentSnapshotVersionsItemArtifactKnowledgeSourcesItemKind,
    #[serde(default)]
    pub label: String,
    /// Optional human-readable size hint, e.g. "12.4k docs".
    #[serde(rename = "sizeLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_label: Option<String>,
}

impl AgentSnapshotVersionsItemArtifactKnowledgeSourcesItem {
    pub fn builder() -> AgentSnapshotVersionsItemArtifactKnowledgeSourcesItemBuilder {
        <AgentSnapshotVersionsItemArtifactKnowledgeSourcesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSnapshotVersionsItemArtifactKnowledgeSourcesItemBuilder {
    href: Option<String>,
    id: Option<String>,
    kind: Option<AgentSnapshotVersionsItemArtifactKnowledgeSourcesItemKind>,
    label: Option<String>,
    size_label: Option<String>,
}

impl AgentSnapshotVersionsItemArtifactKnowledgeSourcesItemBuilder {
    pub fn href(mut self, value: impl Into<String>) -> Self {
        self.href = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(
        mut self,
        value: AgentSnapshotVersionsItemArtifactKnowledgeSourcesItemKind,
    ) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn size_label(mut self, value: impl Into<String>) -> Self {
        self.size_label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentSnapshotVersionsItemArtifactKnowledgeSourcesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AgentSnapshotVersionsItemArtifactKnowledgeSourcesItemBuilder::id)
    /// - [`kind`](AgentSnapshotVersionsItemArtifactKnowledgeSourcesItemBuilder::kind)
    /// - [`label`](AgentSnapshotVersionsItemArtifactKnowledgeSourcesItemBuilder::label)
    pub fn build(
        self,
    ) -> Result<AgentSnapshotVersionsItemArtifactKnowledgeSourcesItem, BuildError> {
        Ok(AgentSnapshotVersionsItemArtifactKnowledgeSourcesItem {
            href: self.href,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            size_label: self.size_label,
        })
    }
}
