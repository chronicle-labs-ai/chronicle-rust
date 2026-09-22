pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecordAgentRunsRequestRunsItem {
    #[serde(rename = "artifactId")]
    #[serde(default)]
    pub artifact_id: String,
    #[serde(rename = "callOptionsHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_options_hash: Option<String>,
    #[serde(rename = "configHash")]
    #[serde(default)]
    pub config_hash: String,
    #[serde(rename = "durationMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RecordAgentRunsRequestRunsItemError>,
    #[serde(rename = "finishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "inputHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_hash: Option<String>,
    pub operation: RecordAgentRunsRequestRunsItemOperation,
    #[serde(rename = "preparedCall")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepared_call: Option<RecordAgentRunsRequestRunsItemPreparedCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<RecordAgentRunsRequestRunsItemResponse>,
    #[serde(rename = "runId")]
    #[serde(default)]
    pub run_id: String,
    #[serde(rename = "schemaVersion")]
    #[serde(default)]
    pub schema_version: String,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub started_at: DateTime<FixedOffset>,
    pub status: RecordAgentRunsRequestRunsItemStatus,
    #[serde(rename = "toolCalls")]
    #[serde(default)]
    pub tool_calls: Vec<RecordAgentRunsRequestRunsItemToolCallsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<HashMap<String, Option<String>>>,
}

impl RecordAgentRunsRequestRunsItem {
    pub fn builder() -> RecordAgentRunsRequestRunsItemBuilder {
        <RecordAgentRunsRequestRunsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecordAgentRunsRequestRunsItemBuilder {
    artifact_id: Option<String>,
    call_options_hash: Option<String>,
    config_hash: Option<String>,
    duration_ms: Option<i64>,
    error: Option<RecordAgentRunsRequestRunsItemError>,
    finished_at: Option<DateTime<FixedOffset>>,
    input_hash: Option<String>,
    operation: Option<RecordAgentRunsRequestRunsItemOperation>,
    prepared_call: Option<RecordAgentRunsRequestRunsItemPreparedCall>,
    response: Option<RecordAgentRunsRequestRunsItemResponse>,
    run_id: Option<String>,
    schema_version: Option<String>,
    started_at: Option<DateTime<FixedOffset>>,
    status: Option<RecordAgentRunsRequestRunsItemStatus>,
    tool_calls: Option<Vec<RecordAgentRunsRequestRunsItemToolCallsItem>>,
    trace: Option<HashMap<String, Option<String>>>,
}

impl RecordAgentRunsRequestRunsItemBuilder {
    pub fn artifact_id(mut self, value: impl Into<String>) -> Self {
        self.artifact_id = Some(value.into());
        self
    }

    pub fn call_options_hash(mut self, value: impl Into<String>) -> Self {
        self.call_options_hash = Some(value.into());
        self
    }

    pub fn config_hash(mut self, value: impl Into<String>) -> Self {
        self.config_hash = Some(value.into());
        self
    }

    pub fn duration_ms(mut self, value: i64) -> Self {
        self.duration_ms = Some(value);
        self
    }

    pub fn error(mut self, value: RecordAgentRunsRequestRunsItemError) -> Self {
        self.error = Some(value);
        self
    }

    pub fn finished_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.finished_at = Some(value);
        self
    }

    pub fn input_hash(mut self, value: impl Into<String>) -> Self {
        self.input_hash = Some(value.into());
        self
    }

    pub fn operation(mut self, value: RecordAgentRunsRequestRunsItemOperation) -> Self {
        self.operation = Some(value);
        self
    }

    pub fn prepared_call(mut self, value: RecordAgentRunsRequestRunsItemPreparedCall) -> Self {
        self.prepared_call = Some(value);
        self
    }

    pub fn response(mut self, value: RecordAgentRunsRequestRunsItemResponse) -> Self {
        self.response = Some(value);
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn schema_version(mut self, value: impl Into<String>) -> Self {
        self.schema_version = Some(value.into());
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn status(mut self, value: RecordAgentRunsRequestRunsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tool_calls(mut self, value: Vec<RecordAgentRunsRequestRunsItemToolCallsItem>) -> Self {
        self.tool_calls = Some(value);
        self
    }

    pub fn trace(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.trace = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecordAgentRunsRequestRunsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`artifact_id`](RecordAgentRunsRequestRunsItemBuilder::artifact_id)
    /// - [`config_hash`](RecordAgentRunsRequestRunsItemBuilder::config_hash)
    /// - [`operation`](RecordAgentRunsRequestRunsItemBuilder::operation)
    /// - [`run_id`](RecordAgentRunsRequestRunsItemBuilder::run_id)
    /// - [`schema_version`](RecordAgentRunsRequestRunsItemBuilder::schema_version)
    /// - [`started_at`](RecordAgentRunsRequestRunsItemBuilder::started_at)
    /// - [`status`](RecordAgentRunsRequestRunsItemBuilder::status)
    /// - [`tool_calls`](RecordAgentRunsRequestRunsItemBuilder::tool_calls)
    pub fn build(self) -> Result<RecordAgentRunsRequestRunsItem, BuildError> {
        Ok(RecordAgentRunsRequestRunsItem {
            artifact_id: self
                .artifact_id
                .ok_or_else(|| BuildError::missing_field("artifact_id"))?,
            call_options_hash: self.call_options_hash,
            config_hash: self
                .config_hash
                .ok_or_else(|| BuildError::missing_field("config_hash"))?,
            duration_ms: self.duration_ms,
            error: self.error,
            finished_at: self.finished_at,
            input_hash: self.input_hash,
            operation: self
                .operation
                .ok_or_else(|| BuildError::missing_field("operation"))?,
            prepared_call: self.prepared_call,
            response: self.response,
            run_id: self
                .run_id
                .ok_or_else(|| BuildError::missing_field("run_id"))?,
            schema_version: self
                .schema_version
                .ok_or_else(|| BuildError::missing_field("schema_version"))?,
            started_at: self
                .started_at
                .ok_or_else(|| BuildError::missing_field("started_at"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            tool_calls: self
                .tool_calls
                .ok_or_else(|| BuildError::missing_field("tool_calls"))?,
            trace: self.trace,
        })
    }
}
