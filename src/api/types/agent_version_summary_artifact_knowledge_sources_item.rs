pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentVersionSummaryArtifactKnowledgeSourcesItem {
    /// Optional jump-out link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(default)]
    pub id: String,
    pub kind: AgentVersionSummaryArtifactKnowledgeSourcesItemKind,
    #[serde(default)]
    pub label: String,
    /// Optional human-readable size hint, e.g. "12.4k docs".
    #[serde(rename = "sizeLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_label: Option<String>,
}

impl AgentVersionSummaryArtifactKnowledgeSourcesItem {
    pub fn builder() -> AgentVersionSummaryArtifactKnowledgeSourcesItemBuilder {
        <AgentVersionSummaryArtifactKnowledgeSourcesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentVersionSummaryArtifactKnowledgeSourcesItemBuilder {
    href: Option<String>,
    id: Option<String>,
    kind: Option<AgentVersionSummaryArtifactKnowledgeSourcesItemKind>,
    label: Option<String>,
    size_label: Option<String>,
}

impl AgentVersionSummaryArtifactKnowledgeSourcesItemBuilder {
    pub fn href(mut self, value: impl Into<String>) -> Self {
        self.href = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: AgentVersionSummaryArtifactKnowledgeSourcesItemKind) -> Self {
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

    /// Consumes the builder and constructs a [`AgentVersionSummaryArtifactKnowledgeSourcesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AgentVersionSummaryArtifactKnowledgeSourcesItemBuilder::id)
    /// - [`kind`](AgentVersionSummaryArtifactKnowledgeSourcesItemBuilder::kind)
    /// - [`label`](AgentVersionSummaryArtifactKnowledgeSourcesItemBuilder::label)
    pub fn build(self) -> Result<AgentVersionSummaryArtifactKnowledgeSourcesItem, BuildError> {
        Ok(AgentVersionSummaryArtifactKnowledgeSourcesItem {
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
