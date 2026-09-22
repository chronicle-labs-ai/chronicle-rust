pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateBacktestJobRequestRecipe {
    /// 1..N agents under test. The first is treated as the comparison baseline by `BacktestResults`.
    #[serde(default)]
    pub agents: Vec<CreateBacktestJobRequestRecipeAgentsItem>,
    pub data: CreateBacktestJobRequestRecipeData,
    /// Optional environment the run targets. Pipeline step 03 sets this; consumers without an environment fall back to the default ephemeral sandbox.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<CreateBacktestJobRequestRecipeEnvironment>,
    #[serde(default)]
    pub graders: Vec<CreateBacktestJobRequestRecipeGradersItem>,
    pub mode: CreateBacktestJobRequestRecipeMode,
    /// Free-text run name shown in the recipe header + top nav.
    #[serde(default)]
    pub name: String,
    /// Optional pinned trace seed; used by the Replay preset to reproduce a single trace as the focal point of the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<String>,
}

impl CreateBacktestJobRequestRecipe {
    pub fn builder() -> CreateBacktestJobRequestRecipeBuilder {
        <CreateBacktestJobRequestRecipeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBacktestJobRequestRecipeBuilder {
    agents: Option<Vec<CreateBacktestJobRequestRecipeAgentsItem>>,
    data: Option<CreateBacktestJobRequestRecipeData>,
    environment: Option<CreateBacktestJobRequestRecipeEnvironment>,
    graders: Option<Vec<CreateBacktestJobRequestRecipeGradersItem>>,
    mode: Option<CreateBacktestJobRequestRecipeMode>,
    name: Option<String>,
    seed: Option<String>,
}

impl CreateBacktestJobRequestRecipeBuilder {
    pub fn agents(mut self, value: Vec<CreateBacktestJobRequestRecipeAgentsItem>) -> Self {
        self.agents = Some(value);
        self
    }

    pub fn data(mut self, value: CreateBacktestJobRequestRecipeData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn environment(mut self, value: CreateBacktestJobRequestRecipeEnvironment) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn graders(mut self, value: Vec<CreateBacktestJobRequestRecipeGradersItem>) -> Self {
        self.graders = Some(value);
        self
    }

    pub fn mode(mut self, value: CreateBacktestJobRequestRecipeMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn seed(mut self, value: impl Into<String>) -> Self {
        self.seed = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateBacktestJobRequestRecipe`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agents`](CreateBacktestJobRequestRecipeBuilder::agents)
    /// - [`data`](CreateBacktestJobRequestRecipeBuilder::data)
    /// - [`graders`](CreateBacktestJobRequestRecipeBuilder::graders)
    /// - [`mode`](CreateBacktestJobRequestRecipeBuilder::mode)
    /// - [`name`](CreateBacktestJobRequestRecipeBuilder::name)
    pub fn build(self) -> Result<CreateBacktestJobRequestRecipe, BuildError> {
        Ok(CreateBacktestJobRequestRecipe {
            agents: self
                .agents
                .ok_or_else(|| BuildError::missing_field("agents"))?,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            environment: self.environment,
            graders: self
                .graders
                .ok_or_else(|| BuildError::missing_field("graders"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            seed: self.seed,
        })
    }
}
