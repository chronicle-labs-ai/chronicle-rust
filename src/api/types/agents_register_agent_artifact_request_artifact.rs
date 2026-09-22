pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RegisterAgentArtifactRequestArtifact {
    #[serde(rename = "artifactId")]
    #[serde(default)]
    pub artifact_id: String,
    #[serde(rename = "configHash")]
    #[serde(default)]
    pub config_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Framework label. Multi-word kebab-case to match the existing TS union (`vercel-ai-sdk`, `openai-agents-python`, etc.).
    pub framework: RegisterAgentArtifactRequestArtifactFramework,
    /// Compact preview of the input contract this artifact expects.
    #[serde(rename = "inputContractPreview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_contract_preview: Option<RegisterAgentArtifactRequestArtifactInputContractPreview>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(rename = "instructionsHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions_hash: Option<String>,
    #[serde(rename = "knowledgeSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_sources: Option<Vec<RegisterAgentArtifactRequestArtifactKnowledgeSourcesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub model: RegisterAgentArtifactRequestArtifactModel,
    #[serde(default)]
    pub name: String,
    /// Compact preview of the output contract this artifact emits.
    #[serde(rename = "outputContractPreview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_contract_preview: Option<RegisterAgentArtifactRequestArtifactOutputContractPreview>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<RegisterAgentArtifactRequestArtifactPolicy>,
    #[serde(default)]
    pub provenance: RegisterAgentArtifactRequestArtifactProvenance,
    #[serde(rename = "providerOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "providerOptionsHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options_hash: Option<String>,
    /// Schema marker — frontend code matches against the literal string `"agent-artifact-v1"` so newer payloads can co-exist when we evolve the shape.
    #[serde(rename = "schemaVersion")]
    #[serde(default)]
    pub schema_version: String,
    #[serde(default)]
    pub tools: Vec<RegisterAgentArtifactRequestArtifactToolsItem>,
    #[serde(default)]
    pub version: String,
    #[serde(rename = "workflowGraphPreview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_graph_preview: Option<RegisterAgentArtifactRequestArtifactWorkflowGraphPreview>,
}

