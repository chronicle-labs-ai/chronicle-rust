pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EnvironmentRecord {
    #[serde(rename = "archivedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub slug: String,
    #[serde(rename = "tenantId")]
    #[serde(default)]
    pub tenant_id: String,
}

impl EnvironmentRecord {
    pub fn builder() -> EnvironmentRecordBuilder {
        <EnvironmentRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnvironmentRecordBuilder {
    archived_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    id: Option<String>,
    label: Option<String>,
    slug: Option<String>,
    tenant_id: Option<String>,
}

impl EnvironmentRecordBuilder {
    pub fn archived_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.archived_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn tenant_id(mut self, value: impl Into<String>) -> Self {
        self.tenant_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EnvironmentRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](EnvironmentRecordBuilder::created_at)
    /// - [`id`](EnvironmentRecordBuilder::id)
    /// - [`label`](EnvironmentRecordBuilder::label)
    /// - [`slug`](EnvironmentRecordBuilder::slug)
    /// - [`tenant_id`](EnvironmentRecordBuilder::tenant_id)
    pub fn build(self) -> Result<EnvironmentRecord, BuildError> {
        Ok(EnvironmentRecord {
            archived_at: self.archived_at,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            description: self.description,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            tenant_id: self
                .tenant_id
                .ok_or_else(|| BuildError::missing_field("tenant_id"))?,
        })
    }
}
