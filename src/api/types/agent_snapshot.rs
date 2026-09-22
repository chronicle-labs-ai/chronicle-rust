pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentSnapshot {
    #[serde(rename = "hashIndex")]
    #[serde(default)]
    pub hash_index: Vec<AgentSnapshotHashIndexItem>,
    #[serde(default)]
    pub runs: Vec<AgentSnapshotRunsItem>,
    pub summary: AgentSnapshotSummary,
    #[serde(default)]
    pub versions: Vec<AgentSnapshotVersionsItem>,
}

impl AgentSnapshot {
    pub fn builder() -> AgentSnapshotBuilder {
        <AgentSnapshotBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSnapshotBuilder {
    hash_index: Option<Vec<AgentSnapshotHashIndexItem>>,
    runs: Option<Vec<AgentSnapshotRunsItem>>,
    summary: Option<AgentSnapshotSummary>,
    versions: Option<Vec<AgentSnapshotVersionsItem>>,
}

impl AgentSnapshotBuilder {
    pub fn hash_index(mut self, value: Vec<AgentSnapshotHashIndexItem>) -> Self {
        self.hash_index = Some(value);
        self
    }

    pub fn runs(mut self, value: Vec<AgentSnapshotRunsItem>) -> Self {
        self.runs = Some(value);
        self
    }

    pub fn summary(mut self, value: AgentSnapshotSummary) -> Self {
        self.summary = Some(value);
        self
    }

    pub fn versions(mut self, value: Vec<AgentSnapshotVersionsItem>) -> Self {
        self.versions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentSnapshot`].
    /// This method will fail if any of the following fields are not set:
    /// - [`hash_index`](AgentSnapshotBuilder::hash_index)
    /// - [`runs`](AgentSnapshotBuilder::runs)
    /// - [`summary`](AgentSnapshotBuilder::summary)
    /// - [`versions`](AgentSnapshotBuilder::versions)
    pub fn build(self) -> Result<AgentSnapshot, BuildError> {
        Ok(AgentSnapshot {
            hash_index: self
                .hash_index
                .ok_or_else(|| BuildError::missing_field("hash_index"))?,
            runs: self.runs.ok_or_else(|| BuildError::missing_field("runs"))?,
            summary: self
                .summary
                .ok_or_else(|| BuildError::missing_field("summary"))?,
            versions: self
                .versions
                .ok_or_else(|| BuildError::missing_field("versions"))?,
        })
    }
}