impl RegisterAgentArtifactRequestArtifact {
    pub fn builder() -> RegisterAgentArtifactRequestArtifactBuilder {
        <RegisterAgentArtifactRequestArtifactBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterAgentArtifactRequestArtifactBuilder {
    artifact_id: Option<String>,
    config_hash: Option<String>,
    description: Option<String>,
    framework: Option<RegisterAgentArtifactRequestArtifactFramework>,
    input_contract_preview: Option<RegisterAgentArtifactRequestArtifactInputContractPreview>,
    instructions: Option<String>,
    instructions_hash: Option<String>,
    knowledge_sources: Option<Vec<RegisterAgentArtifactRequestArtifactKnowledgeSourcesItem>>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    model: Option<RegisterAgentArtifactRequestArtifactModel>,
    name: Option<String>,
    output_contract_preview: Option<RegisterAgentArtifactRequestArtifactOutputContractPreview>,
    policy: Option<RegisterAgentArtifactRequestArtifactPolicy>,
    provenance: Option<RegisterAgentArtifactRequestArtifactProvenance>,
    provider_options: Option<HashMap<String, serde_json::Value>>,
    provider_options_hash: Option<String>,
    schema_version: Option<String>,
    tools: Option<Vec<RegisterAgentArtifactRequestArtifactToolsItem>>,
    version: Option<String>,
    workflow_graph_preview: Option<RegisterAgentArtifactRequestArtifactWorkflowGraphPreview>,
}

impl RegisterAgentArtifactRequestArtifactBuilder {
    pub fn artifact_id(mut self, value: impl Into<String>) -> Self {
        self.artifact_id = Some(value.into());
        self
    }

    pub fn config_hash(mut self, value: impl Into<String>) -> Self {
        self.config_hash = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn framework(mut self, value: RegisterAgentArtifactRequestArtifactFramework) -> Self {
        self.framework = Some(value);
        self
    }

    pub fn input_contract_preview(
        mut self,
        value: RegisterAgentArtifactRequestArtifactInputContractPreview,
    ) -> Self {
        self.input_contract_preview = Some(value);
        self
    }

    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }

    pub fn instructions_hash(mut self, value: impl Into<String>) -> Self {
        self.instructions_hash = Some(value.into());
        self
    }

    pub fn knowledge_sources(
        mut self,
        value: Vec<RegisterAgentArtifactRequestArtifactKnowledgeSourcesItem>,
    ) -> Self {
        self.knowledge_sources = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn model(mut self, value: RegisterAgentArtifactRequestArtifactModel) -> Self {
        self.model = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn output_contract_preview(
        mut self,
        value: RegisterAgentArtifactRequestArtifactOutputContractPreview,
    ) -> Self {
        self.output_contract_preview = Some(value);
        self
    }

    pub fn policy(mut self, value: RegisterAgentArtifactRequestArtifactPolicy) -> Self {
        self.policy = Some(value);
        self
    }

    pub fn provenance(mut self, value: RegisterAgentArtifactRequestArtifactProvenance) -> Self {
        self.provenance = Some(value);
        self
    }

    pub fn provider_options(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.provider_options = Some(value);
        self
    }

    pub fn provider_options_hash(mut self, value: impl Into<String>) -> Self {
        self.provider_options_hash = Some(value.into());
        self
    }

    pub fn schema_version(mut self, value: impl Into<String>) -> Self {
        self.schema_version = Some(value.into());
        self
    }

    pub fn tools(mut self, value: Vec<RegisterAgentArtifactRequestArtifactToolsItem>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn workflow_graph_preview(
        mut self,
        value: RegisterAgentArtifactRequestArtifactWorkflowGraphPreview,
    ) -> Self {
        self.workflow_graph_preview = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RegisterAgentArtifactRequestArtifact`].
    /// This method will fail if any of the following fields are not set:
    /// - [`artifact_id`](RegisterAgentArtifactRequestArtifactBuilder::artifact_id)
    /// - [`config_hash`](RegisterAgentArtifactRequestArtifactBuilder::config_hash)
    /// - [`framework`](RegisterAgentArtifactRequestArtifactBuilder::framework)
    /// - [`model`](RegisterAgentArtifactRequestArtifactBuilder::model)
    /// - [`name`](RegisterAgentArtifactRequestArtifactBuilder::name)
    /// - [`provenance`](RegisterAgentArtifactRequestArtifactBuilder::provenance)
    /// - [`schema_version`](RegisterAgentArtifactRequestArtifactBuilder::schema_version)
    /// - [`tools`](RegisterAgentArtifactRequestArtifactBuilder::tools)
    /// - [`version`](RegisterAgentArtifactRequestArtifactBuilder::version)
    pub fn build(self) -> Result<RegisterAgentArtifactRequestArtifact, BuildError> {
        Ok(RegisterAgentArtifactRequestArtifact {
            artifact_id: self
                .artifact_id
                .ok_or_else(|| BuildError::missing_field("artifact_id"))?,
            config_hash: self
                .config_hash
                .ok_or_else(|| BuildError::missing_field("config_hash"))?,
            description: self.description,
            framework: self
                .framework
                .ok_or_else(|| BuildError::missing_field("framework"))?,
            input_contract_preview: self.input_contract_preview,
            instructions: self.instructions,
            instructions_hash: self.instructions_hash,
            knowledge_sources: self.knowledge_sources,
            metadata: self.metadata,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            output_contract_preview: self.output_contract_preview,
            policy: self.policy,
            provenance: self
                .provenance
                .ok_or_else(|| BuildError::missing_field("provenance"))?,
            provider_options: self.provider_options,
            provider_options_hash: self.provider_options_hash,
            schema_version: self
                .schema_version
                .ok_or_else(|| BuildError::missing_field("schema_version"))?,
            tools: self
                .tools
                .ok_or_else(|| BuildError::missing_field("tools"))?,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
            workflow_graph_preview: self.workflow_graph_preview,
        })
    }
}
