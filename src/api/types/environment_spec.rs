pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EnvironmentSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datastores: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub interception: EnvironmentSpecInterception,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp: Option<Vec<EnvironmentSpecMcpItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<EnvironmentSpecServicesItem>>,
    /// Twins this environment wants running (`backend/twins` models), seeded from the environment's dataset when spun up.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twins: Option<Vec<EnvironmentSpecTwinsItem>>,
}

impl EnvironmentSpec {
    pub fn builder() -> EnvironmentSpecBuilder {
        <EnvironmentSpecBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnvironmentSpecBuilder {
    datastores: Option<Vec<serde_json::Value>>,
    interception: Option<EnvironmentSpecInterception>,
    mcp: Option<Vec<EnvironmentSpecMcpItem>>,
    services: Option<Vec<EnvironmentSpecServicesItem>>,
    twins: Option<Vec<EnvironmentSpecTwinsItem>>,
}

impl EnvironmentSpecBuilder {
    pub fn datastores(mut self, value: Vec<serde_json::Value>) -> Self {
        self.datastores = Some(value);
        self
    }

    pub fn interception(mut self, value: EnvironmentSpecInterception) -> Self {
        self.interception = Some(value);
        self
    }

    pub fn mcp(mut self, value: Vec<EnvironmentSpecMcpItem>) -> Self {
        self.mcp = Some(value);
        self
    }

    pub fn services(mut self, value: Vec<EnvironmentSpecServicesItem>) -> Self {
        self.services = Some(value);
        self
    }

    pub fn twins(mut self, value: Vec<EnvironmentSpecTwinsItem>) -> Self {
        self.twins = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EnvironmentSpec`].
    /// This method will fail if any of the following fields are not set:
    /// - [`interception`](EnvironmentSpecBuilder::interception)
    pub fn build(self) -> Result<EnvironmentSpec, BuildError> {
        Ok(EnvironmentSpec {
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
