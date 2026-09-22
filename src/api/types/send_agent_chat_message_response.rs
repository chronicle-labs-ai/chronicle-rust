pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SendAgentChatMessageResponse {
    /// A live chat conversation with a registered agent version. Sessions are in-memory (lost on restart); the events, trace, and recorded run are durable.
    #[serde(default)]
    pub session: SendAgentChatMessageResponseSession,
}

impl SendAgentChatMessageResponse {
    pub fn builder() -> SendAgentChatMessageResponseBuilder {
        <SendAgentChatMessageResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendAgentChatMessageResponseBuilder {
    session: Option<SendAgentChatMessageResponseSession>,
}

impl SendAgentChatMessageResponseBuilder {
    pub fn session(mut self, value: SendAgentChatMessageResponseSession) -> Self {
        self.session = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SendAgentChatMessageResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`session`](SendAgentChatMessageResponseBuilder::session)
    pub fn build(self) -> Result<SendAgentChatMessageResponse, BuildError> {
        Ok(SendAgentChatMessageResponse {
            session: self
                .session
                .ok_or_else(|| BuildError::missing_field("session"))?,
        })
    }
}
