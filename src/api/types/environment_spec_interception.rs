pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EnvironmentSpecInterception {
    #[serde(rename = "installCa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_ca: Option<bool>,
    #[serde(rename = "regularProxyPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regular_proxy_port: Option<i64>,
    #[serde(rename = "transparentProxyPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transparent_proxy_port: Option<i64>,
}

impl EnvironmentSpecInterception {
    pub fn builder() -> EnvironmentSpecInterceptionBuilder {
        <EnvironmentSpecInterceptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnvironmentSpecInterceptionBuilder {
    install_ca: Option<bool>,
    regular_proxy_port: Option<i64>,
    transparent_proxy_port: Option<i64>,
}

impl EnvironmentSpecInterceptionBuilder {
    pub fn install_ca(mut self, value: bool) -> Self {
        self.install_ca = Some(value);
        self
    }

    pub fn regular_proxy_port(mut self, value: i64) -> Self {
        self.regular_proxy_port = Some(value);
        self
    }

    pub fn transparent_proxy_port(mut self, value: i64) -> Self {
        self.transparent_proxy_port = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EnvironmentSpecInterception`].
    pub fn build(self) -> Result<EnvironmentSpecInterception, BuildError> {
        Ok(EnvironmentSpecInterception {
            install_ca: self.install_ca,
            regular_proxy_port: self.regular_proxy_port,
            transparent_proxy_port: self.transparent_proxy_port,
        })
    }
}
