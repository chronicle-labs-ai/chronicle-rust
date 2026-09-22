pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListEnvironmentsResponseEnvironmentsItem {
    #[serde(default)]
    pub environment: EnvironmentRecord,
    #[serde(default)]
    pub versions: Vec<EnvironmentVersionRecord>,
}

impl ListEnvironmentsResponseEnvironmentsItem {
    pub fn builder() -> ListEnvironmentsResponseEnvironmentsItemBuilder {
        <ListEnvironmentsResponseEnvironmentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEnvironmentsResponseEnvironmentsItemBuilder {
    environment: Option<EnvironmentRecord>,
    versions: Option<Vec<EnvironmentVersionRecord>>,
}

impl ListEnvironmentsResponseEnvironmentsItemBuilder {
    pub fn environment(mut self, value: EnvironmentRecord) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn versions(mut self, value: Vec<EnvironmentVersionRecord>) -> Self {
        self.versions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEnvironmentsResponseEnvironmentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environment`](ListEnvironmentsResponseEnvironmentsItemBuilder::environment)
    /// - [`versions`](ListEnvironmentsResponseEnvironmentsItemBuilder::versions)
    pub fn build(self) -> Result<ListEnvironmentsResponseEnvironmentsItem, BuildError> {
        Ok(ListEnvironmentsResponseEnvironmentsItem {
            environment: self
                .environment
                .ok_or_else(|| BuildError::missing_field("environment"))?,
            versions: self
                .versions
                .ok_or_else(|| BuildError::missing_field("versions"))?,
        })
    }
}
