pub use crate::prelude::*;

/// A live chat conversation with a registered agent version. Sessions are in-memory (lost on restart); the events, trace, and recorded run are durable.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateAgentChatSessionResponseSession {
    #[serde(rename = "agentName")]
    #[serde(default)]
    pub agent_name: String,
    /// Resolved registry version the session executes (`current`, else `stable`), pinned at session creation.
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    pub agent_version: String,
    #[serde(rename = "artifactId")]
    #[serde(default)]
    pub artifact_id: String,
    #[serde(default)]
    pub messages: Vec<CreateAgentChatSessionResponseSessionMessagesItem>,
    /// Registry run id of the most recently recorded turn. Runs are immutable observations, so each turn records its own run; the shared `traceId` ties them into one conversation.
    #[serde(rename = "runId")]
    #[serde(default)]
    pub run_id: String,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub started_at: DateTime<FixedOffset>,
    /// Trace id shared by every event of the conversation. Opens in the timeline and `/v1/trace-tree/:trace_id`.
    #[serde(rename = "traceId")]
    #[serde(default)]
    pub trace_id: String,
}

impl CreateAgentChatSessionResponseSession {
    pub fn builder() -> CreateAgentChatSessionResponseSessionBuilder {
        <CreateAgentChatSessionResponseSessionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAgentChatSessionResponseSessionBuilder {
    agent_name: Option<String>,
    agent_version: Option<String>,
    artifact_id: Option<String>,
    messages: Option<Vec<CreateAgentChatSessionResponseSessionMessagesItem>>,
    run_id: Option<String>,
    session_id: Option<String>,
    started_at: Option<DateTime<FixedOffset>>,
    trace_id: Option<String>,
}

impl CreateAgentChatSessionResponseSessionBuilder {
    pub fn agent_name(mut self, value: impl Into<String>) -> Self {
        self.agent_name = Some(value.into());
        self
    }

    pub fn agent_version(mut self, value: impl Into<String>) -> Self {
        self.agent_version = Some(value.into());
        self
    }

    pub fn artifact_id(mut self, value: impl Into<String>) -> Self {
        self.artifact_id = Some(value.into());
        self
    }

    pub fn messages(
        mut self,
        value: Vec<CreateAgentChatSessionResponseSessionMessagesItem>,
    ) -> Self {
        self.messages = Some(value);
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAgentChatSessionResponseSession`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_name`](CreateAgentChatSessionResponseSessionBuilder::agent_name)
    /// - [`agent_version`](CreateAgentChatSessionResponseSessionBuilder::agent_version)
    /// - [`artifact_id`](CreateAgentChatSessionResponseSessionBuilder::artifact_id)
    /// - [`messages`](CreateAgentChatSessionResponseSessionBuilder::messages)
    /// - [`run_id`](CreateAgentChatSessionResponseSessionBuilder::run_id)
    /// - [`session_id`](CreateAgentChatSessionResponseSessionBuilder::session_id)
    /// - [`started_at`](CreateAgentChatSessionResponseSessionBuilder::started_at)
    /// - [`trace_id`](CreateAgentChatSessionResponseSessionBuilder::trace_id)
    pub fn build(self) -> Result<CreateAgentChatSessionResponseSession, BuildError> {
        Ok(CreateAgentChatSessionResponseSession {
            agent_name: self
                .agent_name
                .ok_or_else(|| BuildError::missing_field("agent_name"))?,
            agent_version: self
                .agent_version
                .ok_or_else(|| BuildError::missing_field("agent_version"))?,
            artifact_id: self
                .artifact_id
                .ok_or_else(|| BuildError::missing_field("artifact_id"))?,
            messages: self
                .messages
                .ok_or_else(|| BuildError::missing_field("messages"))?,
            run_id: self
                .run_id
                .ok_or_else(|| BuildError::missing_field("run_id"))?,
            session_id: self
                .session_id
                .ok_or_else(|| BuildError::missing_field("session_id"))?,
            started_at: self
                .started_at
                .ok_or_else(|| BuildError::missing_field("started_at"))?,
            trace_id: self
                .trace_id
                .ok_or_else(|| BuildError::missing_field("trace_id"))?,
        })
    }
}
