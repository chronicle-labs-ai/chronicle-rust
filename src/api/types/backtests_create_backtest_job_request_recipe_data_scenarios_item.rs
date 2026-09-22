pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateBacktestJobRequestRecipeDataScenariosItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted: Option<bool>,
    /// Cluster bucket emitted by the data-science layer when looking for missing scenarios in a dataset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<CreateBacktestJobRequestRecipeDataScenariosItemBucket>,
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "clusterLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub id: String,
    pub kind: CreateBacktestJobRequestRecipeDataScenariosItemKind,
    #[serde(default)]
    pub label: String,
}

impl CreateBacktestJobRequestRecipeDataScenariosItem {
    pub fn builder() -> CreateBacktestJobRequestRecipeDataScenariosItemBuilder {
        <CreateBacktestJobRequestRecipeDataScenariosItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBacktestJobRequestRecipeDataScenariosItemBuilder {
    accepted: Option<bool>,
    bucket: Option<CreateBacktestJobRequestRecipeDataScenariosItemBucket>,
    cluster_id: Option<String>,
    cluster_label: Option<String>,
    confidence: Option<f64>,
    count: Option<i64>,
    id: Option<String>,
    kind: Option<CreateBacktestJobRequestRecipeDataScenariosItemKind>,
    label: Option<String>,
}

impl CreateBacktestJobRequestRecipeDataScenariosItemBuilder {
    pub fn accepted(mut self, value: bool) -> Self {
        self.accepted = Some(value);
        self
    }

    pub fn bucket(mut self, value: CreateBacktestJobRequestRecipeDataScenariosItemBucket) -> Self {
        self.bucket = Some(value);
        self
    }

    pub fn cluster_id(mut self, value: impl Into<String>) -> Self {
        self.cluster_id = Some(value.into());
        self
    }

    pub fn cluster_label(mut self, value: impl Into<String>) -> Self {
        self.cluster_label = Some(value.into());
        self
    }

    pub fn confidence(mut self, value: f64) -> Self {
        self.confidence = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: CreateBacktestJobRequestRecipeDataScenariosItemKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateBacktestJobRequestRecipeDataScenariosItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](CreateBacktestJobRequestRecipeDataScenariosItemBuilder::count)
    /// - [`id`](CreateBacktestJobRequestRecipeDataScenariosItemBuilder::id)
    /// - [`kind`](CreateBacktestJobRequestRecipeDataScenariosItemBuilder::kind)
    /// - [`label`](CreateBacktestJobRequestRecipeDataScenariosItemBuilder::label)
    pub fn build(self) -> Result<CreateBacktestJobRequestRecipeDataScenariosItem, BuildError> {
        Ok(CreateBacktestJobRequestRecipeDataScenariosItem {
            accepted: self.accepted,
            bucket: self.bucket,
            cluster_id: self.cluster_id,
            cluster_label: self.cluster_label,
            confidence: self.confidence,
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
        })
    }
}
