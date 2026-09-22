pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateBacktestJobRequestRecipeDataScenariosItemKind {
    Adversarial,
    NonEnglish,
    ToolFailure,
    LongTurn,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateBacktestJobRequestRecipeDataScenariosItemKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Adversarial => serializer.serialize_str("adversarial"),
            Self::NonEnglish => serializer.serialize_str("nonEnglish"),
            Self::ToolFailure => serializer.serialize_str("toolFailure"),
            Self::LongTurn => serializer.serialize_str("longTurn"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateBacktestJobRequestRecipeDataScenariosItemKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "adversarial" => Ok(Self::Adversarial),
            "nonEnglish" => Ok(Self::NonEnglish),
            "toolFailure" => Ok(Self::ToolFailure),
            "longTurn" => Ok(Self::LongTurn),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateBacktestJobRequestRecipeDataScenariosItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Adversarial => write!(f, "adversarial"),
            Self::NonEnglish => write!(f, "nonEnglish"),
            Self::ToolFailure => write!(f, "toolFailure"),
            Self::LongTurn => write!(f, "longTurn"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
