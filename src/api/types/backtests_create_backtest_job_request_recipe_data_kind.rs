pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateBacktestJobRequestRecipeDataKind {
    Composed,
    Dataset,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateBacktestJobRequestRecipeDataKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Composed => serializer.serialize_str("composed"),
            Self::Dataset => serializer.serialize_str("dataset"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateBacktestJobRequestRecipeDataKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "composed" => Ok(Self::Composed),
            "dataset" => Ok(Self::Dataset),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateBacktestJobRequestRecipeDataKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Composed => write!(f, "composed"),
            Self::Dataset => write!(f, "dataset"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
