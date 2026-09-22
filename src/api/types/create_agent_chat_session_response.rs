pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateAgentChatSessionResponse {
    /// A live chat conversation with a registered agent version. Sessions are in-memory (lost on restart); the events, trace, and recorded run are durable.
    #[serde(default)]
    pub session: CreateAgentChatSessionResponseSession,
}

impl CreateAgentChatSessionResponse {
    pub fn builder() -> CreateAgentChatSessionResponseBuilder {
        <CreateAgentChatSessionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAgentChatSessionResponseBuilder {
    session: Option<CreateAgentChatSessionResponseSession>,
}

impl CreateAgentChatSessionResponseBuilder {
    pub fn session(mut self, value: CreateAgentChatSessionResponseSession) -> Self {
        self.session = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAgentChatSessionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`session`](CreateAgentChatSessionResponseBuilder::session)
    pub fn build(self) -> Result<CreateAgentChatSessionResponse, BuildError> {
        Ok(CreateAgentChatSessionResponse {
            session: self
                .session
                .ok_or_else(|| BuildError::missing_field("session"))?,
        })
    }
}
