pub use crate::prelude::*;

/// The 13 hash domains the wrapper tracks. The first eight describe the artifact (config-time); the last five describe a run (observed at call-time).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentSnapshotHashIndexItemKind {
    AgentRoot,
    Prompt,
    ModelContract,
    ProviderOptions,
    ToolContract,
    RuntimePolicy,
    Dependency,
    KnowledgeContract,
    WorkflowGraph,
    EffectiveRun,
    ProviderObservation,
    Operational,
    Output,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AgentSnapshotHashIndexItemKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AgentRoot => serializer.serialize_str("agent.root"),
            Self::Prompt => serializer.serialize_str("prompt"),
            Self::ModelContract => serializer.serialize_str("model.contract"),
            Self::ProviderOptions => serializer.serialize_str("provider.options"),
            Self::ToolContract => serializer.serialize_str("tool.contract"),
            Self::RuntimePolicy => serializer.serialize_str("runtime.policy"),
            Self::Dependency => serializer.serialize_str("dependency"),
            Self::KnowledgeContract => serializer.serialize_str("knowledge.contract"),
            Self::WorkflowGraph => serializer.serialize_str("workflow.graph"),
            Self::EffectiveRun => serializer.serialize_str("effective.run"),
            Self::ProviderObservation => serializer.serialize_str("provider.observation"),
            Self::Operational => serializer.serialize_str("operational"),
            Self::Output => serializer.serialize_str("output"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AgentSnapshotHashIndexItemKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "agent.root" => Ok(Self::AgentRoot),
            "prompt" => Ok(Self::Prompt),
            "model.contract" => Ok(Self::ModelContract),
            "provider.options" => Ok(Self::ProviderOptions),
            "tool.contract" => Ok(Self::ToolContract),
            "runtime.policy" => Ok(Self::RuntimePolicy),
            "dependency" => Ok(Self::Dependency),
            "knowledge.contract" => Ok(Self::KnowledgeContract),
            "workflow.graph" => Ok(Self::WorkflowGraph),
            "effective.run" => Ok(Self::EffectiveRun),
            "provider.observation" => Ok(Self::ProviderObservation),
            "operational" => Ok(Self::Operational),
            "output" => Ok(Self::Output),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AgentSnapshotHashIndexItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AgentRoot => write!(f, "agent.root"),
            Self::Prompt => write!(f, "prompt"),
            Self::ModelContract => write!(f, "model.contract"),
            Self::ProviderOptions => write!(f, "provider.options"),
            Self::ToolContract => write!(f, "tool.contract"),
            Self::RuntimePolicy => write!(f, "runtime.policy"),
            Self::Dependency => write!(f, "dependency"),
            Self::KnowledgeContract => write!(f, "knowledge.contract"),
            Self::WorkflowGraph => write!(f, "workflow.graph"),
            Self::EffectiveRun => write!(f, "effective.run"),
            Self::ProviderObservation => write!(f, "provider.observation"),
            Self::Operational => write!(f, "operational"),
            Self::Output => write!(f, "output"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
