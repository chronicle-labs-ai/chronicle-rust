pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentVersionSummaryArtifact {
    #[serde(rename = "artifactId")]
    #[serde(default)]
    pub artifact_id: String,
    #[serde(rename = "configHash")]
    #[serde(default)]
    pub config_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Framework label. Multi-word kebab-case to match the existing TS union (`vercel-ai-sdk`, `openai-agents-python`, etc.).
    pub framework: AgentVersionSummaryArtifactFramework,
    /// Compact preview of the input contract this artifact expects.
    #[serde(rename = "inputContractPreview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_contract_preview: Option<AgentVersionSummaryArtifactInputContractPreview>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(rename = "instructionsHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions_hash: Option<String>,
    #[serde(rename = "knowledgeSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_sources: Option<Vec<AgentVersionSummaryArtifactKnowledgeSourcesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub model: AgentVersionSummaryArtifactModel,
    #[serde(default)]
    pub name: String,
    /// Compact preview of the output contract this artifact emits.
    #[serde(rename = "outputContractPreview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_contract_preview: Option<AgentVersionSummaryArtifactOutputContractPreview>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<AgentVersionSummaryArtifactPolicy>,
    #[serde(default)]
    pub provenance: AgentVersionSummaryArtifactProvenance,
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
    pub tools: Vec<AgentVersionSummaryArtifactToolsItem>,
    #[serde(default)]
    pub version: String,
    #[serde(rename = "workflowGraphPreview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_graph_preview: Option<AgentVersionSummaryArtifactWorkflowGraphPreview>,
}

impl AgentVersionSummaryArtifact {
    pub fn builder() -> AgentVersionSummaryArtifactBuilder {
        <AgentVersionSummaryArtifactBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentVersionSummaryArtifactBuilder {
    artifact_id: Option<String>,
    config_hash: Option<String>,
    description: Option<String>,
    framework: Option<AgentVersionSummaryArtifactFramework>,
    input_contract_preview: Option<AgentVersionSummaryArtifactInputContractPreview>,
    instructions: Option<String>,
    instructions_hash: Option<String>,
    knowledge_sources: Option<Vec<AgentVersionSummaryArtifactKnowledgeSourcesItem>>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    model: Option<AgentVersionSummaryArtifactModel>,
    name: Option<String>,
    output_contract_preview: Option<AgentVersionSummaryArtifactOutputContractPreview>,
    policy: Option<AgentVersionSummaryArtifactPolicy>,
    provenance: Option<AgentVersionSummaryArtifactProvenance>,
    provider_options: Option<HashMap<String, serde_json::Value>>,
    provider_options_hash: Option<String>,
    schema_version: Option<String>,
    tools: Option<Vec<AgentVersionSummaryArtifactToolsItem>>,
    version: Option<String>,
    workflow_graph_preview: Option<AgentVersionSummaryArtifactWorkflowGraphPreview>,
}

impl AgentVersionSummaryArtifactBuilder {
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

    pub fn framework(mut self, value: AgentVersionSummaryArtifactFramework) -> Self {
        self.framework = Some(value);
        self
    }

    pub fn input_contract_preview(
        mut self,
        value: AgentVersionSummaryArtifactInputContractPreview,
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
        value: Vec<AgentVersionSummaryArtifactKnowledgeSourcesItem>,
    ) -> Self {
        self.knowledge_sources = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn model(mut self, value: AgentVersionSummaryArtifactModel) -> Self {
        self.model = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn output_contract_preview(
        mut self,
        value: AgentVersionSummaryArtifactOutputContractPreview,
    ) -> Self {
        self.output_contract_preview = Some(value);
        self
    }

    pub fn policy(mut self, value: AgentVersionSummaryArtifactPolicy) -> Self {
        self.policy = Some(value);
        self
    }

    pub fn provenance(mut self, value: AgentVersionSummaryArtifactProvenance) -> Self {
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

    pub fn tools(mut self, value: Vec<AgentVersionSummaryArtifactToolsItem>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn workflow_graph_preview(
        mut self,
        value: AgentVersionSummaryArtifactWorkflowGraphPreview,
    ) -> Self {
        self.workflow_graph_preview = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentVersionSummaryArtifact`].
    /// This method will fail if any of the following fields are not set:
    /// - [`artifact_id`](AgentVersionSummaryArtifactBuilder::artifact_id)
    /// - [`config_hash`](AgentVersionSummaryArtifactBuilder::config_hash)
    /// - [`framework`](AgentVersionSummaryArtifactBuilder::framework)
    /// - [`model`](AgentVersionSummaryArtifactBuilder::model)
    /// - [`name`](AgentVersionSummaryArtifactBuilder::name)
    /// - [`provenance`](AgentVersionSummaryArtifactBuilder::provenance)
    /// - [`schema_version`](AgentVersionSummaryArtifactBuilder::schema_version)
    /// - [`tools`](AgentVersionSummaryArtifactBuilder::tools)
    /// - [`version`](AgentVersionSummaryArtifactBuilder::version)
    pub fn build(self) -> Result<AgentVersionSummaryArtifact, BuildError> {
        Ok(AgentVersionSummaryArtifact {
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
