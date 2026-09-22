pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RecordAgentRunsRequestRunsItemResponse {
    #[serde(rename = "bodyHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_hash: Option<String>,
    #[serde(rename = "finishReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
    /// Subset of headers preserved for the run drawer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, Option<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "modelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(rename = "modelMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "providerMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<RecordAgentRunsRequestRunsItemResponseUsage>,
}

impl RecordAgentRunsRequestRunsItemResponse {
    pub fn builder() -> RecordAgentRunsRequestRunsItemResponseBuilder {
        <RecordAgentRunsRequestRunsItemResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecordAgentRunsRequestRunsItemResponseBuilder {
    body_hash: Option<String>,
    finish_reason: Option<String>,
    headers: Option<HashMap<String, Option<String>>>,
    id: Option<String>,
    model_id: Option<String>,
    model_metadata: Option<HashMap<String, serde_json::Value>>,
    provider_metadata: Option<HashMap<String, serde_json::Value>>,
    usage: Option<RecordAgentRunsRequestRunsItemResponseUsage>,
}

impl RecordAgentRunsRequestRunsItemResponseBuilder {
    pub fn body_hash(mut self, value: impl Into<String>) -> Self {
        self.body_hash = Some(value.into());
        self
    }

    pub fn finish_reason(mut self, value: impl Into<String>) -> Self {
        self.finish_reason = Some(value.into());
        self
    }

    pub fn headers(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    pub fn model_metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.model_metadata = Some(value);
        self
    }

    pub fn provider_metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.provider_metadata = Some(value);
        self
    }

    pub fn usage(mut self, value: RecordAgentRunsRequestRunsItemResponseUsage) -> Self {
        self.usage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecordAgentRunsRequestRunsItemResponse`].
    pub fn build(self) -> Result<RecordAgentRunsRequestRunsItemResponse, BuildError> {
        Ok(RecordAgentRunsRequestRunsItemResponse {
            body_hash: self.body_hash,
            finish_reason: self.finish_reason,
            headers: self.headers,
            id: self.id,
            model_id: self.model_id,
            model_metadata: self.model_metadata,
            provider_metadata: self.provider_metadata,
            usage: self.usage,
        })
    }
}
