pub use crate::prelude::*;

/// Captured exception info for a failed trial. Mirrors Harbor's `ExceptionInfo` — kind for routing/retry, message for humans.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BacktestTrialDetailResponseTrialException {
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub message: String,
}

impl BacktestTrialDetailResponseTrialException {
    pub fn builder() -> BacktestTrialDetailResponseTrialExceptionBuilder {
        <BacktestTrialDetailResponseTrialExceptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BacktestTrialDetailResponseTrialExceptionBuilder {
    kind: Option<String>,
    message: Option<String>,
}

impl BacktestTrialDetailResponseTrialExceptionBuilder {
    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BacktestTrialDetailResponseTrialException`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](BacktestTrialDetailResponseTrialExceptionBuilder::kind)
    /// - [`message`](BacktestTrialDetailResponseTrialExceptionBuilder::message)
    pub fn build(self) -> Result<BacktestTrialDetailResponseTrialException, BuildError> {
        Ok(BacktestTrialDetailResponseTrialException {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
