pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CancelBacktestJobResponse {
    /// True only when an active execution accepted cancellation.
    #[serde(default)]
    pub aborted: bool,
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
    /// Lifecycle state of an entire `BacktestJob`. Maps 1:1 onto the `status` column in `migrations/013_create_backtest_runtime.sql`.
    #[serde(rename = "previousStatus")]
    pub previous_status: CancelBacktestJobResponsePreviousStatus,
}

impl CancelBacktestJobResponse {
    pub fn builder() -> CancelBacktestJobResponseBuilder {
        <CancelBacktestJobResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelBacktestJobResponseBuilder {
    aborted: Option<bool>,
    job_id: Option<String>,
    previous_status: Option<CancelBacktestJobResponsePreviousStatus>,
}

impl CancelBacktestJobResponseBuilder {
    pub fn aborted(mut self, value: bool) -> Self {
        self.aborted = Some(value);
        self
    }

    pub fn job_id(mut self, value: impl Into<String>) -> Self {
        self.job_id = Some(value.into());
        self
    }

    pub fn previous_status(mut self, value: CancelBacktestJobResponsePreviousStatus) -> Self {
        self.previous_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CancelBacktestJobResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`aborted`](CancelBacktestJobResponseBuilder::aborted)
    /// - [`job_id`](CancelBacktestJobResponseBuilder::job_id)
    /// - [`previous_status`](CancelBacktestJobResponseBuilder::previous_status)
    pub fn build(self) -> Result<CancelBacktestJobResponse, BuildError> {
        Ok(CancelBacktestJobResponse {
            aborted: self
                .aborted
                .ok_or_else(|| BuildError::missing_field("aborted"))?,
            job_id: self
                .job_id
                .ok_or_else(|| BuildError::missing_field("job_id"))?,
            previous_status: self
                .previous_status
                .ok_or_else(|| BuildError::missing_field("previous_status"))?,
        })
    }
}
