pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateBacktestJobRequestRecipeDataSourcesItemFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<String>,
}

impl CreateBacktestJobRequestRecipeDataSourcesItemFilters {
    pub fn builder() -> CreateBacktestJobRequestRecipeDataSourcesItemFiltersBuilder {
        <CreateBacktestJobRequestRecipeDataSourcesItemFiltersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBacktestJobRequestRecipeDataSourcesItemFiltersBuilder {
    clusters: Option<Vec<String>>,
    outcome: Option<String>,
    seed: Option<String>,
    window: Option<String>,
}

impl CreateBacktestJobRequestRecipeDataSourcesItemFiltersBuilder {
    pub fn clusters(mut self, value: Vec<String>) -> Self {
        self.clusters = Some(value);
        self
    }

    pub fn outcome(mut self, value: impl Into<String>) -> Self {
        self.outcome = Some(value.into());
        self
    }

    pub fn seed(mut self, value: impl Into<String>) -> Self {
        self.seed = Some(value.into());
        self
    }

    pub fn window(mut self, value: impl Into<String>) -> Self {
        self.window = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateBacktestJobRequestRecipeDataSourcesItemFilters`].
    pub fn build(self) -> Result<CreateBacktestJobRequestRecipeDataSourcesItemFilters, BuildError> {
        Ok(CreateBacktestJobRequestRecipeDataSourcesItemFilters {
            clusters: self.clusters,
            outcome: self.outcome,
            seed: self.seed,
            window: self.window,
        })
    }
}
