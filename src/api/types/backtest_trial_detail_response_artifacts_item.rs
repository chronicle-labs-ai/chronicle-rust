pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BacktestTrialDetailResponseArtifactsItem {
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub id: String,
    /// Row projection of `"BacktestArtifact"`.
    pub kind: BacktestTrialDetailResponseArtifactsItemKind,
    #[serde(default)]
    pub path: String,
    #[serde(rename = "sizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
    #[serde(rename = "trialId")]
    #[serde(default)]
    pub trial_id: String,
}

impl BacktestTrialDetailResponseArtifactsItem {
    pub fn builder() -> BacktestTrialDetailResponseArtifactsItemBuilder {
        <BacktestTrialDetailResponseArtifactsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BacktestTrialDetailResponseArtifactsItemBuilder {
    content_type: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    kind: Option<BacktestTrialDetailResponseArtifactsItemKind>,
    path: Option<String>,
    size_bytes: Option<i64>,
    trial_id: Option<String>,
}

impl BacktestTrialDetailResponseArtifactsItemBuilder {
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: BacktestTrialDetailResponseArtifactsItemKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn size_bytes(mut self, value: i64) -> Self {
        self.size_bytes = Some(value);
        self
    }

    pub fn trial_id(mut self, value: impl Into<String>) -> Self {
        self.trial_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BacktestTrialDetailResponseArtifactsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](BacktestTrialDetailResponseArtifactsItemBuilder::created_at)
    /// - [`id`](BacktestTrialDetailResponseArtifactsItemBuilder::id)
    /// - [`kind`](BacktestTrialDetailResponseArtifactsItemBuilder::kind)
    /// - [`path`](BacktestTrialDetailResponseArtifactsItemBuilder::path)
    /// - [`trial_id`](BacktestTrialDetailResponseArtifactsItemBuilder::trial_id)
    pub fn build(self) -> Result<BacktestTrialDetailResponseArtifactsItem, BuildError> {
        Ok(BacktestTrialDetailResponseArtifactsItem {
            content_type: self.content_type,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            size_bytes: self.size_bytes,
            trial_id: self
                .trial_id
                .ok_or_else(|| BuildError::missing_field("trial_id"))?,
        })
    }
}
