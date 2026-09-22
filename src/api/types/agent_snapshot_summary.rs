pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentSnapshotSummary {
    #[serde(rename = "capabilityTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// Framework label. Multi-word kebab-case to match the existing TS union (`vercel-ai-sdk`, `openai-agents-python`, etc.).
    pub framework: AgentSnapshotSummaryFramework,
    /// Last drift event, if any.
    #[serde(rename = "lastDriftAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_drift_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "lastRunAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "latestVersion")]
    #[serde(default)]
    pub latest_version: String,
    #[serde(default)]
    pub model: AgentSnapshotSummaryModel,
    #[serde(rename = "modelLabel")]
    #[serde(default)]
    pub model_label: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "personaSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persona_summary: Option<String>,
    #[serde(rename = "playgroundUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playground_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(rename = "runbookUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runbook_url: Option<String>,
    #[serde(rename = "successRate")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub success_rate: f64,
    #[serde(rename = "totalRuns")]
    #[serde(default)]
    pub total_runs: i64,
    #[serde(rename = "versionCount")]
    #[serde(default)]
    pub version_count: i64,
}

impl AgentSnapshotSummary {
    pub fn builder() -> AgentSnapshotSummaryBuilder {
        <AgentSnapshotSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSnapshotSummaryBuilder {
    capability_tags: Option<Vec<String>>,
    category: Option<String>,
    description: Option<String>,
    environment: Option<String>,
    framework: Option<AgentSnapshotSummaryFramework>,
    last_drift_at: Option<DateTime<FixedOffset>>,
    last_run_at: Option<DateTime<FixedOffset>>,
    latest_version: Option<String>,
    model: Option<AgentSnapshotSummaryModel>,
    model_label: Option<String>,
    name: Option<String>,
    owner: Option<String>,
    persona_summary: Option<String>,
    playground_url: Option<String>,
    purpose: Option<String>,
    runbook_url: Option<String>,
    success_rate: Option<f64>,
    total_runs: Option<i64>,
    version_count: Option<i64>,
}

impl AgentSnapshotSummaryBuilder {
    pub fn capability_tags(mut self, value: Vec<String>) -> Self {
        self.capability_tags = Some(value);
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn framework(mut self, value: AgentSnapshotSummaryFramework) -> Self {
        self.framework = Some(value);
        self
    }

    pub fn last_drift_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_drift_at = Some(value);
        self
    }

    pub fn last_run_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_run_at = Some(value);
        self
    }

    pub fn latest_version(mut self, value: impl Into<String>) -> Self {
        self.latest_version = Some(value.into());
        self
    }

    pub fn model(mut self, value: AgentSnapshotSummaryModel) -> Self {
        self.model = Some(value);
        self
    }

    pub fn model_label(mut self, value: impl Into<String>) -> Self {
        self.model_label = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn owner(mut self, value: impl Into<String>) -> Self {
        self.owner = Some(value.into());
        self
    }

    pub fn persona_summary(mut self, value: impl Into<String>) -> Self {
        self.persona_summary = Some(value.into());
        self
    }

    pub fn playground_url(mut self, value: impl Into<String>) -> Self {
        self.playground_url = Some(value.into());
        self
    }

    pub fn purpose(mut self, value: impl Into<String>) -> Self {
        self.purpose = Some(value.into());
        self
    }

    pub fn runbook_url(mut self, value: impl Into<String>) -> Self {
        self.runbook_url = Some(value.into());
        self
    }

    pub fn success_rate(mut self, value: f64) -> Self {
        self.success_rate = Some(value);
        self
    }

    pub fn total_runs(mut self, value: i64) -> Self {
        self.total_runs = Some(value);
        self
    }

    pub fn version_count(mut self, value: i64) -> Self {
        self.version_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentSnapshotSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`framework`](AgentSnapshotSummaryBuilder::framework)
    /// - [`latest_version`](AgentSnapshotSummaryBuilder::latest_version)
    /// - [`model`](AgentSnapshotSummaryBuilder::model)
    /// - [`model_label`](AgentSnapshotSummaryBuilder::model_label)
    /// - [`name`](AgentSnapshotSummaryBuilder::name)
    /// - [`success_rate`](AgentSnapshotSummaryBuilder::success_rate)
    /// - [`total_runs`](AgentSnapshotSummaryBuilder::total_runs)
    /// - [`version_count`](AgentSnapshotSummaryBuilder::version_count)
    pub fn build(self) -> Result<AgentSnapshotSummary, BuildError> {
        Ok(AgentSnapshotSummary {
            capability_tags: self.capability_tags,
            category: self.category,
            description: self.description,
            environment: self.environment,
            framework: self
                .framework
                .ok_or_else(|| BuildError::missing_field("framework"))?,
            last_drift_at: self.last_drift_at,
            last_run_at: self.last_run_at,
            latest_version: self
                .latest_version
                .ok_or_else(|| BuildError::missing_field("latest_version"))?,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            model_label: self
                .model_label
                .ok_or_else(|| BuildError::missing_field("model_label"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            owner: self.owner,
            persona_summary: self.persona_summary,
            playground_url: self.playground_url,
            purpose: self.purpose,
            runbook_url: self.runbook_url,
            success_rate: self
                .success_rate
                .ok_or_else(|| BuildError::missing_field("success_rate"))?,
            total_runs: self
                .total_runs
                .ok_or_else(|| BuildError::missing_field("total_runs"))?,
            version_count: self
                .version_count
                .ok_or_else(|| BuildError::missing_field("version_count"))?,
        })
    }
}
