pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentSnapshotRunsItemToolCallsItem {
    #[serde(rename = "argsHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args_hash: Option<String>,
    /// Optional small preview of the tool args.
    #[serde(rename = "argsPreview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args_preview: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "callId")]
    #[serde(default)]
    pub call_id: String,
    #[serde(rename = "durationMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<AgentSnapshotRunsItemToolCallsItemError>,
    #[serde(rename = "finishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "resultHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_hash: Option<String>,
    #[serde(rename = "resultPreview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_preview: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub started_at: DateTime<FixedOffset>,
    pub status: AgentSnapshotRunsItemToolCallsItemStatus,
    #[serde(rename = "toolName")]
    #[serde(default)]
    pub tool_name: String,
}

impl AgentSnapshotRunsItemToolCallsItem {
    pub fn builder() -> AgentSnapshotRunsItemToolCallsItemBuilder {
        <AgentSnapshotRunsItemToolCallsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSnapshotRunsItemToolCallsItemBuilder {
    args_hash: Option<String>,
    args_preview: Option<HashMap<String, serde_json::Value>>,
    call_id: Option<String>,
    duration_ms: Option<i64>,
    error: Option<AgentSnapshotRunsItemToolCallsItemError>,
    finished_at: Option<DateTime<FixedOffset>>,
    result_hash: Option<String>,
    result_preview: Option<HashMap<String, serde_json::Value>>,
    started_at: Option<DateTime<FixedOffset>>,
    status: Option<AgentSnapshotRunsItemToolCallsItemStatus>,
    tool_name: Option<String>,
}

impl AgentSnapshotRunsItemToolCallsItemBuilder {
    pub fn args_hash(mut self, value: impl Into<String>) -> Self {
        self.args_hash = Some(value.into());
        self
    }

    pub fn args_preview(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args_preview = Some(value);
        self
    }

    pub fn call_id(mut self, value: impl Into<String>) -> Self {
        self.call_id = Some(value.into());
        self
    }

    pub fn duration_ms(mut self, value: i64) -> Self {
        self.duration_ms = Some(value);
        self
    }

    pub fn error(mut self, value: AgentSnapshotRunsItemToolCallsItemError) -> Self {
        self.error = Some(value);
        self
    }

    pub fn finished_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.finished_at = Some(value);
        self
    }

    pub fn result_hash(mut self, value: impl Into<String>) -> Self {
        self.result_hash = Some(value.into());
        self
    }

    pub fn result_preview(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.result_preview = Some(value);
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn status(mut self, value: AgentSnapshotRunsItemToolCallsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tool_name(mut self, value: impl Into<String>) -> Self {
        self.tool_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentSnapshotRunsItemToolCallsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`call_id`](AgentSnapshotRunsItemToolCallsItemBuilder::call_id)
    /// - [`started_at`](AgentSnapshotRunsItemToolCallsItemBuilder::started_at)
    /// - [`status`](AgentSnapshotRunsItemToolCallsItemBuilder::status)
    /// - [`tool_name`](AgentSnapshotRunsItemToolCallsItemBuilder::tool_name)
    pub fn build(self) -> Result<AgentSnapshotRunsItemToolCallsItem, BuildError> {
        Ok(AgentSnapshotRunsItemToolCallsItem {
            args_hash: self.args_hash,
            args_preview: self.args_preview,
            call_id: self
                .call_id
                .ok_or_else(|| BuildError::missing_field("call_id"))?,
            duration_ms: self.duration_ms,
            error: self.error,
            finished_at: self.finished_at,
            result_hash: self.result_hash,
            result_preview: self.result_preview,
            started_at: self
                .started_at
                .ok_or_else(|| BuildError::missing_field("started_at"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            tool_name: self
                .tool_name
                .ok_or_else(|| BuildError::missing_field("tool_name"))?,
        })
    }
}
