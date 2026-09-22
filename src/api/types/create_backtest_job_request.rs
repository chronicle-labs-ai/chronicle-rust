pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateBacktestJobRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cases: Option<Vec<CreateBacktestJobRequestCasesItem>>,
    #[serde(rename = "evaluatorProfileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluator_profile_id: Option<String>,
    #[serde(rename = "nConcurrent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_concurrent: Option<i64>,
    #[serde(default)]
    pub name: String,
    pub recipe: CreateBacktestJobRequestRecipe,
}

impl CreateBacktestJobRequest {
    pub fn builder() -> CreateBacktestJobRequestBuilder {
        <CreateBacktestJobRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBacktestJobRequestBuilder {
    cases: Option<Vec<CreateBacktestJobRequestCasesItem>>,
    evaluator_profile_id: Option<String>,
    n_concurrent: Option<i64>,
    name: Option<String>,
    recipe: Option<CreateBacktestJobRequestRecipe>,
}

impl CreateBacktestJobRequestBuilder {
    pub fn cases(mut self, value: Vec<CreateBacktestJobRequestCasesItem>) -> Self {
        self.cases = Some(value);
        self
    }

    pub fn evaluator_profile_id(mut self, value: impl Into<String>) -> Self {
        self.evaluator_profile_id = Some(value.into());
        self
    }

    pub fn n_concurrent(mut self, value: i64) -> Self {
        self.n_concurrent = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn recipe(mut self, value: CreateBacktestJobRequestRecipe) -> Self {
        self.recipe = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateBacktestJobRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateBacktestJobRequestBuilder::name)
    /// - [`recipe`](CreateBacktestJobRequestBuilder::recipe)
    pub fn build(self) -> Result<CreateBacktestJobRequest, BuildError> {
        Ok(CreateBacktestJobRequest {
            cases: self.cases,
            evaluator_profile_id: self.evaluator_profile_id,
            n_concurrent: self.n_concurrent,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            recipe: self
                .recipe
                .ok_or_else(|| BuildError::missing_field("recipe"))?,
        })
    }
}
