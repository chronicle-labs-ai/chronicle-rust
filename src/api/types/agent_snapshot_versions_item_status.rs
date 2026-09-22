pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentSnapshotVersionsItemStatus {
    Current,
    Stable,
    Deprecated,
    Draft,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AgentSnapshotVersionsItemStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Current => serializer.serialize_str("current"),
            Self::Stable => serializer.serialize_str("stable"),
            Self::Deprecated => serializer.serialize_str("deprecated"),
            Self::Draft => serializer.serialize_str("draft"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AgentSnapshotVersionsItemStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "current" => Ok(Self::Current),
            "stable" => Ok(Self::Stable),
            "deprecated" => Ok(Self::Deprecated),
            "draft" => Ok(Self::Draft),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AgentSnapshotVersionsItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Current => write!(f, "current"),
            Self::Stable => write!(f, "stable"),
            Self::Deprecated => write!(f, "deprecated"),
            Self::Draft => write!(f, "draft"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
