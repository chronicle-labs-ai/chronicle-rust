pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentSnapshotHashIndexItem {
    /// Owning agent's display name — lets hash-index results deep-link straight to the agent detail surface.
    #[serde(rename = "agentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    #[serde(rename = "artifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<String>,
    /// Framework label. Multi-word kebab-case to match the existing TS union (`vercel-ai-sdk`, `openai-agents-python`, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<AgentSnapshotHashIndexItemFramework>,
    #[serde(default)]
    pub hash: String,
    /// The 13 hash domains the wrapper tracks. The first eight describe the artifact (config-time); the last five describe a run (observed at call-time).
    pub kind: AgentSnapshotHashIndexItemKind,
    #[serde(rename = "observedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub observed_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub path: String,
    /// Stringified preview of the value rendered next to the hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
    #[serde(rename = "runId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

impl AgentSnapshotHashIndexItem {
    pub fn builder() -> AgentSnapshotHashIndexItemBuilder {
        <AgentSnapshotHashIndexItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSnapshotHashIndexItemBuilder {
    agent_name: Option<String>,
    artifact_id: Option<String>,
    framework: Option<AgentSnapshotHashIndexItemFramework>,
    hash: Option<String>,
    kind: Option<AgentSnapshotHashIndexItemKind>,
    observed_at: Option<DateTime<FixedOffset>>,
    path: Option<String>,
    preview: Option<String>,
    run_id: Option<String>,
}

impl AgentSnapshotHashIndexItemBuilder {
    pub fn agent_name(mut self, value: impl Into<String>) -> Self {
        self.agent_name = Some(value.into());
        self
    }

    pub fn artifact_id(mut self, value: impl Into<String>) -> Self {
        self.artifact_id = Some(value.into());
        self
    }

    pub fn framework(mut self, value: AgentSnapshotHashIndexItemFramework) -> Self {
        self.framework = Some(value);
        self
    }

    pub fn hash(mut self, value: impl Into<String>) -> Self {
        self.hash = Some(value.into());
        self
    }

    pub fn kind(mut self, value: AgentSnapshotHashIndexItemKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn observed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.observed_at = Some(value);
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn preview(mut self, value: impl Into<String>) -> Self {
        self.preview = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentSnapshotHashIndexItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`hash`](AgentSnapshotHashIndexItemBuilder::hash)
    /// - [`kind`](AgentSnapshotHashIndexItemBuilder::kind)
    /// - [`observed_at`](AgentSnapshotHashIndexItemBuilder::observed_at)
    /// - [`path`](AgentSnapshotHashIndexItemBuilder::path)
    pub fn build(self) -> Result<AgentSnapshotHashIndexItem, BuildError> {
        Ok(AgentSnapshotHashIndexItem {
            agent_name: self.agent_name,
            artifact_id: self.artifact_id,
            framework: self.framework,
            hash: self.hash.ok_or_else(|| BuildError::missing_field("hash"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            observed_at: self
                .observed_at
                .ok_or_else(|| BuildError::missing_field("observed_at"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            preview: self.preview,
            run_id: self.run_id,
        })
    }
}
