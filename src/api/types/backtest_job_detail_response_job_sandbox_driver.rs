pub use crate::prelude::*;

/// Sandbox driver selected by trusted server policy at submit time. Echoed onto the `BacktestJob.sandboxDriver` column so persisted jobs record which implementation backed the `Sandbox` trait.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BacktestJobDetailResponseJobSandboxDriver {
    Docker,
    Daytona,
    Mock,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BacktestJobDetailResponseJobSandboxDriver {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Docker => serializer.serialize_str("docker"),
            Self::Daytona => serializer.serialize_str("daytona"),
            Self::Mock => serializer.serialize_str("mock"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BacktestJobDetailResponseJobSandboxDriver {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "docker" => Ok(Self::Docker),
            "daytona" => Ok(Self::Daytona),
            "mock" => Ok(Self::Mock),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BacktestJobDetailResponseJobSandboxDriver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Docker => write!(f, "docker"),
            Self::Daytona => write!(f, "daytona"),
            Self::Mock => write!(f, "mock"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
