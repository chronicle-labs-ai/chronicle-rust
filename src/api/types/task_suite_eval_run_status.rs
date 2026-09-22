pub use crate::prelude::*;

/// Status badge tone for an eval run.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaskSuiteEvalRunStatus {
    Passing,
    Regressed,
    Running,
    Failed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TaskSuiteEvalRunStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Passing => serializer.serialize_str("passing"),
            Self::Regressed => serializer.serialize_str("regressed"),
            Self::Running => serializer.serialize_str("running"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TaskSuiteEvalRunStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "passing" => Ok(Self::Passing),
            "regressed" => Ok(Self::Regressed),
            "running" => Ok(Self::Running),
            "failed" => Ok(Self::Failed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TaskSuiteEvalRunStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Passing => write!(f, "passing"),
            Self::Regressed => write!(f, "regressed"),
            Self::Running => write!(f, "running"),
            Self::Failed => write!(f, "failed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
