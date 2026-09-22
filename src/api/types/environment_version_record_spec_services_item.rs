pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EnvironmentVersionRecordSpecServicesItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorities: Option<Vec<String>>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "openapiUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openapi_uri: Option<String>,
}

impl EnvironmentVersionRecordSpecServicesItem {
    pub fn builder() -> EnvironmentVersionRecordSpecServicesItemBuilder {
        <EnvironmentVersionRecordSpecServicesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnvironmentVersionRecordSpecServicesItemBuilder {
    authorities: Option<Vec<String>>,
    name: Option<String>,
    openapi_uri: Option<String>,
}

impl EnvironmentVersionRecordSpecServicesItemBuilder {
    pub fn authorities(mut self, value: Vec<String>) -> Self {
        self.authorities = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn openapi_uri(mut self, value: impl Into<String>) -> Self {
        self.openapi_uri = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EnvironmentVersionRecordSpecServicesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](EnvironmentVersionRecordSpecServicesItemBuilder::name)
    pub fn build(self) -> Result<EnvironmentVersionRecordSpecServicesItem, BuildError> {
        Ok(EnvironmentVersionRecordSpecServicesItem {
            authorities: self.authorities,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            openapi_uri: self.openapi_uri,
        })
    }
}
