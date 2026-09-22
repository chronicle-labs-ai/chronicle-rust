pub use crate::prelude::*;

/// New split, or `null` to mark unassigned.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateTracesRequestPatchSplit {
    Train,
    Validation,
    Test,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateTracesRequestPatchSplit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Train => serializer.serialize_str("train"),
            Self::Validation => serializer.serialize_str("validation"),
            Self::Test => serializer.serialize_str("test"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateTracesRequestPatchSplit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "train" => Ok(Self::Train),
            "validation" => Ok(Self::Validation),
            "test" => Ok(Self::Test),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateTracesRequestPatchSplit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Train => write!(f, "train"),
            Self::Validation => write!(f, "validation"),
            Self::Test => write!(f, "test"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
