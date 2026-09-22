pub use crate::prelude::*;

/// Lifecycle state of a single `BacktestTrial` (one (case × agent) cell).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListBacktestJobTrialsResponseTrialsItemStatus {
    Pending,
    Setup,
    Running,
    Verifying,
    Succeeded,
    Failed,
    Cancelled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListBacktestJobTrialsResponseTrialsItemStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pending => serializer.serialize_str("pending"),
            Self::Setup => serializer.serialize_str("setup"),
            Self::Running => serializer.serialize_str("running"),
            Self::Verifying => serializer.serialize_str("verifying"),
            Self::Succeeded => serializer.serialize_str("succeeded"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::Cancelled => serializer.serialize_str("cancelled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListBacktestJobTrialsResponseTrialsItemStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending" => Ok(Self::Pending),
            "setup" => Ok(Self::Setup),
            "running" => Ok(Self::Running),
            "verifying" => Ok(Self::Verifying),
            "succeeded" => Ok(Self::Succeeded),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListBacktestJobTrialsResponseTrialsItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Setup => write!(f, "setup"),
            Self::Running => write!(f, "running"),
            Self::Verifying => write!(f, "verifying"),
            Self::Succeeded => write!(f, "succeeded"),
            Self::Failed => write!(f, "failed"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
