pub use crate::prelude::*;

/// Code handler snapshot — required when `kind` is `code`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateBacktestJobRequestRecipeGradersItemCode {
    /// Language of a code scorer. Handlers run inside the trial's trusted verifier sandbox: Python via `python3`, TypeScript via `node` (>= 22.6, type-stripping) — both shipped in the sandbox runtime image.
    pub language: CreateBacktestJobRequestRecipeGradersItemCodeLanguage,
    #[serde(default)]
    pub source: String,
}

impl CreateBacktestJobRequestRecipeGradersItemCode {
    pub fn builder() -> CreateBacktestJobRequestRecipeGradersItemCodeBuilder {
        <CreateBacktestJobRequestRecipeGradersItemCodeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBacktestJobRequestRecipeGradersItemCodeBuilder {
    language: Option<CreateBacktestJobRequestRecipeGradersItemCodeLanguage>,
    source: Option<String>,
}

impl CreateBacktestJobRequestRecipeGradersItemCodeBuilder {
    pub fn language(
        mut self,
        value: CreateBacktestJobRequestRecipeGradersItemCodeLanguage,
    ) -> Self {
        self.language = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateBacktestJobRequestRecipeGradersItemCode`].
    /// This method will fail if any of the following fields are not set:
    /// - [`language`](CreateBacktestJobRequestRecipeGradersItemCodeBuilder::language)
    /// - [`source`](CreateBacktestJobRequestRecipeGradersItemCodeBuilder::source)
    pub fn build(self) -> Result<CreateBacktestJobRequestRecipeGradersItemCode, BuildError> {
        Ok(CreateBacktestJobRequestRecipeGradersItemCode {
            language: self
                .language
                .ok_or_else(|| BuildError::missing_field("language"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
