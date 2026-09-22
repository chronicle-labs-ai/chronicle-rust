pub use crate::prelude::*;

/// One tool invocation the agent made while producing a chat turn. Mirrors what the executor reports from the agent's step telemetry; each step is also ingested as timeline events under the session trace (an `agent`/`tool.call` event plus a `source`/`eventType` result event).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SendAgentChatMessageResponseSessionMessagesItemStepsItem {
    #[serde(rename = "argsPreview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args_preview: Option<HashMap<String, serde_json::Value>>,
    /// Timeline event type of the tool result (e.g. `hotel-offers.search`).
    #[serde(rename = "eventType")]
    #[serde(default)]
    pub event_type: String,
    #[serde(rename = "resultPreview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_preview: Option<HashMap<String, serde_json::Value>>,
    /// Timeline source the tool call resolved against (e.g. `giata-sim`).
    #[serde(default)]
    pub source: String,
    #[serde(rename = "stepId")]
    #[serde(default)]
    pub step_id: String,
    /// Human-readable one-liner for the transcript and the timeline row.
    #[serde(default)]
    pub text: String,
    #[serde(rename = "toolName")]
    #[serde(default)]
    pub tool_name: String,
}

impl SendAgentChatMessageResponseSessionMessagesItemStepsItem {
    pub fn builder() -> SendAgentChatMessageResponseSessionMessagesItemStepsItemBuilder {
        <SendAgentChatMessageResponseSessionMessagesItemStepsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendAgentChatMessageResponseSessionMessagesItemStepsItemBuilder {
    args_preview: Option<HashMap<String, serde_json::Value>>,
    event_type: Option<String>,
    result_preview: Option<HashMap<String, serde_json::Value>>,
    source: Option<String>,
    step_id: Option<String>,
    text: Option<String>,
    tool_name: Option<String>,
}

impl SendAgentChatMessageResponseSessionMessagesItemStepsItemBuilder {
    pub fn args_preview(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args_preview = Some(value);
        self
    }

    pub fn event_type(mut self, value: impl Into<String>) -> Self {
        self.event_type = Some(value.into());
        self
    }

    pub fn result_preview(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.result_preview = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn step_id(mut self, value: impl Into<String>) -> Self {
        self.step_id = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn tool_name(mut self, value: impl Into<String>) -> Self {
        self.tool_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SendAgentChatMessageResponseSessionMessagesItemStepsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_type`](SendAgentChatMessageResponseSessionMessagesItemStepsItemBuilder::event_type)
    /// - [`source`](SendAgentChatMessageResponseSessionMessagesItemStepsItemBuilder::source)
    /// - [`step_id`](SendAgentChatMessageResponseSessionMessagesItemStepsItemBuilder::step_id)
    /// - [`text`](SendAgentChatMessageResponseSessionMessagesItemStepsItemBuilder::text)
    /// - [`tool_name`](SendAgentChatMessageResponseSessionMessagesItemStepsItemBuilder::tool_name)
    pub fn build(
        self,
    ) -> Result<SendAgentChatMessageResponseSessionMessagesItemStepsItem, BuildError> {
        Ok(SendAgentChatMessageResponseSessionMessagesItemStepsItem {
            args_preview: self.args_preview,
            event_type: self
                .event_type
                .ok_or_else(|| BuildError::missing_field("event_type"))?,
            result_preview: self.result_preview,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            step_id: self
                .step_id
                .ok_or_else(|| BuildError::missing_field("step_id"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            tool_name: self
                .tool_name
                .ok_or_else(|| BuildError::missing_field("tool_name"))?,
        })
    }
}
