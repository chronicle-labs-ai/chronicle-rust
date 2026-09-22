pub use crate::prelude::*;

/// Mutable, human-authored metadata attached to a logical Agent identity. Artifact configuration remains immutable inside `AgentRegistryVersionRecord`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RegisterAgentArtifactRequestMetadata {
    #[serde(rename = "capabilityTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "personaSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persona_summary: Option<String>,
    #[serde(rename = "playgroundUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playground_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(rename = "runbookUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runbook_url: Option<String>,
}

impl RegisterAgentArtifactRequestMetadata {
    pub fn builder() -> RegisterAgentArtifactRequestMetadataBuilder {
        <RegisterAgentArtifactRequestMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterAgentArtifactRequestMetadataBuilder {
    capability_tags: Option<Vec<String>>,
    category: Option<String>,
    environment: Option<String>,
    owner: Option<String>,
    persona_summary: Option<String>,
    playground_url: Option<String>,
    purpose: Option<String>,
    runbook_url: Option<String>,
}

impl RegisterAgentArtifactRequestMetadataBuilder {
    pub fn capability_tags(mut self, value: Vec<String>) -> Self {
        self.capability_tags = Some(value);
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn owner(mut self, value: impl Into<String>) -> Self {
        self.owner = Some(value.into());
        self
    }

    pub fn persona_summary(mut self, value: impl Into<String>) -> Self {
        self.persona_summary = Some(value.into());
        self
    }

    pub fn playground_url(mut self, value: impl Into<String>) -> Self {
        self.playground_url = Some(value.into());
        self
    }

    pub fn purpose(mut self, value: impl Into<String>) -> Self {
        self.purpose = Some(value.into());
        self
    }

    pub fn runbook_url(mut self, value: impl Into<String>) -> Self {
        self.runbook_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RegisterAgentArtifactRequestMetadata`].
    pub fn build(self) -> Result<RegisterAgentArtifactRequestMetadata, BuildError> {
        Ok(RegisterAgentArtifactRequestMetadata {
            capability_tags: self.capability_tags,
            category: self.category,
            environment: self.environment,
            owner: self.owner,
            persona_summary: self.persona_summary,
            playground_url: self.playground_url,
            purpose: self.purpose,
            runbook_url: self.runbook_url,
        })
    }
}
