pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnvironmentVersionResponse {
    #[serde(default)]
    pub environment: EnvironmentRecord,
    pub version: EnvironmentVersionRecord,
}

impl EnvironmentVersionResponse {
    pub fn builder() -> EnvironmentVersionResponseBuilder {
        <EnvironmentVersionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnvironmentVersionResponseBuilder {
    environment: Option<EnvironmentRecord>,
    version: Option<EnvironmentVersionRecord>,
}

impl EnvironmentVersionResponseBuilder {
    pub fn environment(mut self, value: EnvironmentRecord) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn version(mut self, value: EnvironmentVersionRecord) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EnvironmentVersionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environment`](EnvironmentVersionResponseBuilder::environment)
    /// - [`version`](EnvironmentVersionResponseBuilder::version)
    pub fn build(self) -> Result<EnvironmentVersionResponse, BuildError> {
        Ok(EnvironmentVersionResponse {
            environment: self
                .environment
                .ok_or_else(|| BuildError::missing_field("environment"))?,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
