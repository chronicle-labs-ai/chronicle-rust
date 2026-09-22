pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentSnapshotVersionsItemArtifactProvenance {
    #[serde(rename = "aiSdkVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_sdk_version: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "dependencyLockHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency_lock_hash: Option<String>,
    #[serde(rename = "frameworkVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_version: Option<String>,
    #[serde(rename = "gitSha")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_sha: Option<String>,
    #[serde(rename = "publishedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_by: Option<String>,
}

impl AgentSnapshotVersionsItemArtifactProvenance {
    pub fn builder() -> AgentSnapshotVersionsItemArtifactProvenanceBuilder {
        <AgentSnapshotVersionsItemArtifactProvenanceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSnapshotVersionsItemArtifactProvenanceBuilder {
    ai_sdk_version: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    dependency_lock_hash: Option<String>,
    framework_version: Option<String>,
    git_sha: Option<String>,
    published_by: Option<String>,
}

impl AgentSnapshotVersionsItemArtifactProvenanceBuilder {
    pub fn ai_sdk_version(mut self, value: impl Into<String>) -> Self {
        self.ai_sdk_version = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn dependency_lock_hash(mut self, value: impl Into<String>) -> Self {
        self.dependency_lock_hash = Some(value.into());
        self
    }

    pub fn framework_version(mut self, value: impl Into<String>) -> Self {
        self.framework_version = Some(value.into());
        self
    }

    pub fn git_sha(mut self, value: impl Into<String>) -> Self {
        self.git_sha = Some(value.into());
        self
    }

    pub fn published_by(mut self, value: impl Into<String>) -> Self {
        self.published_by = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentSnapshotVersionsItemArtifactProvenance`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](AgentSnapshotVersionsItemArtifactProvenanceBuilder::created_at)
    pub fn build(self) -> Result<AgentSnapshotVersionsItemArtifactProvenance, BuildError> {
        Ok(AgentSnapshotVersionsItemArtifactProvenance {
            ai_sdk_version: self.ai_sdk_version,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            dependency_lock_hash: self.dependency_lock_hash,
            framework_version: self.framework_version,
            git_sha: self.git_sha,
            published_by: self.published_by,
        })
    }
}
