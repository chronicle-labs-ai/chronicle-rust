pub use crate::prelude::*;

/// Compact preview of the output contract this artifact emits.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentSnapshotVersionsItemArtifactOutputContractPreview {
    /// Representative example payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,
    /// One-line shape summary.
    #[serde(rename = "schemaSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_summary: Option<String>,
}

impl AgentSnapshotVersionsItemArtifactOutputContractPreview {
    pub fn builder() -> AgentSnapshotVersionsItemArtifactOutputContractPreviewBuilder {
        <AgentSnapshotVersionsItemArtifactOutputContractPreviewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSnapshotVersionsItemArtifactOutputContractPreviewBuilder {
    example: Option<serde_json::Value>,
    schema_summary: Option<String>,
}

impl AgentSnapshotVersionsItemArtifactOutputContractPreviewBuilder {
    pub fn example(mut self, value: serde_json::Value) -> Self {
        self.example = Some(value);
        self
    }

    pub fn schema_summary(mut self, value: impl Into<String>) -> Self {
        self.schema_summary = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentSnapshotVersionsItemArtifactOutputContractPreview`].
    pub fn build(
        self,
    ) -> Result<AgentSnapshotVersionsItemArtifactOutputContractPreview, BuildError> {
        Ok(AgentSnapshotVersionsItemArtifactOutputContractPreview {
            example: self.example,
            schema_summary: self.schema_summary,
        })
    }
}
