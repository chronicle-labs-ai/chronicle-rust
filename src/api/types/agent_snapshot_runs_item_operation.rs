pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentSnapshotRunsItemOperation {
    Generate,
    Stream,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AgentSnapshotRunsItemOperation {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Generate => serializer.serialize_str("generate"),
            Self::Stream => serializer.serialize_str("stream"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AgentSnapshotRunsItemOperation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "generate" => Ok(Self::Generate),
            "stream" => Ok(Self::Stream),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AgentSnapshotRunsItemOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generate => write!(f, "generate"),
            Self::Stream => write!(f, "stream"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
