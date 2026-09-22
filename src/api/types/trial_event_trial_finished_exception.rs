pub use crate::prelude::*;

/// Captured exception info for a failed trial. Mirrors Harbor's `ExceptionInfo` — kind for routing/retry, message for humans.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TrialEventTrialFinishedException {
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub message: String,
}

impl TrialEventTrialFinishedException {
    pub fn builder() -> TrialEventTrialFinishedExceptionBuilder {
        <TrialEventTrialFinishedExceptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TrialEventTrialFinishedExceptionBuilder {
    kind: Option<String>,
    message: Option<String>,
}

impl TrialEventTrialFinishedExceptionBuilder {
    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TrialEventTrialFinishedException`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TrialEventTrialFinishedExceptionBuilder::kind)
    /// - [`message`](TrialEventTrialFinishedExceptionBuilder::message)
    pub fn build(self) -> Result<TrialEventTrialFinishedException, BuildError> {
        Ok(TrialEventTrialFinishedException {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
