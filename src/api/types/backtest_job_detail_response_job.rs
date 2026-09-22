pub use crate::prelude::*;

/// Row projection of `"BacktestJob"`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BacktestJobDetailResponseJob {
    #[serde(rename = "completedTrials")]
    #[serde(default)]
    pub completed_trials: i64,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// Mutable Dataset selected when the job was launched.
    #[serde(rename = "datasetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_id: Option<String>,
    /// Immutable Dataset Version used to compile this job's cases. Once set, later working-copy edits cannot alter the run inputs.
    #[serde(rename = "datasetVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_version_id: Option<String>,
    #[serde(rename = "exceptionKind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception_kind: Option<String>,
    #[serde(rename = "failedTrials")]
    #[serde(default)]
    pub failed_trials: i64,
    #[serde(rename = "finishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub id: String,
    pub mode: BacktestJobDetailResponseJobMode,
    #[serde(rename = "nConcurrent")]
    #[serde(default)]
    pub n_concurrent: i64,
    #[serde(default)]
    pub name: String,
    pub recipe: serde_json::Value,
    /// Retry policy applied to transient trial failures (network errors, sandbox-create timeouts). Mirrors Harbor's `RetryConfig`. Pass `null` on the wire (`None` here) to fall back to defaults.
    #[serde(rename = "retryConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_config: Option<BacktestJobDetailResponseJobRetryConfig>,
    /// Sandbox driver selected by trusted server policy at submit time. Echoed onto the `BacktestJob.sandboxDriver` column so persisted jobs record which implementation backed the `Sandbox` trait.
    #[serde(rename = "sandboxDriver")]
    pub sandbox_driver: BacktestJobDetailResponseJobSandboxDriver,
    #[serde(rename = "scheduledFor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_for: Option<DateTime<FixedOffset>>,
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<FixedOffset>>,
    /// Lifecycle state of an entire `BacktestJob`. Maps 1:1 onto the `status` column in `migrations/013_create_backtest_runtime.sql`.
    pub status: BacktestJobDetailResponseJobStatus,
    #[serde(rename = "tenantId")]
    #[serde(default)]
    pub tenant_id: String,
    #[serde(rename = "totalTrials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_trials: Option<i64>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verdict: Option<String>,
}

impl BacktestJobDetailResponseJob {
    pub fn builder() -> BacktestJobDetailResponseJobBuilder {
        <BacktestJobDetailResponseJobBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BacktestJobDetailResponseJobBuilder {
    completed_trials: Option<i64>,
    created_at: Option<DateTime<FixedOffset>>,
    created_by: Option<String>,
    dataset_id: Option<String>,
    dataset_version_id: Option<String>,
    exception_kind: Option<String>,
    failed_trials: Option<i64>,
    finished_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    mode: Option<BacktestJobDetailResponseJobMode>,
    n_concurrent: Option<i64>,
    name: Option<String>,
    recipe: Option<serde_json::Value>,
    retry_config: Option<BacktestJobDetailResponseJobRetryConfig>,
    sandbox_driver: Option<BacktestJobDetailResponseJobSandboxDriver>,
    scheduled_for: Option<DateTime<FixedOffset>>,
    started_at: Option<DateTime<FixedOffset>>,
    status: Option<BacktestJobDetailResponseJobStatus>,
    tenant_id: Option<String>,
    total_trials: Option<i64>,
    updated_at: Option<DateTime<FixedOffset>>,
    verdict: Option<String>,
}

impl BacktestJobDetailResponseJobBuilder {
    pub fn completed_trials(mut self, value: i64) -> Self {
        self.completed_trials = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn dataset_id(mut self, value: impl Into<String>) -> Self {
        self.dataset_id = Some(value.into());
        self
    }

    pub fn dataset_version_id(mut self, value: impl Into<String>) -> Self {
        self.dataset_version_id = Some(value.into());
        self
    }

    pub fn exception_kind(mut self, value: impl Into<String>) -> Self {
        self.exception_kind = Some(value.into());
        self
    }

    pub fn failed_trials(mut self, value: i64) -> Self {
        self.failed_trials = Some(value);
        self
    }

    pub fn finished_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.finished_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn mode(mut self, value: BacktestJobDetailResponseJobMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn n_concurrent(mut self, value: i64) -> Self {
        self.n_concurrent = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn recipe(mut self, value: serde_json::Value) -> Self {
        self.recipe = Some(value);
        self
    }

    pub fn retry_config(mut self, value: BacktestJobDetailResponseJobRetryConfig) -> Self {
        self.retry_config = Some(value);
        self
    }

    pub fn sandbox_driver(mut self, value: BacktestJobDetailResponseJobSandboxDriver) -> Self {
        self.sandbox_driver = Some(value);
        self
    }

    pub fn scheduled_for(mut self, value: DateTime<FixedOffset>) -> Self {
        self.scheduled_for = Some(value);
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn status(mut self, value: BacktestJobDetailResponseJobStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tenant_id(mut self, value: impl Into<String>) -> Self {
        self.tenant_id = Some(value.into());
        self
    }

    pub fn total_trials(mut self, value: i64) -> Self {
        self.total_trials = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn verdict(mut self, value: impl Into<String>) -> Self {
        self.verdict = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BacktestJobDetailResponseJob`].
    /// This method will fail if any of the following fields are not set:
    /// - [`completed_trials`](BacktestJobDetailResponseJobBuilder::completed_trials)
    /// - [`created_at`](BacktestJobDetailResponseJobBuilder::created_at)
    /// - [`failed_trials`](BacktestJobDetailResponseJobBuilder::failed_trials)
    /// - [`id`](BacktestJobDetailResponseJobBuilder::id)
    /// - [`mode`](BacktestJobDetailResponseJobBuilder::mode)
    /// - [`n_concurrent`](BacktestJobDetailResponseJobBuilder::n_concurrent)
    /// - [`name`](BacktestJobDetailResponseJobBuilder::name)
    /// - [`recipe`](BacktestJobDetailResponseJobBuilder::recipe)
    /// - [`sandbox_driver`](BacktestJobDetailResponseJobBuilder::sandbox_driver)
    /// - [`status`](BacktestJobDetailResponseJobBuilder::status)
    /// - [`tenant_id`](BacktestJobDetailResponseJobBuilder::tenant_id)
    /// - [`updated_at`](BacktestJobDetailResponseJobBuilder::updated_at)
    pub fn build(self) -> Result<BacktestJobDetailResponseJob, BuildError> {
        Ok(BacktestJobDetailResponseJob {
            completed_trials: self
                .completed_trials
                .ok_or_else(|| BuildError::missing_field("completed_trials"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            created_by: self.created_by,
            dataset_id: self.dataset_id,
            dataset_version_id: self.dataset_version_id,
            exception_kind: self.exception_kind,
            failed_trials: self
                .failed_trials
                .ok_or_else(|| BuildError::missing_field("failed_trials"))?,
            finished_at: self.finished_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
            n_concurrent: self
                .n_concurrent
                .ok_or_else(|| BuildError::missing_field("n_concurrent"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            recipe: self
                .recipe
                .ok_or_else(|| BuildError::missing_field("recipe"))?,
            retry_config: self.retry_config,
            sandbox_driver: self
                .sandbox_driver
                .ok_or_else(|| BuildError::missing_field("sandbox_driver"))?,
            scheduled_for: self.scheduled_for,
            started_at: self.started_at,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            tenant_id: self
                .tenant_id
                .ok_or_else(|| BuildError::missing_field("tenant_id"))?,
            total_trials: self.total_trials,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            verdict: self.verdict,
        })
    }
}
