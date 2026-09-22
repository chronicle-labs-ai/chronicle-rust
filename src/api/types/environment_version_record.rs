pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnvironmentVersionRecord {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "environmentId")]
    #[serde(default)]
    pub environment_id: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub spec: EnvironmentVersionRecordSpec,
    pub status: EnvironmentVersionRecordStatus,
    #[serde(rename = "tenantId")]
    #[serde(default)]
    pub tenant_id: String,
    #[serde(default)]
    pub version: String,
}

impl EnvironmentVersionRecord {
    pub fn builder() -> EnvironmentVersionRecordBuilder {
        <EnvironmentVersionRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnvironmentVersionRecordBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    environment_id: Option<String>,
    id: Option<String>,
    spec: Option<EnvironmentVersionRecordSpec>,
    status: Option<EnvironmentVersionRecordStatus>,
    tenant_id: Option<String>,
    version: Option<String>,
}

impl EnvironmentVersionRecordBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn environment_id(mut self, value: impl Into<String>) -> Self {
        self.environment_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn spec(mut self, value: EnvironmentVersionRecordSpec) -> Self {
        self.spec = Some(value);
        self
    }

    pub fn status(mut self, value: EnvironmentVersionRecordStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tenant_id(mut self, value: impl Into<String>) -> Self {
        self.tenant_id = Some(value.into());
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EnvironmentVersionRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](EnvironmentVersionRecordBuilder::created_at)
    /// - [`environment_id`](EnvironmentVersionRecordBuilder::environment_id)
    /// - [`id`](EnvironmentVersionRecordBuilder::id)
    /// - [`spec`](EnvironmentVersionRecordBuilder::spec)
    /// - [`status`](EnvironmentVersionRecordBuilder::status)
    /// - [`tenant_id`](EnvironmentVersionRecordBuilder::tenant_id)
    /// - [`version`](EnvironmentVersionRecordBuilder::version)
    pub fn build(self) -> Result<EnvironmentVersionRecord, BuildError> {
        Ok(EnvironmentVersionRecord {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            environment_id: self
                .environment_id
                .ok_or_else(|| BuildError::missing_field("environment_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            spec: self.spec.ok_or_else(|| BuildError::missing_field("spec"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            tenant_id: self
                .tenant_id
                .ok_or_else(|| BuildError::missing_field("tenant_id"))?,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
