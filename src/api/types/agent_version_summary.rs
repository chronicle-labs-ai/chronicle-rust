pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentVersionSummary {
    pub artifact: AgentVersionSummaryArtifact,
    #[serde(rename = "lastRunAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "meanDurationMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean_duration_ms: Option<i64>,
    #[serde(rename = "p95DurationMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p95duration_ms: Option<i64>,
    #[serde(rename = "resolvedModelIds")]
    #[serde(default)]
    pub resolved_model_ids: Vec<String>,
    #[serde(rename = "runCount")]
    #[serde(default)]
    pub run_count: i64,
    pub status: AgentVersionSummaryStatus,
    /// Successful runs / total runs (0..1).
    #[serde(rename = "successRate")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub success_rate: f64,
    #[serde(rename = "totalTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<i64>,
}

impl AgentVersionSummary {
    pub fn builder() -> AgentVersionSummaryBuilder {
        <AgentVersionSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentVersionSummaryBuilder {
    artifact: Option<AgentVersionSummaryArtifact>,
    last_run_at: Option<DateTime<FixedOffset>>,
    mean_duration_ms: Option<i64>,
    p95duration_ms: Option<i64>,
    resolved_model_ids: Option<Vec<String>>,
    run_count: Option<i64>,
    status: Option<AgentVersionSummaryStatus>,
    success_rate: Option<f64>,
    total_tokens: Option<i64>,
}

impl AgentVersionSummaryBuilder {
    pub fn artifact(mut self, value: AgentVersionSummaryArtifact) -> Self {
        self.artifact = Some(value);
        self
    }

    pub fn last_run_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_run_at = Some(value);
        self
    }

    pub fn mean_duration_ms(mut self, value: i64) -> Self {
        self.mean_duration_ms = Some(value);
        self
    }

    pub fn p95duration_ms(mut self, value: i64) -> Self {
        self.p95duration_ms = Some(value);
        self
    }

    pub fn resolved_model_ids(mut self, value: Vec<String>) -> Self {
        self.resolved_model_ids = Some(value);
        self
    }

    pub fn run_count(mut self, value: i64) -> Self {
        self.run_count = Some(value);
        self
    }

    pub fn status(mut self, value: AgentVersionSummaryStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn success_rate(mut self, value: f64) -> Self {
        self.success_rate = Some(value);
        self
    }

    pub fn total_tokens(mut self, value: i64) -> Self {
        self.total_tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentVersionSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`artifact`](AgentVersionSummaryBuilder::artifact)
    /// - [`resolved_model_ids`](AgentVersionSummaryBuilder::resolved_model_ids)
    /// - [`run_count`](AgentVersionSummaryBuilder::run_count)
    /// - [`status`](AgentVersionSummaryBuilder::status)
    /// - [`success_rate`](AgentVersionSummaryBuilder::success_rate)
    pub fn build(self) -> Result<AgentVersionSummary, BuildError> {
        Ok(AgentVersionSummary {
            artifact: self
                .artifact
                .ok_or_else(|| BuildError::missing_field("artifact"))?,
            last_run_at: self.last_run_at,
            mean_duration_ms: self.mean_duration_ms,
            p95duration_ms: self.p95duration_ms,
            resolved_model_ids: self
                .resolved_model_ids
                .ok_or_else(|| BuildError::missing_field("resolved_model_ids"))?,
            run_count: self
                .run_count
                .ok_or_else(|| BuildError::missing_field("run_count"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            success_rate: self
                .success_rate
                .ok_or_else(|| BuildError::missing_field("success_rate"))?,
            total_tokens: self.total_tokens,
        })
    }
}
