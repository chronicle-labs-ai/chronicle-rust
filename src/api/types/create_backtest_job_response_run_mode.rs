pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateBacktestJobResponseRunMode {
    Replay,
    Compare,
    Regression,
    Suite,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateBacktestJobResponseRunMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Replay => serializer.serialize_str("replay"),
            Self::Compare => serializer.serialize_str("compare"),
            Self::Regression => serializer.serialize_str("regression"),
            Self::Suite => serializer.serialize_str("suite"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateBacktestJobResponseRunMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "replay" => Ok(Self::Replay),
            "compare" => Ok(Self::Compare),
            "regression" => Ok(Self::Regression),
            "suite" => Ok(Self::Suite),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateBacktestJobResponseRunMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Replay => write!(f, "replay"),
            Self::Compare => write!(f, "compare"),
            Self::Regression => write!(f, "regression"),
            Self::Suite => write!(f, "suite"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
