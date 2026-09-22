pub use crate::prelude::*;

/// Language of a code scorer. Handlers run inside the trial's trusted verifier sandbox: Python via `python3`, TypeScript via `node` (>= 22.6, type-stripping) — both shipped in the sandbox runtime image.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemCodeLanguage {
    Python,
    Typescript,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemCodeLanguage {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Python => serializer.serialize_str("python"),
            Self::Typescript => serializer.serialize_str("typescript"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de>
    for BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemCodeLanguage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "python" => Ok(Self::Python),
            "typescript" => Ok(Self::Typescript),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemCodeLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Python => write!(f, "python"),
            Self::Typescript => write!(f, "typescript"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
