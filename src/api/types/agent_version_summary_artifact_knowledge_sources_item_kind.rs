pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentVersionSummaryArtifactKnowledgeSourcesItemKind {
    Vector,
    Doc,
    Table,
    Graph,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AgentVersionSummaryArtifactKnowledgeSourcesItemKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Vector => serializer.serialize_str("vector"),
            Self::Doc => serializer.serialize_str("doc"),
            Self::Table => serializer.serialize_str("table"),
            Self::Graph => serializer.serialize_str("graph"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AgentVersionSummaryArtifactKnowledgeSourcesItemKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "vector" => Ok(Self::Vector),
            "doc" => Ok(Self::Doc),
            "table" => Ok(Self::Table),
            "graph" => Ok(Self::Graph),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AgentVersionSummaryArtifactKnowledgeSourcesItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Vector => write!(f, "vector"),
            Self::Doc => write!(f, "doc"),
            Self::Table => write!(f, "table"),
            Self::Graph => write!(f, "graph"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
