pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateSdkKeyRequestScopesItem {
    TracesWrite,
    EventsRead,
    EventsWrite,
    UsersWrite,
    SignalsWrite,
    AgentsWrite,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateSdkKeyRequestScopesItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::TracesWrite => serializer.serialize_str("traces:write"),
            Self::EventsRead => serializer.serialize_str("events:read"),
            Self::EventsWrite => serializer.serialize_str("events:write"),
            Self::UsersWrite => serializer.serialize_str("users:write"),
            Self::SignalsWrite => serializer.serialize_str("signals:write"),
            Self::AgentsWrite => serializer.serialize_str("agents:write"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateSdkKeyRequestScopesItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "traces:write" => Ok(Self::TracesWrite),
            "events:read" => Ok(Self::EventsRead),
            "events:write" => Ok(Self::EventsWrite),
            "users:write" => Ok(Self::UsersWrite),
            "signals:write" => Ok(Self::SignalsWrite),
            "agents:write" => Ok(Self::AgentsWrite),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateSdkKeyRequestScopesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TracesWrite => write!(f, "traces:write"),
            Self::EventsRead => write!(f, "events:read"),
            Self::EventsWrite => write!(f, "events:write"),
            Self::UsersWrite => write!(f, "users:write"),
            Self::SignalsWrite => write!(f, "signals:write"),
            Self::AgentsWrite => write!(f, "agents:write"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
