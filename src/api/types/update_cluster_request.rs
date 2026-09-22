pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateClusterRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "similarityCenter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity_center: Option<Vec<f64>>,
}

impl UpdateClusterRequest {
    pub fn builder() -> UpdateClusterRequestBuilder {
        <UpdateClusterRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateClusterRequestBuilder {
    color: Option<String>,
    description: Option<String>,
    label: Option<String>,
    similarity_center: Option<Vec<f64>>,
}

impl UpdateClusterRequestBuilder {
    pub fn color(mut self, value: impl Into<String>) -> Self {
        self.color = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn similarity_center(mut self, value: Vec<f64>) -> Self {
        self.similarity_center = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateClusterRequest`].
    pub fn build(self) -> Result<UpdateClusterRequest, BuildError> {
        Ok(UpdateClusterRequest {
            color: self.color,
            description: self.description,
            label: self.label,
            similarity_center: self.similarity_center,
        })
    }
}
