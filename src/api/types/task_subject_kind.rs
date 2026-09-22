pub use crate::prelude::*;

/// The canonical event-store subject curated into a Dataset. Providers that cannot form a real trace keep a single event subject rather than inventing a trace identifier. `Task` marks a hand-authored task with no captured subject; its revision holds zero events and its subject id is minted by the service.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaskSubjectKind {
    Trace,
    Event,
    Task,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TaskSubjectKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Trace => serializer.serialize_str("trace"),
            Self::Event => serializer.serialize_str("event"),
            Self::Task => serializer.serialize_str("task"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TaskSubjectKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "trace" => Ok(Self::Trace),
            "event" => Ok(Self::Event),
            "task" => Ok(Self::Task),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TaskSubjectKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Trace => write!(f, "trace"),
            Self::Event => write!(f, "event"),
            Self::Task => write!(f, "task"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
