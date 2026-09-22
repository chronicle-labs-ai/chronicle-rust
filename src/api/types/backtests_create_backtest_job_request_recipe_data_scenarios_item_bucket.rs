pub use crate::prelude::*;

/// Cluster bucket emitted by the data-science layer when looking for missing scenarios in a dataset.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateBacktestJobRequestRecipeDataScenariosItemBucket {
    Captured,
    Adjacent,
    Emerging,
    Edge,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateBacktestJobRequestRecipeDataScenariosItemBucket {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Captured => serializer.serialize_str("captured"),
            Self::Adjacent => serializer.serialize_str("adjacent"),
            Self::Emerging => serializer.serialize_str("emerging"),
            Self::Edge => serializer.serialize_str("edge"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateBacktestJobRequestRecipeDataScenariosItemBucket {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "captured" => Ok(Self::Captured),
            "adjacent" => Ok(Self::Adjacent),
            "emerging" => Ok(Self::Emerging),
            "edge" => Ok(Self::Edge),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateBacktestJobRequestRecipeDataScenariosItemBucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Captured => write!(f, "captured"),
            Self::Adjacent => write!(f, "adjacent"),
            Self::Emerging => write!(f, "emerging"),
            Self::Edge => write!(f, "edge"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
