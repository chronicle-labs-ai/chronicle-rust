pub use crate::prelude::*;

/// Compact preview of the input contract this artifact expects.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RegisterAgentArtifactRequestArtifactInputContractPreview {
    /// Representative example payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,
    /// One-line shape summary.
    #[serde(rename = "schemaSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_summary: Option<String>,
}

impl RegisterAgentArtifactRequestArtifactInputContractPreview {
    pub fn builder() -> RegisterAgentArtifactRequestArtifactInputContractPreviewBuilder {
        <RegisterAgentArtifactRequestArtifactInputContractPreviewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterAgentArtifactRequestArtifactInputContractPreviewBuilder {
    example: Option<serde_json::Value>,
    schema_summary: Option<String>,
}

impl RegisterAgentArtifactRequestArtifactInputContractPreviewBuilder {
    pub fn example(mut self, value: serde_json::Value) -> Self {
        self.example = Some(value);
        self
    }

    pub fn schema_summary(mut self, value: impl Into<String>) -> Self {
        self.schema_summary = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RegisterAgentArtifactRequestArtifactInputContractPreview`].
    pub fn build(
        self,
    ) -> Result<RegisterAgentArtifactRequestArtifactInputContractPreview, BuildError> {
        Ok(RegisterAgentArtifactRequestArtifactInputContractPreview {
            example: self.example,
            schema_summary: self.schema_summary,
        })
    }
}
