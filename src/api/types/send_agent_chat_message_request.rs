pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendAgentChatMessageRequest {
    #[serde(default)]
    pub text: String,
}

impl SendAgentChatMessageRequest {
    pub fn builder() -> SendAgentChatMessageRequestBuilder {
        <SendAgentChatMessageRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendAgentChatMessageRequestBuilder {
    text: Option<String>,
}

impl SendAgentChatMessageRequestBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SendAgentChatMessageRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](SendAgentChatMessageRequestBuilder::text)
    pub fn build(self) -> Result<SendAgentChatMessageRequest, BuildError> {
        Ok(SendAgentChatMessageRequest {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
