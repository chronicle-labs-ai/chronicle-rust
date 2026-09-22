pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentSnapshotVersionsItemArtifactToolsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "inputSchemaHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema_hash: Option<String>,
    /// Optional small input-schema preview rendered in the Tools tab.
    #[serde(rename = "inputSchemaPreview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema_preview: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub name: String,
}

impl AgentSnapshotVersionsItemArtifactToolsItem {
    pub fn builder() -> AgentSnapshotVersionsItemArtifactToolsItemBuilder {
        <AgentSnapshotVersionsItemArtifactToolsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSnapshotVersionsItemArtifactToolsItemBuilder {
    description: Option<String>,
    input_schema_hash: Option<String>,
    input_schema_preview: Option<HashMap<String, serde_json::Value>>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    name: Option<String>,
}

impl AgentSnapshotVersionsItemArtifactToolsItemBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn input_schema_hash(mut self, value: impl Into<String>) -> Self {
        self.input_schema_hash = Some(value.into());
        self
    }

    pub fn input_schema_preview(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.input_schema_preview = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentSnapshotVersionsItemArtifactToolsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](AgentSnapshotVersionsItemArtifactToolsItemBuilder::name)
    pub fn build(self) -> Result<AgentSnapshotVersionsItemArtifactToolsItem, BuildError> {
        Ok(AgentSnapshotVersionsItemArtifactToolsItem {
            description: self.description,
            input_schema_hash: self.input_schema_hash,
            input_schema_preview: self.input_schema_preview,
            metadata: self.metadata,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
