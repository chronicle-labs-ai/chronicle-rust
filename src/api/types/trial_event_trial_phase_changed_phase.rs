pub use crate::prelude::*;

/// Phase of execution emitted on the SSE stream as the trial progresses. Distinct from `TrialStatus` — `TrialPhase` is fine-grained progress inside the lifecycle, `TrialStatus` is the persisted summary.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TrialEventTrialPhaseChangedPhase {
    Queued,
    EnvironmentStart,
    EnvironmentReady,
    AgentSetup,
    AgentRunning,
    VerifierRunning,
    ArtifactCollection,
    Cleanup,
    Done,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TrialEventTrialPhaseChangedPhase {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Queued => serializer.serialize_str("queued"),
            Self::EnvironmentStart => serializer.serialize_str("environment-start"),
            Self::EnvironmentReady => serializer.serialize_str("environment-ready"),
            Self::AgentSetup => serializer.serialize_str("agent-setup"),
            Self::AgentRunning => serializer.serialize_str("agent-running"),
            Self::VerifierRunning => serializer.serialize_str("verifier-running"),
            Self::ArtifactCollection => serializer.serialize_str("artifact-collection"),
            Self::Cleanup => serializer.serialize_str("cleanup"),
            Self::Done => serializer.serialize_str("done"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TrialEventTrialPhaseChangedPhase {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "queued" => Ok(Self::Queued),
            "environment-start" => Ok(Self::EnvironmentStart),
            "environment-ready" => Ok(Self::EnvironmentReady),
            "agent-setup" => Ok(Self::AgentSetup),
            "agent-running" => Ok(Self::AgentRunning),
            "verifier-running" => Ok(Self::VerifierRunning),
            "artifact-collection" => Ok(Self::ArtifactCollection),
            "cleanup" => Ok(Self::Cleanup),
            "done" => Ok(Self::Done),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TrialEventTrialPhaseChangedPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Queued => write!(f, "queued"),
            Self::EnvironmentStart => write!(f, "environment-start"),
            Self::EnvironmentReady => write!(f, "environment-ready"),
            Self::AgentSetup => write!(f, "agent-setup"),
            Self::AgentRunning => write!(f, "agent-running"),
            Self::VerifierRunning => write!(f, "verifier-running"),
            Self::ArtifactCollection => write!(f, "artifact-collection"),
            Self::Cleanup => write!(f, "cleanup"),
            Self::Done => write!(f, "done"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
