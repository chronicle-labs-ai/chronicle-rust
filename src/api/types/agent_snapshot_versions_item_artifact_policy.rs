pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentSnapshotVersionsItemArtifactPolicy {
    #[serde(rename = "allowedTools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_tools: Option<Vec<String>>,
    #[serde(rename = "approvalRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_required: Option<Vec<String>>,
    #[serde(rename = "maxSteps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_steps: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl AgentSnapshotVersionsItemArtifactPolicy {
    pub fn builder() -> AgentSnapshotVersionsItemArtifactPolicyBuilder {
        <AgentSnapshotVersionsItemArtifactPolicyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSnapshotVersionsItemArtifactPolicyBuilder {
    allowed_tools: Option<Vec<String>>,
    approval_required: Option<Vec<String>>,
    max_steps: Option<i64>,
    metadata: Option<HashMap<String, serde_json::Value>>,
}

impl AgentSnapshotVersionsItemArtifactPolicyBuilder {
    pub fn allowed_tools(mut self, value: Vec<String>) -> Self {
        self.allowed_tools = Some(value);
        self
    }

    pub fn approval_required(mut self, value: Vec<String>) -> Self {
        self.approval_required = Some(value);
        self
    }

    pub fn max_steps(mut self, value: i64) -> Self {
        self.max_steps = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentSnapshotVersionsItemArtifactPolicy`].
    pub fn build(self) -> Result<AgentSnapshotVersionsItemArtifactPolicy, BuildError> {
        Ok(AgentSnapshotVersionsItemArtifactPolicy {
            allowed_tools: self.allowed_tools,
            approval_required: self.approval_required,
            max_steps: self.max_steps,
            metadata: self.metadata,
        })
    }
}
