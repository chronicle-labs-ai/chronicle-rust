pub use crate::prelude::*;

/// Optional environment the run targets. Pipeline step 03 sets this; consumers without an environment fall back to the default ephemeral sandbox.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateBacktestJobRequestRecipeEnvironment {
    /// Whether this environment is a freshly cloned ephemeral sandbox or a saved long-lived environment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<bool>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub label: String,
    #[serde(rename = "snapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[serde(rename = "snapshotLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_label: Option<String>,
    /// Mirror of `SandboxRuntimeStatus` strings ("started", "stopped", …). Kept loose so callers don't need to import the environments package.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl CreateBacktestJobRequestRecipeEnvironment {
    pub fn builder() -> CreateBacktestJobRequestRecipeEnvironmentBuilder {
        <CreateBacktestJobRequestRecipeEnvironmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBacktestJobRequestRecipeEnvironmentBuilder {
    ephemeral: Option<bool>,
    id: Option<String>,
    label: Option<String>,
    snapshot_id: Option<String>,
    snapshot_label: Option<String>,
    status: Option<String>,
}

impl CreateBacktestJobRequestRecipeEnvironmentBuilder {
    pub fn ephemeral(mut self, value: bool) -> Self {
        self.ephemeral = Some(value);
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

    pub fn snapshot_id(mut self, value: impl Into<String>) -> Self {
        self.snapshot_id = Some(value.into());
        self
    }

    pub fn snapshot_label(mut self, value: impl Into<String>) -> Self {
        self.snapshot_label = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateBacktestJobRequestRecipeEnvironment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateBacktestJobRequestRecipeEnvironmentBuilder::id)
    /// - [`label`](CreateBacktestJobRequestRecipeEnvironmentBuilder::label)
    pub fn build(self) -> Result<CreateBacktestJobRequestRecipeEnvironment, BuildError> {
        Ok(CreateBacktestJobRequestRecipeEnvironment {
            ephemeral: self.ephemeral,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            snapshot_id: self.snapshot_id,
            snapshot_label: self.snapshot_label,
            status: self.status,
        })
    }
}
