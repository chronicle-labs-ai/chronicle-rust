pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateEnvironmentVersionRequest {
    #[serde(default)]
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<EnvironmentSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<EnvironmentVersionStatus>,
}

impl CreateEnvironmentVersionRequest {
    pub fn builder() -> CreateEnvironmentVersionRequestBuilder {
        <CreateEnvironmentVersionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateEnvironmentVersionRequestBuilder {
    version: Option<String>,
    spec: Option<EnvironmentSpec>,
    status: Option<EnvironmentVersionStatus>,
}

impl CreateEnvironmentVersionRequestBuilder {
    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn spec(mut self, value: EnvironmentSpec) -> Self {
        self.spec = Some(value);
        self
    }

    pub fn status(mut self, value: EnvironmentVersionStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateEnvironmentVersionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](CreateEnvironmentVersionRequestBuilder::version)
    pub fn build(self) -> Result<CreateEnvironmentVersionRequest, BuildError> {
        Ok(CreateEnvironmentVersionRequest {
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
            spec: self.spec,
            status: self.status,
        })
    }
}
