pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateBacktestJobRequestRecipeData {
    /// Populated only when `kind == "dataset"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<String>,
    #[serde(rename = "datasetLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_label: Option<String>,
    pub kind: CreateBacktestJobRequestRecipeDataKind,
    /// Optional name the user wants to save this composed dataset as. `null` (vs absent) tells the UI the user opted out of saving.
    #[serde(rename = "savedAs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved_as: Option<String>,
    #[serde(default)]
    pub scenarios: Vec<CreateBacktestJobRequestRecipeDataScenariosItem>,
    #[serde(default)]
    pub sources: Vec<CreateBacktestJobRequestRecipeDataSourcesItem>,
}

impl CreateBacktestJobRequestRecipeData {
    pub fn builder() -> CreateBacktestJobRequestRecipeDataBuilder {
        <CreateBacktestJobRequestRecipeDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBacktestJobRequestRecipeDataBuilder {
    dataset: Option<String>,
    dataset_label: Option<String>,
    kind: Option<CreateBacktestJobRequestRecipeDataKind>,
    saved_as: Option<String>,
    scenarios: Option<Vec<CreateBacktestJobRequestRecipeDataScenariosItem>>,
    sources: Option<Vec<CreateBacktestJobRequestRecipeDataSourcesItem>>,
}

impl CreateBacktestJobRequestRecipeDataBuilder {
    pub fn dataset(mut self, value: impl Into<String>) -> Self {
        self.dataset = Some(value.into());
        self
    }

    pub fn dataset_label(mut self, value: impl Into<String>) -> Self {
        self.dataset_label = Some(value.into());
        self
    }

    pub fn kind(mut self, value: CreateBacktestJobRequestRecipeDataKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn saved_as(mut self, value: impl Into<String>) -> Self {
        self.saved_as = Some(value.into());
        self
    }

    pub fn scenarios(
        mut self,
        value: Vec<CreateBacktestJobRequestRecipeDataScenariosItem>,
    ) -> Self {
        self.scenarios = Some(value);
        self
    }

    pub fn sources(mut self, value: Vec<CreateBacktestJobRequestRecipeDataSourcesItem>) -> Self {
        self.sources = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateBacktestJobRequestRecipeData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](CreateBacktestJobRequestRecipeDataBuilder::kind)
    /// - [`scenarios`](CreateBacktestJobRequestRecipeDataBuilder::scenarios)
    /// - [`sources`](CreateBacktestJobRequestRecipeDataBuilder::sources)
    pub fn build(self) -> Result<CreateBacktestJobRequestRecipeData, BuildError> {
        Ok(CreateBacktestJobRequestRecipeData {
            dataset: self.dataset,
            dataset_label: self.dataset_label,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            saved_as: self.saved_as,
            scenarios: self
                .scenarios
                .ok_or_else(|| BuildError::missing_field("scenarios"))?,
            sources: self
                .sources
                .ok_or_else(|| BuildError::missing_field("sources"))?,
        })
    }
}
