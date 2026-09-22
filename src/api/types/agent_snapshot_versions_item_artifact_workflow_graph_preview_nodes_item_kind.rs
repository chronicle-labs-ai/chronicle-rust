pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewNodesItemKind {
    Input,
    Tool,
    Model,
    Branch,
    Output,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewNodesItemKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Input => serializer.serialize_str("input"),
            Self::Tool => serializer.serialize_str("tool"),
            Self::Model => serializer.serialize_str("model"),
            Self::Branch => serializer.serialize_str("branch"),
            Self::Output => serializer.serialize_str("output"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewNodesItemKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "input" => Ok(Self::Input),
            "tool" => Ok(Self::Tool),
            "model" => Ok(Self::Model),
            "branch" => Ok(Self::Branch),
            "output" => Ok(Self::Output),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AgentSnapshotVersionsItemArtifactWorkflowGraphPreviewNodesItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input => write!(f, "input"),
            Self::Tool => write!(f, "tool"),
            Self::Model => write!(f, "model"),
            Self::Branch => write!(f, "branch"),
            Self::Output => write!(f, "output"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
