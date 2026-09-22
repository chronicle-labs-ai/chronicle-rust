pub use crate::prelude::*;

/// Intended use of a dataset — drives the colored badge on the picker and lets apps route additions to the right backend (eval suite, training set, replay corpus, manual review queue).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateTaskSuiteWithTraceResponseDatasetPurpose {
    Eval,
    Training,
    Replay,
    Review,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateTaskSuiteWithTraceResponseDatasetPurpose {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Eval => serializer.serialize_str("eval"),
            Self::Training => serializer.serialize_str("training"),
            Self::Replay => serializer.serialize_str("replay"),
            Self::Review => serializer.serialize_str("review"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateTaskSuiteWithTraceResponseDatasetPurpose {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "eval" => Ok(Self::Eval),
            "training" => Ok(Self::Training),
            "replay" => Ok(Self::Replay),
            "review" => Ok(Self::Review),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateTaskSuiteWithTraceResponseDatasetPurpose {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eval => write!(f, "eval"),
            Self::Training => write!(f, "training"),
            Self::Replay => write!(f, "replay"),
            Self::Review => write!(f, "review"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
