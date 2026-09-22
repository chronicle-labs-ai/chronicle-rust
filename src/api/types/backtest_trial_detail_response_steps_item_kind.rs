pub use crate::prelude::*;

/// What a step is. Drives the glyph on the timeline and which detail renderer opens when the span is clicked.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BacktestTrialDetailResponseStepsItemKind {
    SeedEvent,
    StateChange,
    ToolCall,
    Message,
    FinalAnswer,
    Phase,
    VerifierRun,
    Grade,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BacktestTrialDetailResponseStepsItemKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SeedEvent => serializer.serialize_str("seed-event"),
            Self::StateChange => serializer.serialize_str("state-change"),
            Self::ToolCall => serializer.serialize_str("tool-call"),
            Self::Message => serializer.serialize_str("message"),
            Self::FinalAnswer => serializer.serialize_str("final-answer"),
            Self::Phase => serializer.serialize_str("phase"),
            Self::VerifierRun => serializer.serialize_str("verifier-run"),
            Self::Grade => serializer.serialize_str("grade"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BacktestTrialDetailResponseStepsItemKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "seed-event" => Ok(Self::SeedEvent),
            "state-change" => Ok(Self::StateChange),
            "tool-call" => Ok(Self::ToolCall),
            "message" => Ok(Self::Message),
            "final-answer" => Ok(Self::FinalAnswer),
            "phase" => Ok(Self::Phase),
            "verifier-run" => Ok(Self::VerifierRun),
            "grade" => Ok(Self::Grade),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BacktestTrialDetailResponseStepsItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SeedEvent => write!(f, "seed-event"),
            Self::StateChange => write!(f, "state-change"),
            Self::ToolCall => write!(f, "tool-call"),
            Self::Message => write!(f, "message"),
            Self::FinalAnswer => write!(f, "final-answer"),
            Self::Phase => write!(f, "phase"),
            Self::VerifierRun => write!(f, "verifier-run"),
            Self::Grade => write!(f, "grade"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
