pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendAgentChatMessageResponseSessionMessagesItem {
    /// Present when the turn failed — the transcript keeps the user message and surfaces the failure instead of fabricating a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Canonical event ids this message ingested into the event store.
    #[serde(rename = "eventIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ids: Option<Vec<String>>,
    #[serde(rename = "messageId")]
    #[serde(default)]
    pub message_id: String,
    #[serde(rename = "occurredAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub occurred_at: DateTime<FixedOffset>,
    pub role: SendAgentChatMessageResponseSessionMessagesItemRole,
    /// Tool calls made while producing this message. Empty for user messages and failed turns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<SendAgentChatMessageResponseSessionMessagesItemStepsItem>>,
    #[serde(default)]
    pub text: String,
}

impl SendAgentChatMessageResponseSessionMessagesItem {
    pub fn builder() -> SendAgentChatMessageResponseSessionMessagesItemBuilder {
        <SendAgentChatMessageResponseSessionMessagesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendAgentChatMessageResponseSessionMessagesItemBuilder {
    error: Option<String>,
    event_ids: Option<Vec<String>>,
    message_id: Option<String>,
    occurred_at: Option<DateTime<FixedOffset>>,
    role: Option<SendAgentChatMessageResponseSessionMessagesItemRole>,
    steps: Option<Vec<SendAgentChatMessageResponseSessionMessagesItemStepsItem>>,
    text: Option<String>,
}

impl SendAgentChatMessageResponseSessionMessagesItemBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn event_ids(mut self, value: Vec<String>) -> Self {
        self.event_ids = Some(value);
        self
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn occurred_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.occurred_at = Some(value);
        self
    }

    pub fn role(mut self, value: SendAgentChatMessageResponseSessionMessagesItemRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn steps(
        mut self,
        value: Vec<SendAgentChatMessageResponseSessionMessagesItemStepsItem>,
    ) -> Self {
        self.steps = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SendAgentChatMessageResponseSessionMessagesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_id`](SendAgentChatMessageResponseSessionMessagesItemBuilder::message_id)
    /// - [`occurred_at`](SendAgentChatMessageResponseSessionMessagesItemBuilder::occurred_at)
    /// - [`role`](SendAgentChatMessageResponseSessionMessagesItemBuilder::role)
    /// - [`text`](SendAgentChatMessageResponseSessionMessagesItemBuilder::text)
    pub fn build(self) -> Result<SendAgentChatMessageResponseSessionMessagesItem, BuildError> {
        Ok(SendAgentChatMessageResponseSessionMessagesItem {
            error: self.error,
            event_ids: self.event_ids,
            message_id: self
                .message_id
                .ok_or_else(|| BuildError::missing_field("message_id"))?,
            occurred_at: self
                .occurred_at
                .ok_or_else(|| BuildError::missing_field("occurred_at"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            steps: self.steps,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
