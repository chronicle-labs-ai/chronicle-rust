pub use crate::prelude::*;

/// Row projection of `"BacktestArtifact"`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BacktestTrialDetailResponseArtifactsItemKind {
    AgentLog,
    VerifierLog,
    Trajectory,
    Screenshot,
    RewardJson,
    RewardTxt,
    Tar,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BacktestTrialDetailResponseArtifactsItemKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AgentLog => serializer.serialize_str("agent-log"),
            Self::VerifierLog => serializer.serialize_str("verifier-log"),
            Self::Trajectory => serializer.serialize_str("trajectory"),
            Self::Screenshot => serializer.serialize_str("screenshot"),
            Self::RewardJson => serializer.serialize_str("reward-json"),
            Self::RewardTxt => serializer.serialize_str("reward-txt"),
            Self::Tar => serializer.serialize_str("tar"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BacktestTrialDetailResponseArtifactsItemKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "agent-log" => Ok(Self::AgentLog),
            "verifier-log" => Ok(Self::VerifierLog),
            "trajectory" => Ok(Self::Trajectory),
            "screenshot" => Ok(Self::Screenshot),
            "reward-json" => Ok(Self::RewardJson),
            "reward-txt" => Ok(Self::RewardTxt),
            "tar" => Ok(Self::Tar),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BacktestTrialDetailResponseArtifactsItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AgentLog => write!(f, "agent-log"),
            Self::VerifierLog => write!(f, "verifier-log"),
            Self::Trajectory => write!(f, "trajectory"),
            Self::Screenshot => write!(f, "screenshot"),
            Self::RewardJson => write!(f, "reward-json"),
            Self::RewardTxt => write!(f, "reward-txt"),
            Self::Tar => write!(f, "tar"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
