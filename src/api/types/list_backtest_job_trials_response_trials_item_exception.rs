pub use crate::prelude::*;

/// Captured exception info for a failed trial. Mirrors Harbor's `ExceptionInfo` — kind for routing/retry, message for humans.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListBacktestJobTrialsResponseTrialsItemException {
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub message: String,
}

impl ListBacktestJobTrialsResponseTrialsItemException {
    pub fn builder() -> ListBacktestJobTrialsResponseTrialsItemExceptionBuilder {
        <ListBacktestJobTrialsResponseTrialsItemExceptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBacktestJobTrialsResponseTrialsItemExceptionBuilder {
    kind: Option<String>,
    message: Option<String>,
}

impl ListBacktestJobTrialsResponseTrialsItemExceptionBuilder {
    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListBacktestJobTrialsResponseTrialsItemException`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](ListBacktestJobTrialsResponseTrialsItemExceptionBuilder::kind)
    /// - [`message`](ListBacktestJobTrialsResponseTrialsItemExceptionBuilder::message)
    pub fn build(self) -> Result<ListBacktestJobTrialsResponseTrialsItemException, BuildError> {
        Ok(ListBacktestJobTrialsResponseTrialsItemException {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
