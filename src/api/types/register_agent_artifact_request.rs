pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RegisterAgentArtifactRequest {
    pub artifact: RegisterAgentArtifactRequestArtifact,
    /// Mutable, human-authored metadata attached to a logical Agent identity. Artifact configuration remains immutable inside `AgentRegistryVersionRecord`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<RegisterAgentArtifactRequestMetadata>,
    /// Defaults to `current`. Registering a new current version atomically demotes the previous current version to stable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RegisterAgentArtifactRequestStatus>,
}

impl RegisterAgentArtifactRequest {
    pub fn builder() -> RegisterAgentArtifactRequestBuilder {
        <RegisterAgentArtifactRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterAgentArtifactRequestBuilder {
    artifact: Option<RegisterAgentArtifactRequestArtifact>,
    metadata: Option<RegisterAgentArtifactRequestMetadata>,
    status: Option<RegisterAgentArtifactRequestStatus>,
}

impl RegisterAgentArtifactRequestBuilder {
    pub fn artifact(mut self, value: RegisterAgentArtifactRequestArtifact) -> Self {
        self.artifact = Some(value);
        self
    }

    pub fn metadata(mut self, value: RegisterAgentArtifactRequestMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn status(mut self, value: RegisterAgentArtifactRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RegisterAgentArtifactRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`artifact`](RegisterAgentArtifactRequestBuilder::artifact)
    pub fn build(self) -> Result<RegisterAgentArtifactRequest, BuildError> {
        Ok(RegisterAgentArtifactRequest {
            artifact: self
                .artifact
                .ok_or_else(|| BuildError::missing_field("artifact"))?,
            metadata: self.metadata,
            status: self.status,
        })
    }
}
