pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateClusterRequest {
    #[serde(default)]
    pub color: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "idempotencyKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_cluster_request_idempotency_key: Option<String>,
    #[serde(default)]
    pub label: String,
    #[serde(rename = "similarityCenter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity_center: Option<Vec<f64>>,
}

impl CreateClusterRequest {
    pub fn builder() -> CreateClusterRequestBuilder {
        <CreateClusterRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateClusterRequestBuilder {
    color: Option<String>,
    description: Option<String>,
    create_cluster_request_idempotency_key: Option<String>,
    label: Option<String>,
    similarity_center: Option<Vec<f64>>,
}

impl CreateClusterRequestBuilder {
    pub fn color(mut self, value: impl Into<String>) -> Self {
        self.color = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn create_cluster_request_idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.create_cluster_request_idempotency_key = Some(value.into());
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

    /// Consumes the builder and constructs a [`CreateClusterRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`color`](CreateClusterRequestBuilder::color)
    /// - [`label`](CreateClusterRequestBuilder::label)
    pub fn build(self) -> Result<CreateClusterRequest, BuildError> {
        Ok(CreateClusterRequest {
            color: self
                .color
                .ok_or_else(|| BuildError::missing_field("color"))?,
            description: self.description,
            create_cluster_request_idempotency_key: self.create_cluster_request_idempotency_key,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            similarity_center: self.similarity_center,
        })
    }
}
