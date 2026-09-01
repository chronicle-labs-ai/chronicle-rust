pub use crate::prelude::*;

/// The inferred payload shape for one source and event type.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SourceSchema {
    #[serde(default)]
    pub org_id: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub event_type: String,
    /// Bumped when the inferred shape changes.
    #[serde(default)]
    pub version: i64,
    #[serde(default)]
    pub field_names: Vec<String>,
    #[serde(default)]
    pub field_types: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_event: Option<HashMap<String, serde_json::Value>>,
}

impl SourceSchema {
    pub fn builder() -> SourceSchemaBuilder {
        <SourceSchemaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourceSchemaBuilder {
    org_id: Option<String>,
    source: Option<String>,
    event_type: Option<String>,
    version: Option<i64>,
    field_names: Option<Vec<String>>,
    field_types: Option<Vec<String>>,
    sample_event: Option<HashMap<String, serde_json::Value>>,
}

impl SourceSchemaBuilder {
    pub fn org_id(mut self, value: impl Into<String>) -> Self {
        self.org_id = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn event_type(mut self, value: impl Into<String>) -> Self {
        self.event_type = Some(value.into());
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn field_names(mut self, value: Vec<String>) -> Self {
        self.field_names = Some(value);
        self
    }

    pub fn field_types(mut self, value: Vec<String>) -> Self {
        self.field_types = Some(value);
        self
    }

    pub fn sample_event(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.sample_event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SourceSchema`].
    /// This method will fail if any of the following fields are not set:
    /// - [`org_id`](SourceSchemaBuilder::org_id)
    /// - [`source`](SourceSchemaBuilder::source)
    /// - [`event_type`](SourceSchemaBuilder::event_type)
    /// - [`version`](SourceSchemaBuilder::version)
    /// - [`field_names`](SourceSchemaBuilder::field_names)
    /// - [`field_types`](SourceSchemaBuilder::field_types)
    pub fn build(self) -> Result<SourceSchema, BuildError> {
        Ok(SourceSchema {
            org_id: self
                .org_id
                .ok_or_else(|| BuildError::missing_field("org_id"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            event_type: self
                .event_type
                .ok_or_else(|| BuildError::missing_field("event_type"))?,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
            field_names: self
                .field_names
                .ok_or_else(|| BuildError::missing_field("field_names"))?,
            field_types: self
                .field_types
                .ok_or_else(|| BuildError::missing_field("field_types"))?,
            sample_event: self.sample_event,
        })
    }
}
