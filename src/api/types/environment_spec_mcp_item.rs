pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EnvironmentSpecMcpItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl EnvironmentSpecMcpItem {
    pub fn builder() -> EnvironmentSpecMcpItemBuilder {
        <EnvironmentSpecMcpItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnvironmentSpecMcpItemBuilder {
    command: Option<String>,
    name: Option<String>,
    transport: Option<String>,
    url: Option<String>,
}

impl EnvironmentSpecMcpItemBuilder {
    pub fn command(mut self, value: impl Into<String>) -> Self {
        self.command = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn transport(mut self, value: impl Into<String>) -> Self {
        self.transport = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EnvironmentSpecMcpItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](EnvironmentSpecMcpItemBuilder::name)
    pub fn build(self) -> Result<EnvironmentSpecMcpItem, BuildError> {
        Ok(EnvironmentSpecMcpItem {
            command: self.command,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            transport: self.transport,
            url: self.url,
        })
    }
}
