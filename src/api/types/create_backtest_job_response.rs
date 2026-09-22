pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateBacktestJobResponse {
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
    /// Compact projection of a backtest run rendered on the list view. Combines the recipe identity (mode, environment, agents, dataset) with run lifecycle metadata (status, verdict, divergences).
    pub run: CreateBacktestJobResponseRun,
}

impl CreateBacktestJobResponse {
    pub fn builder() -> CreateBacktestJobResponseBuilder {
        <CreateBacktestJobResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBacktestJobResponseBuilder {
    job_id: Option<String>,
    run: Option<CreateBacktestJobResponseRun>,
}

impl CreateBacktestJobResponseBuilder {
    pub fn job_id(mut self, value: impl Into<String>) -> Self {
        self.job_id = Some(value.into());
        self
    }

    pub fn run(mut self, value: CreateBacktestJobResponseRun) -> Self {
        self.run = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateBacktestJobResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`job_id`](CreateBacktestJobResponseBuilder::job_id)
    /// - [`run`](CreateBacktestJobResponseBuilder::run)
    pub fn build(self) -> Result<CreateBacktestJobResponse, BuildError> {
        Ok(CreateBacktestJobResponse {
            job_id: self
                .job_id
                .ok_or_else(|| BuildError::missing_field("job_id"))?,
            run: self.run.ok_or_else(|| BuildError::missing_field("run"))?,
        })
    }
}
