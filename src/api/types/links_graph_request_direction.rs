pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GraphRequestDirection {
    Outgoing,
    Incoming,
    Both,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GraphRequestDirection {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Outgoing => serializer.serialize_str("outgoing"),
            Self::Incoming => serializer.serialize_str("incoming"),
            Self::Both => serializer.serialize_str("both"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GraphRequestDirection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "outgoing" => Ok(Self::Outgoing),
            "incoming" => Ok(Self::Incoming),
            "both" => Ok(Self::Both),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GraphRequestDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Outgoing => write!(f, "outgoing"),
            Self::Incoming => write!(f, "incoming"),
            Self::Both => write!(f, "both"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
