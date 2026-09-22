pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EnvironmentResponse {
    #[serde(default)]
    pub environment: EnvironmentRecord,
    #[serde(default)]
    pub versions: Vec<EnvironmentVersionRecord>,
}

impl EnvironmentResponse {
    pub fn builder() -> EnvironmentResponseBuilder {
        <EnvironmentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnvironmentResponseBuilder {
    environment: Option<EnvironmentRecord>,
    versions: Option<Vec<EnvironmentVersionRecord>>,
}

impl EnvironmentResponseBuilder {
    pub fn environment(mut self, value: EnvironmentRecord) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn versions(mut self, value: Vec<EnvironmentVersionRecord>) -> Self {
        self.versions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EnvironmentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environment`](EnvironmentResponseBuilder::environment)
    /// - [`versions`](EnvironmentResponseBuilder::versions)
    pub fn build(self) -> Result<EnvironmentResponse, BuildError> {
        Ok(EnvironmentResponse {
            environment: self
                .environment
                .ok_or_else(|| BuildError::missing_field("environment"))?,
            versions: self
                .versions
                .ok_or_else(|| BuildError::missing_field("versions"))?,
        })
    }
}
