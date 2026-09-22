pub use crate::prelude::*;

/// Grader source — where this grader came from when it was added to the recipe. Determines the chip copy ("proposed" vs "library" vs "custom" vs "dataset").
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaskSuiteSnapshotTasksItemVerifiersItemSource {
    Proposed,
    Library,
    Custom,
    Dataset,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TaskSuiteSnapshotTasksItemVerifiersItemSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Proposed => serializer.serialize_str("proposed"),
            Self::Library => serializer.serialize_str("library"),
            Self::Custom => serializer.serialize_str("custom"),
            Self::Dataset => serializer.serialize_str("dataset"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TaskSuiteSnapshotTasksItemVerifiersItemSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "proposed" => Ok(Self::Proposed),
            "library" => Ok(Self::Library),
            "custom" => Ok(Self::Custom),
            "dataset" => Ok(Self::Dataset),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TaskSuiteSnapshotTasksItemVerifiersItemSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Proposed => write!(f, "proposed"),
            Self::Library => write!(f, "library"),
            Self::Custom => write!(f, "custom"),
            Self::Dataset => write!(f, "dataset"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
