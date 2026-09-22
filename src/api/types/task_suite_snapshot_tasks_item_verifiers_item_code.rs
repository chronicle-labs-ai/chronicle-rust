pub use crate::prelude::*;

/// Code handler snapshot — required when `kind` is `code`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaskSuiteSnapshotTasksItemVerifiersItemCode {
    /// Language of a code scorer. Handlers run inside the trial's trusted verifier sandbox: Python via `python3`, TypeScript via `node` (>= 22.6, type-stripping) — both shipped in the sandbox runtime image.
    pub language: TaskSuiteSnapshotTasksItemVerifiersItemCodeLanguage,
    #[serde(default)]
    pub source: String,
}

impl TaskSuiteSnapshotTasksItemVerifiersItemCode {
    pub fn builder() -> TaskSuiteSnapshotTasksItemVerifiersItemCodeBuilder {
        <TaskSuiteSnapshotTasksItemVerifiersItemCodeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskSuiteSnapshotTasksItemVerifiersItemCodeBuilder {
    language: Option<TaskSuiteSnapshotTasksItemVerifiersItemCodeLanguage>,
    source: Option<String>,
}

impl TaskSuiteSnapshotTasksItemVerifiersItemCodeBuilder {
    pub fn language(mut self, value: TaskSuiteSnapshotTasksItemVerifiersItemCodeLanguage) -> Self {
        self.language = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskSuiteSnapshotTasksItemVerifiersItemCode`].
    /// This method will fail if any of the following fields are not set:
    /// - [`language`](TaskSuiteSnapshotTasksItemVerifiersItemCodeBuilder::language)
    /// - [`source`](TaskSuiteSnapshotTasksItemVerifiersItemCodeBuilder::source)
    pub fn build(self) -> Result<TaskSuiteSnapshotTasksItemVerifiersItemCode, BuildError> {
        Ok(TaskSuiteSnapshotTasksItemVerifiersItemCode {
            language: self
                .language
                .ok_or_else(|| BuildError::missing_field("language"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
