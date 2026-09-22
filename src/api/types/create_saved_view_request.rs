pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateSavedViewRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "idempotencyKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_saved_view_request_idempotency_key: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "schemaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<i64>,
    pub scope: CreateSavedViewRequestScope,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortcut: Option<String>,
    #[serde(default)]
    pub state: CreateSavedViewRequestState,
}

impl CreateSavedViewRequest {
    pub fn builder() -> CreateSavedViewRequestBuilder {
        <CreateSavedViewRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSavedViewRequestBuilder {
    description: Option<String>,
    create_saved_view_request_idempotency_key: Option<String>,
    name: Option<String>,
    schema_version: Option<i64>,
    scope: Option<CreateSavedViewRequestScope>,
    shortcut: Option<String>,
    state: Option<CreateSavedViewRequestState>,
}

impl CreateSavedViewRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn create_saved_view_request_idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.create_saved_view_request_idempotency_key = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn schema_version(mut self, value: i64) -> Self {
        self.schema_version = Some(value);
        self
    }

    pub fn scope(mut self, value: CreateSavedViewRequestScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn shortcut(mut self, value: impl Into<String>) -> Self {
        self.shortcut = Some(value.into());
        self
    }

    pub fn state(mut self, value: CreateSavedViewRequestState) -> Self {
        self.state = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSavedViewRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateSavedViewRequestBuilder::name)
    /// - [`scope`](CreateSavedViewRequestBuilder::scope)
    /// - [`state`](CreateSavedViewRequestBuilder::state)
    pub fn build(self) -> Result<CreateSavedViewRequest, BuildError> {
        Ok(CreateSavedViewRequest {
            description: self.description,
            create_saved_view_request_idempotency_key: self
                .create_saved_view_request_idempotency_key,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            schema_version: self.schema_version,
            scope: self
                .scope
                .ok_or_else(|| BuildError::missing_field("scope"))?,
            shortcut: self.shortcut,
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}
