pub use crate::prelude::*;

/// Who performed a step; one timeline lane per actor.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BacktestTrialDetailResponseStepsItemActor {
    Environment,
    Agent,
    Verifier,
    Grader,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BacktestTrialDetailResponseStepsItemActor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Environment => serializer.serialize_str("environment"),
            Self::Agent => serializer.serialize_str("agent"),
            Self::Verifier => serializer.serialize_str("verifier"),
            Self::Grader => serializer.serialize_str("grader"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BacktestTrialDetailResponseStepsItemActor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "environment" => Ok(Self::Environment),
            "agent" => Ok(Self::Agent),
            "verifier" => Ok(Self::Verifier),
            "grader" => Ok(Self::Grader),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BacktestTrialDetailResponseStepsItemActor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Environment => write!(f, "environment"),
            Self::Agent => write!(f, "agent"),
            Self::Verifier => write!(f, "verifier"),
            Self::Grader => write!(f, "grader"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
