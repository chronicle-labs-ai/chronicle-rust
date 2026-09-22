pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateBacktestJobRequestRecipeGradersItemKind {
    Rubric,
    Classifier,
    Metric,
    Embedding,
    Assertion,
    Code,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateBacktestJobRequestRecipeGradersItemKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Rubric => serializer.serialize_str("rubric"),
            Self::Classifier => serializer.serialize_str("classifier"),
            Self::Metric => serializer.serialize_str("metric"),
            Self::Embedding => serializer.serialize_str("embedding"),
            Self::Assertion => serializer.serialize_str("assertion"),
            Self::Code => serializer.serialize_str("code"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateBacktestJobRequestRecipeGradersItemKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "rubric" => Ok(Self::Rubric),
            "classifier" => Ok(Self::Classifier),
            "metric" => Ok(Self::Metric),
            "embedding" => Ok(Self::Embedding),
            "assertion" => Ok(Self::Assertion),
            "code" => Ok(Self::Code),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateBacktestJobRequestRecipeGradersItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rubric => write!(f, "rubric"),
            Self::Classifier => write!(f, "classifier"),
            Self::Metric => write!(f, "metric"),
            Self::Embedding => write!(f, "embedding"),
            Self::Assertion => write!(f, "assertion"),
            Self::Code => write!(f, "code"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
