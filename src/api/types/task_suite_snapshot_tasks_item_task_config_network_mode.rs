pub use crate::prelude::*;

/// Network policy for the agent sandbox, mirroring Harbor's `[environment].network_mode`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaskSuiteSnapshotTasksItemTaskConfigNetworkMode {
    Public,
    NoNetwork,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TaskSuiteSnapshotTasksItemTaskConfigNetworkMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Public => serializer.serialize_str("public"),
            Self::NoNetwork => serializer.serialize_str("no-network"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TaskSuiteSnapshotTasksItemTaskConfigNetworkMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "public" => Ok(Self::Public),
            "no-network" => Ok(Self::NoNetwork),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TaskSuiteSnapshotTasksItemTaskConfigNetworkMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Public => write!(f, "public"),
            Self::NoNetwork => write!(f, "no-network"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
