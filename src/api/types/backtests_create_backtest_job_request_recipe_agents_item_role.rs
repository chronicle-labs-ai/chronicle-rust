pub use crate::prelude::*;

/// Defaults to `candidate`; the Results table treats the first baseline as the reference column.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateBacktestJobRequestRecipeAgentsItemRole {
    Baseline,
    Candidate,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateBacktestJobRequestRecipeAgentsItemRole {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Baseline => serializer.serialize_str("baseline"),
            Self::Candidate => serializer.serialize_str("candidate"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateBacktestJobRequestRecipeAgentsItemRole {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "baseline" => Ok(Self::Baseline),
            "candidate" => Ok(Self::Candidate),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateBacktestJobRequestRecipeAgentsItemRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Baseline => write!(f, "baseline"),
            Self::Candidate => write!(f, "candidate"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
