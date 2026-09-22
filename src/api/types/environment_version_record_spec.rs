pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EnvironmentVersionRecordSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datastores: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub interception: EnvironmentVersionRecordSpecInterception,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp: Option<Vec<EnvironmentVersionRecordSpecMcpItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<EnvironmentVersionRecordSpecServicesItem>>,
    /// Twins this environment wants running (`backend/twins` models), seeded from the environment's dataset when spun up.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twins: Option<Vec<EnvironmentVersionRecordSpecTwinsItem>>,
}

impl EnvironmentVersionRecordSpec {
    pub fn builder() -> EnvironmentVersionRecordSpecBuilder {
        <EnvironmentVersionRecordSpecBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnvironmentVersionRecordSpecBuilder {
    datastores: Option<Vec<serde_json::Value>>,
    interception: Option<EnvironmentVersionRecordSpecInterception>,
    mcp: Option<Vec<EnvironmentVersionRecordSpecMcpItem>>,
    services: Option<Vec<EnvironmentVersionRecordSpecServicesItem>>,
    twins: Option<Vec<EnvironmentVersionRecordSpecTwinsItem>>,
}

impl EnvironmentVersionRecordSpecBuilder {
    pub fn datastores(mut self, value: Vec<serde_json::Value>) -> Self {
        self.datastores = Some(value);
        self
    }

    pub fn interception(mut self, value: EnvironmentVersionRecordSpecInterception) -> Self {
        self.interception = Some(value);
        self
    }

    pub fn mcp(mut self, value: Vec<EnvironmentVersionRecordSpecMcpItem>) -> Self {
        self.mcp = Some(value);
        self
    }

    pub fn services(mut self, value: Vec<EnvironmentVersionRecordSpecServicesItem>) -> Self {
        self.services = Some(value);
        self
    }

    pub fn twins(mut self, value: Vec<EnvironmentVersionRecordSpecTwinsItem>) -> Self {
        self.twins = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EnvironmentVersionRecordSpec`].
    /// This method will fail if any of the following fields are not set:
    /// - [`interception`](EnvironmentVersionRecordSpecBuilder::interception)
    pub fn build(self) -> Result<EnvironmentVersionRecordSpec, BuildError> {
        Ok(EnvironmentVersionRecordSpec {
            datastores: self.datastores,
            interception: self
                .interception
                .ok_or_else(|| BuildError::missing_field("interception"))?,
            mcp: self.mcp,
            services: self.services,
            twins: self.twins,
        })
    }
}
