pub use crate::prelude::*;

/// Row projection of `"BacktestTrial"`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListBacktestJobTrialsResponseTrialsItem {
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
    #[serde(rename = "agentLabel")]
    #[serde(default)]
    pub agent_label: String,
    #[serde(default)]
    pub attempt: i64,
    #[serde(rename = "caseCluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_cluster: Option<String>,
    #[serde(rename = "caseId")]
    #[serde(default)]
    pub case_id: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "durationMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<i64>,
    /// Captured exception info for a failed trial. Mirrors Harbor's `ExceptionInfo` — kind for routing/retry, message for humans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception: Option<ListBacktestJobTrialsResponseTrialsItemException>,
    #[serde(default)]
    pub id: String,
    /// The case instruction this trial executed — persisted at launch so result surfaces can show the real prompt instead of reconstructing (or inventing) one client-side.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction: Option<String>,
    #[serde(rename = "isBaseline")]
    #[serde(default)]
    pub is_baseline: bool,
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "sandboxId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox_id: Option<String>,
    /// Lifecycle state of a single `BacktestTrial` (one (case × agent) cell).
    pub status: ListBacktestJobTrialsResponseTrialsItemStatus,
    #[serde(rename = "tenantId")]
    #[serde(default)]
    pub tenant_id: String,
    /// Per-phase timing bookkeeping. Each pair is `(started_at, finished_at)`; `None` until that phase starts/ends.
    #[serde(default)]
    pub timings: ListBacktestJobTrialsResponseTrialsItemTimings,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl ListBacktestJobTrialsResponseTrialsItem {
    pub fn builder() -> ListBacktestJobTrialsResponseTrialsItemBuilder {
        <ListBacktestJobTrialsResponseTrialsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBacktestJobTrialsResponseTrialsItemBuilder {
    agent_id: Option<String>,
    agent_label: Option<String>,
    attempt: Option<i64>,
    case_cluster: Option<String>,
    case_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    duration_ms: Option<i64>,
    exception: Option<ListBacktestJobTrialsResponseTrialsItemException>,
    id: Option<String>,
    instruction: Option<String>,
    is_baseline: Option<bool>,
    job_id: Option<String>,
    sandbox_id: Option<String>,
    status: Option<ListBacktestJobTrialsResponseTrialsItemStatus>,
    tenant_id: Option<String>,
    timings: Option<ListBacktestJobTrialsResponseTrialsItemTimings>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ListBacktestJobTrialsResponseTrialsItemBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn agent_label(mut self, value: impl Into<String>) -> Self {
        self.agent_label = Some(value.into());
        self
    }

    pub fn attempt(mut self, value: i64) -> Self {
        self.attempt = Some(value);
        self
    }

    pub fn case_cluster(mut self, value: impl Into<String>) -> Self {
        self.case_cluster = Some(value.into());
        self
    }

    pub fn case_id(mut self, value: impl Into<String>) -> Self {
        self.case_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn duration_ms(mut self, value: i64) -> Self {
        self.duration_ms = Some(value);
        self
    }

    pub fn exception(mut self, value: ListBacktestJobTrialsResponseTrialsItemException) -> Self {
        self.exception = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn instruction(mut self, value: impl Into<String>) -> Self {
        self.instruction = Some(value.into());
        self
    }

    pub fn is_baseline(mut self, value: bool) -> Self {
        self.is_baseline = Some(value);
        self
    }

    pub fn job_id(mut self, value: impl Into<String>) -> Self {
        self.job_id = Some(value.into());
        self
    }

    pub fn sandbox_id(mut self, value: impl Into<String>) -> Self {
        self.sandbox_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListBacktestJobTrialsResponseTrialsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tenant_id(mut self, value: impl Into<String>) -> Self {
        self.tenant_id = Some(value.into());
        self
    }

    pub fn timings(mut self, value: ListBacktestJobTrialsResponseTrialsItemTimings) -> Self {
        self.timings = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListBacktestJobTrialsResponseTrialsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](ListBacktestJobTrialsResponseTrialsItemBuilder::agent_id)
    /// - [`agent_label`](ListBacktestJobTrialsResponseTrialsItemBuilder::agent_label)
    /// - [`attempt`](ListBacktestJobTrialsResponseTrialsItemBuilder::attempt)
    /// - [`case_id`](ListBacktestJobTrialsResponseTrialsItemBuilder::case_id)
    /// - [`created_at`](ListBacktestJobTrialsResponseTrialsItemBuilder::created_at)
    /// - [`id`](ListBacktestJobTrialsResponseTrialsItemBuilder::id)
    /// - [`is_baseline`](ListBacktestJobTrialsResponseTrialsItemBuilder::is_baseline)
    /// - [`job_id`](ListBacktestJobTrialsResponseTrialsItemBuilder::job_id)
    /// - [`status`](ListBacktestJobTrialsResponseTrialsItemBuilder::status)
    /// - [`tenant_id`](ListBacktestJobTrialsResponseTrialsItemBuilder::tenant_id)
    /// - [`timings`](ListBacktestJobTrialsResponseTrialsItemBuilder::timings)
    /// - [`updated_at`](ListBacktestJobTrialsResponseTrialsItemBuilder::updated_at)
    pub fn build(self) -> Result<ListBacktestJobTrialsResponseTrialsItem, BuildError> {
        Ok(ListBacktestJobTrialsResponseTrialsItem {
            agent_id: self
                .agent_id
                .ok_or_else(|| BuildError::missing_field("agent_id"))?,
            agent_label: self
                .agent_label
                .ok_or_else(|| BuildError::missing_field("agent_label"))?,
            attempt: self
                .attempt
                .ok_or_else(|| BuildError::missing_field("attempt"))?,
            case_cluster: self.case_cluster,
            case_id: self
                .case_id
                .ok_or_else(|| BuildError::missing_field("case_id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            duration_ms: self.duration_ms,
            exception: self.exception,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            instruction: self.instruction,
            is_baseline: self
                .is_baseline
                .ok_or_else(|| BuildError::missing_field("is_baseline"))?,
            job_id: self
                .job_id
                .ok_or_else(|| BuildError::missing_field("job_id"))?,
            sandbox_id: self.sandbox_id,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            tenant_id: self
                .tenant_id
                .ok_or_else(|| BuildError::missing_field("tenant_id"))?,
            timings: self
                .timings
                .ok_or_else(|| BuildError::missing_field("timings"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
