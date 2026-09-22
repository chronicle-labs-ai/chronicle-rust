pub use crate::prelude::*;

/// Framework label. Multi-word kebab-case to match the existing TS union (`vercel-ai-sdk`, `openai-agents-python`, etc.).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BacktestsAvailabilityAgentsItemFramework {
    VercelAiSdk,
    OpenaiAgents,
    Langchain,
    Mastra,
    LangchainPython,
    Llamaindex,
    Crewai,
    Smolagents,
    PydanticAi,
    Strands,
    GoogleAdk,
    OpenaiAgentsPython,
    Autogen,
    Eve,
    SalesforceAgentforce,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BacktestsAvailabilityAgentsItemFramework {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::VercelAiSdk => serializer.serialize_str("vercel-ai-sdk"),
            Self::OpenaiAgents => serializer.serialize_str("openai-agents"),
            Self::Langchain => serializer.serialize_str("langchain"),
            Self::Mastra => serializer.serialize_str("mastra"),
            Self::LangchainPython => serializer.serialize_str("langchain-python"),
            Self::Llamaindex => serializer.serialize_str("llamaindex"),
            Self::Crewai => serializer.serialize_str("crewai"),
            Self::Smolagents => serializer.serialize_str("smolagents"),
            Self::PydanticAi => serializer.serialize_str("pydantic-ai"),
            Self::Strands => serializer.serialize_str("strands"),
            Self::GoogleAdk => serializer.serialize_str("google-adk"),
            Self::OpenaiAgentsPython => serializer.serialize_str("openai-agents-python"),
            Self::Autogen => serializer.serialize_str("autogen"),
            Self::Eve => serializer.serialize_str("eve"),
            Self::SalesforceAgentforce => serializer.serialize_str("salesforce-agentforce"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BacktestsAvailabilityAgentsItemFramework {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "vercel-ai-sdk" => Ok(Self::VercelAiSdk),
            "openai-agents" => Ok(Self::OpenaiAgents),
            "langchain" => Ok(Self::Langchain),
            "mastra" => Ok(Self::Mastra),
            "langchain-python" => Ok(Self::LangchainPython),
            "llamaindex" => Ok(Self::Llamaindex),
            "crewai" => Ok(Self::Crewai),
            "smolagents" => Ok(Self::Smolagents),
            "pydantic-ai" => Ok(Self::PydanticAi),
            "strands" => Ok(Self::Strands),
            "google-adk" => Ok(Self::GoogleAdk),
            "openai-agents-python" => Ok(Self::OpenaiAgentsPython),
            "autogen" => Ok(Self::Autogen),
            "eve" => Ok(Self::Eve),
            "salesforce-agentforce" => Ok(Self::SalesforceAgentforce),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BacktestsAvailabilityAgentsItemFramework {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VercelAiSdk => write!(f, "vercel-ai-sdk"),
            Self::OpenaiAgents => write!(f, "openai-agents"),
            Self::Langchain => write!(f, "langchain"),
            Self::Mastra => write!(f, "mastra"),
            Self::LangchainPython => write!(f, "langchain-python"),
            Self::Llamaindex => write!(f, "llamaindex"),
            Self::Crewai => write!(f, "crewai"),
            Self::Smolagents => write!(f, "smolagents"),
            Self::PydanticAi => write!(f, "pydantic-ai"),
            Self::Strands => write!(f, "strands"),
            Self::GoogleAdk => write!(f, "google-adk"),
            Self::OpenaiAgentsPython => write!(f, "openai-agents-python"),
            Self::Autogen => write!(f, "autogen"),
            Self::Eve => write!(f, "eve"),
            Self::SalesforceAgentforce => write!(f, "salesforce-agentforce"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
