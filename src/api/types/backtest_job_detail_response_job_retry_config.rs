pub use crate::prelude::*;

/// Retry policy applied to transient trial failures (network errors, sandbox-create timeouts). Mirrors Harbor's `RetryConfig`. Pass `null` on the wire (`None` here) to fall back to defaults.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BacktestJobDetailResponseJobRetryConfig {
    /// Exception kinds that explicitly should not be retried even if otherwise matched by `include_exceptions`.
    #[serde(rename = "excludeExceptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_exceptions: Option<Vec<String>>,
    /// When set, only exception kinds in this list trigger a retry. `None` means "retry every transient kind".
    #[serde(rename = "includeExceptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_exceptions: Option<Vec<String>>,
    #[serde(rename = "maxRetries")]
    #[serde(default)]
    pub max_retries: i64,
    #[serde(rename = "maxWaitSec")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub max_wait_sec: f64,
    #[serde(rename = "minWaitSec")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub min_wait_sec: f64,
    #[serde(rename = "waitMultiplier")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub wait_multiplier: f64,
}

impl BacktestJobDetailResponseJobRetryConfig {
    pub fn builder() -> BacktestJobDetailResponseJobRetryConfigBuilder {
        <BacktestJobDetailResponseJobRetryConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BacktestJobDetailResponseJobRetryConfigBuilder {
    exclude_exceptions: Option<Vec<String>>,
    include_exceptions: Option<Vec<String>>,
    max_retries: Option<i64>,
    max_wait_sec: Option<f64>,
    min_wait_sec: Option<f64>,
    wait_multiplier: Option<f64>,
}

impl BacktestJobDetailResponseJobRetryConfigBuilder {
    pub fn exclude_exceptions(mut self, value: Vec<String>) -> Self {
        self.exclude_exceptions = Some(value);
        self
    }

    pub fn include_exceptions(mut self, value: Vec<String>) -> Self {
        self.include_exceptions = Some(value);
        self
    }

    pub fn max_retries(mut self, value: i64) -> Self {
        self.max_retries = Some(value);
        self
    }

    pub fn max_wait_sec(mut self, value: f64) -> Self {
        self.max_wait_sec = Some(value);
        self
    }

    pub fn min_wait_sec(mut self, value: f64) -> Self {
        self.min_wait_sec = Some(value);
        self
    }

    pub fn wait_multiplier(mut self, value: f64) -> Self {
        self.wait_multiplier = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BacktestJobDetailResponseJobRetryConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`max_retries`](BacktestJobDetailResponseJobRetryConfigBuilder::max_retries)
    /// - [`max_wait_sec`](BacktestJobDetailResponseJobRetryConfigBuilder::max_wait_sec)
    /// - [`min_wait_sec`](BacktestJobDetailResponseJobRetryConfigBuilder::min_wait_sec)
    /// - [`wait_multiplier`](BacktestJobDetailResponseJobRetryConfigBuilder::wait_multiplier)
    pub fn build(self) -> Result<BacktestJobDetailResponseJobRetryConfig, BuildError> {
        Ok(BacktestJobDetailResponseJobRetryConfig {
            exclude_exceptions: self.exclude_exceptions,
            include_exceptions: self.include_exceptions,
            max_retries: self
                .max_retries
                .ok_or_else(|| BuildError::missing_field("max_retries"))?,
            max_wait_sec: self
                .max_wait_sec
                .ok_or_else(|| BuildError::missing_field("max_wait_sec"))?,
            min_wait_sec: self
                .min_wait_sec
                .ok_or_else(|| BuildError::missing_field("min_wait_sec"))?,
            wait_multiplier: self
                .wait_multiplier
                .ok_or_else(|| BuildError::missing_field("wait_multiplier"))?,
        })
    }
}
