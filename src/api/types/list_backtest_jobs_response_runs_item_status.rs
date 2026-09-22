pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListBacktestJobsResponseRunsItemStatus {
    Running,
    Done,
    Paused,
    Scheduled,
    Draft,
    Failed,
    Cancelled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListBacktestJobsResponseRunsItemStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Running => serializer.serialize_str("running"),
            Self::Done => serializer.serialize_str("done"),
            Self::Paused => serializer.serialize_str("paused"),
            Self::Scheduled => serializer.serialize_str("scheduled"),
            Self::Draft => serializer.serialize_str("draft"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::Cancelled => serializer.serialize_str("cancelled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListBacktestJobsResponseRunsItemStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "running" => Ok(Self::Running),
            "done" => Ok(Self::Done),
            "paused" => Ok(Self::Paused),
            "scheduled" => Ok(Self::Scheduled),
            "draft" => Ok(Self::Draft),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListBacktestJobsResponseRunsItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Running => write!(f, "running"),
            Self::Done => write!(f, "done"),
            Self::Paused => write!(f, "paused"),
            Self::Scheduled => write!(f, "scheduled"),
            Self::Draft => write!(f, "draft"),
            Self::Failed => write!(f, "failed"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
