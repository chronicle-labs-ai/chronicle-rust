pub use crate::prelude::*;

/// Compact projection of a backtest run rendered on the list view. Combines the recipe identity (mode, environment, agents, dataset) with run lifecycle metadata (status, verdict, divergences).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListBacktestJobsResponseRunsItem {
    /// Agent ids participating in the run; the first is the baseline.
    #[serde(rename = "agentIds")]
    #[serde(default)]
    pub agent_ids: Vec<String>,
    /// Display label for the dataset / production window seed.
    #[serde(rename = "datasetLabel")]
    #[serde(default)]
    pub dataset_label: String,
    /// Divergences observed (only set when status is `done` / `failed`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub divergences: Option<i64>,
    #[serde(rename = "environmentLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hue: Option<String>,
    #[serde(default)]
    pub id: String,
    pub mode: ListBacktestJobsResponseRunsItemMode,
    /// Display name of the run (matches `BacktestRecipe.name`).
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "scheduledFor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_for: Option<DateTime<FixedOffset>>,
    pub status: ListBacktestJobsResponseRunsItemStatus,
    /// Total cases × agents; null while still drafting.
    #[serde(rename = "totalRuns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_runs: Option<i64>,
    /// ISO timestamp of the most recent state change — drives "ago".
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verdict: Option<String>,
}

impl ListBacktestJobsResponseRunsItem {
    pub fn builder() -> ListBacktestJobsResponseRunsItemBuilder {
        <ListBacktestJobsResponseRunsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBacktestJobsResponseRunsItemBuilder {
    agent_ids: Option<Vec<String>>,
    dataset_label: Option<String>,
    divergences: Option<i64>,
    environment_label: Option<String>,
    hue: Option<String>,
    id: Option<String>,
    mode: Option<ListBacktestJobsResponseRunsItemMode>,
    name: Option<String>,
    owner: Option<String>,
    scheduled_for: Option<DateTime<FixedOffset>>,
    status: Option<ListBacktestJobsResponseRunsItemStatus>,
    total_runs: Option<i64>,
    updated_at: Option<DateTime<FixedOffset>>,
    verdict: Option<String>,
}

impl ListBacktestJobsResponseRunsItemBuilder {
    pub fn agent_ids(mut self, value: Vec<String>) -> Self {
        self.agent_ids = Some(value);
        self
    }

    pub fn dataset_label(mut self, value: impl Into<String>) -> Self {
        self.dataset_label = Some(value.into());
        self
    }

    pub fn divergences(mut self, value: i64) -> Self {
        self.divergences = Some(value);
        self
    }

    pub fn environment_label(mut self, value: impl Into<String>) -> Self {
        self.environment_label = Some(value.into());
        self
    }

    pub fn hue(mut self, value: impl Into<String>) -> Self {
        self.hue = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn mode(mut self, value: ListBacktestJobsResponseRunsItemMode) -> Self {
        self.mode = Some(value);
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

    pub fn scheduled_for(mut self, value: DateTime<FixedOffset>) -> Self {
        self.scheduled_for = Some(value);
        self
    }

    pub fn status(mut self, value: ListBacktestJobsResponseRunsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn total_runs(mut self, value: i64) -> Self {
        self.total_runs = Some(value);
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

    /// Consumes the builder and constructs a [`ListBacktestJobsResponseRunsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_ids`](ListBacktestJobsResponseRunsItemBuilder::agent_ids)
    /// - [`dataset_label`](ListBacktestJobsResponseRunsItemBuilder::dataset_label)
    /// - [`id`](ListBacktestJobsResponseRunsItemBuilder::id)
    /// - [`mode`](ListBacktestJobsResponseRunsItemBuilder::mode)
    /// - [`name`](ListBacktestJobsResponseRunsItemBuilder::name)
    /// - [`status`](ListBacktestJobsResponseRunsItemBuilder::status)
    /// - [`updated_at`](ListBacktestJobsResponseRunsItemBuilder::updated_at)
    pub fn build(self) -> Result<ListBacktestJobsResponseRunsItem, BuildError> {
        Ok(ListBacktestJobsResponseRunsItem {
            agent_ids: self
                .agent_ids
                .ok_or_else(|| BuildError::missing_field("agent_ids"))?,
            dataset_label: self
                .dataset_label
                .ok_or_else(|| BuildError::missing_field("dataset_label"))?,
            divergences: self.divergences,
            environment_label: self.environment_label,
            hue: self.hue,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            owner: self.owner,
            scheduled_for: self.scheduled_for,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            total_runs: self.total_runs,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            verdict: self.verdict,
        })
    }
}
