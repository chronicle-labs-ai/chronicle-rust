pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SdkKey {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "lastUsedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub prefix: String,
    #[serde(rename = "revokedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub scopes: Vec<String>,
    #[serde(rename = "tenantId")]
    #[serde(default)]
    pub tenant_id: String,
}

impl SdkKey {
    pub fn builder() -> SdkKeyBuilder {
        <SdkKeyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SdkKeyBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    last_used_at: Option<DateTime<FixedOffset>>,
    name: Option<String>,
    prefix: Option<String>,
    revoked_at: Option<DateTime<FixedOffset>>,
    scopes: Option<Vec<String>>,
    tenant_id: Option<String>,
}

impl SdkKeyBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_used_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_used_at = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn prefix(mut self, value: impl Into<String>) -> Self {
        self.prefix = Some(value.into());
        self
    }

    pub fn revoked_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.revoked_at = Some(value);
        self
    }

    pub fn scopes(mut self, value: Vec<String>) -> Self {
        self.scopes = Some(value);
        self
    }

    pub fn tenant_id(mut self, value: impl Into<String>) -> Self {
        self.tenant_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SdkKey`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](SdkKeyBuilder::created_at)
    /// - [`id`](SdkKeyBuilder::id)
    /// - [`name`](SdkKeyBuilder::name)
    /// - [`prefix`](SdkKeyBuilder::prefix)
    /// - [`scopes`](SdkKeyBuilder::scopes)
    /// - [`tenant_id`](SdkKeyBuilder::tenant_id)
    pub fn build(self) -> Result<SdkKey, BuildError> {
        Ok(SdkKey {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_used_at: self.last_used_at,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            prefix: self
                .prefix
                .ok_or_else(|| BuildError::missing_field("prefix"))?,
            revoked_at: self.revoked_at,
            scopes: self
                .scopes
                .ok_or_else(|| BuildError::missing_field("scopes"))?,
            tenant_id: self
                .tenant_id
                .ok_or_else(|| BuildError::missing_field("tenant_id"))?,
        })
    }
}
