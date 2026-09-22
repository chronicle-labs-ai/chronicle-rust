pub use crate::prelude::*;

/// Code handler snapshot — required when `kind` is `code`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemCode {
    /// Language of a code scorer. Handlers run inside the trial's trusted verifier sandbox: Python via `python3`, TypeScript via `node` (>= 22.6, type-stripping) — both shipped in the sandbox runtime image.
    pub language: BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemCodeLanguage,
    #[serde(default)]
    pub source: String,
}

impl BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemCode {
    pub fn builder() -> BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemCodeBuilder
    {
        <BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemCodeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemCodeBuilder {
    language: Option<BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemCodeLanguage>,
    source: Option<String>,
}

impl BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemCodeBuilder {
    pub fn language(
        mut self,
        value: BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemCodeLanguage,
    ) -> Self {
        self.language = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemCode`].
    /// This method will fail if any of the following fields are not set:
    /// - [`language`](BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemCodeBuilder::language)
    /// - [`source`](BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemCodeBuilder::source)
    pub fn build(
        self,
    ) -> Result<BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemCode, BuildError>
    {
        Ok(
            BacktestsAvailabilityDatasetSnapshotsValueTasksItemVerifiersItemCode {
                language: self
                    .language
                    .ok_or_else(|| BuildError::missing_field("language"))?,
                source: self
                    .source
                    .ok_or_else(|| BuildError::missing_field("source"))?,
            },
        )
    }
}
