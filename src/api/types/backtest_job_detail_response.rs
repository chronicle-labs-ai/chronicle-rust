pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BacktestJobDetailResponse {
    /// Row projection of `"BacktestJob"`.
    pub job: BacktestJobDetailResponseJob,
    /// Compact projection of a backtest run rendered on the list view. Combines the recipe identity (mode, environment, agents, dataset) with run lifecycle metadata (status, verdict, divergences).
    pub run: BacktestJobDetailResponseRun,
}

impl BacktestJobDetailResponse {
    pub fn builder() -> BacktestJobDetailResponseBuilder {
        <BacktestJobDetailResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BacktestJobDetailResponseBuilder {
    job: Option<BacktestJobDetailResponseJob>,
    run: Option<BacktestJobDetailResponseRun>,
}

impl BacktestJobDetailResponseBuilder {
    pub fn job(mut self, value: BacktestJobDetailResponseJob) -> Self {
        self.job = Some(value);
        self
    }

    pub fn run(mut self, value: BacktestJobDetailResponseRun) -> Self {
        self.run = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BacktestJobDetailResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`job`](BacktestJobDetailResponseBuilder::job)
    /// - [`run`](BacktestJobDetailResponseBuilder::run)
    pub fn build(self) -> Result<BacktestJobDetailResponse, BuildError> {
        Ok(BacktestJobDetailResponse {
            job: self.job.ok_or_else(|| BuildError::missing_field("job"))?,
            run: self.run.ok_or_else(|| BuildError::missing_field("run"))?,
        })
    }
}
