pub use crate::prelude::*;

/// Code handler snapshot — required when `kind` is `code`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BacktestTrialDetailResponseScorersItemGraderCode {
    /// Language of a code scorer. Handlers run inside the trial's trusted verifier sandbox: Python via `python3`, TypeScript via `node` (>= 22.6, type-stripping) — both shipped in the sandbox runtime image.
    pub language: BacktestTrialDetailResponseScorersItemGraderCodeLanguage,
    #[serde(default)]
    pub source: String,
}

impl BacktestTrialDetailResponseScorersItemGraderCode {
    pub fn builder() -> BacktestTrialDetailResponseScorersItemGraderCodeBuilder {
        <BacktestTrialDetailResponseScorersItemGraderCodeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BacktestTrialDetailResponseScorersItemGraderCodeBuilder {
    language: Option<BacktestTrialDetailResponseScorersItemGraderCodeLanguage>,
    source: Option<String>,
}

impl BacktestTrialDetailResponseScorersItemGraderCodeBuilder {
    pub fn language(
        mut self,
        value: BacktestTrialDetailResponseScorersItemGraderCodeLanguage,
    ) -> Self {
        self.language = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BacktestTrialDetailResponseScorersItemGraderCode`].
    /// This method will fail if any of the following fields are not set:
    /// - [`language`](BacktestTrialDetailResponseScorersItemGraderCodeBuilder::language)
    /// - [`source`](BacktestTrialDetailResponseScorersItemGraderCodeBuilder::source)
    pub fn build(self) -> Result<BacktestTrialDetailResponseScorersItemGraderCode, BuildError> {
        Ok(BacktestTrialDetailResponseScorersItemGraderCode {
            language: self
                .language
                .ok_or_else(|| BuildError::missing_field("language"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
