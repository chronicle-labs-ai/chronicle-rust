use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions, SseStream};
use reqwest::Method;
use std::collections::HashMap;

pub struct AgentsClient {
    pub http_client: HttpClient,
}

impl AgentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// # Examples
    ///
    /// ```no_run
    /// use chroniclelabs::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Chronicle::new(config).expect("Failed to build client");
    ///     client.agents.list_agents(None).await;
    /// }
    /// ```
    pub async fn list_agents(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Vec<AgentSummary>, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/agents", None, None, options)
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use chroniclelabs::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Chronicle::new(config).expect("Failed to build client");
    ///     client
    ///         .agents
    ///         .search_agent_hash_index(
    ///             &SearchAgentHashIndexQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search_agent_hash_index(
        &self,
        request: &SearchAgentHashIndexQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<HashIndexEntry>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/agents/hash-index",
                None,
                QueryBuilder::new()
                    .string("q", request.q.clone())
                    .string("domains", request.domains.clone())
                    .build(),
                options,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use chroniclelabs::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Chronicle::new(config).expect("Failed to build client");
    ///     client.agents.subscribe_to_agent_changes(None).await;
    /// }
    /// ```
    pub async fn subscribe_to_agent_changes(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<SseStream<HashMap<String, serde_json::Value>>, ApiError> {
        self.http_client
            .execute_sse_request(
                Method::GET,
                "v1/agents/subscribe",
                None,
                None,
                options,
                None,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use chroniclelabs::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Chronicle::new(config).expect("Failed to build client");
    ///     client
    ///         .agents
    ///         .update_agent(
    ///             &"name".to_string(),
    ///             &UpdateAgentRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_agent(
        &self,
        name: &str,
        request: &UpdateAgentRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgentSummary, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/agents/{}", name),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use chroniclelabs::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Chronicle::new(config).expect("Failed to build client");
    ///     client
    ///         .agents
    ///         .get_agent_snapshot(&"name".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_agent_snapshot(
        &self,
        name: &str,
        options: Option<RequestOptions>,
    ) -> Result<Option<AgentSnapshot>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/agents/{}/snapshot", name),
                None,
                None,
                options,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use chroniclelabs::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Chronicle::new(config).expect("Failed to build client");
    ///     client
    ///         .agents
    ///         .pin_latest_agent_version(&"name".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn pin_latest_agent_version(
        &self,
        name: &str,
        options: Option<RequestOptions>,
    ) -> Result<AgentSummary, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/agents/{}/pin-latest", name),
                None,
                None,
                options,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use chroniclelabs::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Chronicle::new(config).expect("Failed to build client");
    ///     client
    ///         .agents
    ///         .create_agent_chat_session(&"name".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn create_agent_chat_session(
        &self,
        name: &str,
        options: Option<RequestOptions>,
    ) -> Result<CreateAgentChatSessionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/agents/{}/chat/sessions", name),
                None,
                None,
                options,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use chroniclelabs::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Chronicle::new(config).expect("Failed to build client");
    ///     client
    ///         .agents
    ///         .get_agent_chat_session(&"name".to_string(), &"session_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_agent_chat_session(
        &self,
        name: &str,
        session_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AgentChatSession, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/agents/{}/chat/sessions/{}", name, session_id),
                None,
                None,
                options,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use chroniclelabs::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Chronicle::new(config).expect("Failed to build client");
    ///     client
    ///         .agents
    ///         .send_agent_chat_message(
    ///             &"name".to_string(),
    ///             &"session_id".to_string(),
    ///             &SendAgentChatMessageRequest {
    ///                 text: "text".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn send_agent_chat_message(
        &self,
        name: &str,
        session_id: &str,
        request: &SendAgentChatMessageRequest,
        options: Option<RequestOptions>,
    ) -> Result<SendAgentChatMessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/agents/{}/chat/sessions/{}/messages", name, session_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Requires scope agents:write.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use chroniclelabs::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Chronicle::new(config).expect("Failed to build client");
    ///     client
    ///         .agents
    ///         .register_agent_artifact(
    ///             &RegisterAgentArtifactRequest {
    ///                 artifact: RegisterAgentArtifactRequestArtifact {
    ///                     artifact_id: "artifactId".to_string(),
    ///                     config_hash: "configHash".to_string(),
    ///                     description: None,
    ///                     framework: RegisterAgentArtifactRequestArtifactFramework::VercelAiSdk,
    ///                     input_contract_preview: None,
    ///                     instructions: None,
    ///                     instructions_hash: None,
    ///                     knowledge_sources: None,
    ///                     metadata: None,
    ///                     model: RegisterAgentArtifactRequestArtifactModel {
    ///                         label: "label".to_string(),
    ///                         ..Default::default()
    ///                     },
    ///                     name: "name".to_string(),
    ///                     output_contract_preview: None,
    ///                     policy: None,
    ///                     provenance: RegisterAgentArtifactRequestArtifactProvenance {
    ///                         created_at: DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z").unwrap(),
    ///                         ..Default::default()
    ///                     },
    ///                     provider_options: None,
    ///                     provider_options_hash: None,
    ///                     schema_version: "schemaVersion".to_string(),
    ///                     tools: vec![RegisterAgentArtifactRequestArtifactToolsItem {
    ///                         name: "name".to_string(),
    ///                         ..Default::default()
    ///                     }],
    ///                     version: "version".to_string(),
    ///                     workflow_graph_preview: None,
    ///                 },
    ///                 metadata: None,
    ///                 status: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn register_agent_artifact(
        &self,
        request: &RegisterAgentArtifactRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgentVersionSummary, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/agents/register",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Requires scope agents:write.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use chroniclelabs::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Chronicle::new(config).expect("Failed to build client");
    ///     client
    ///         .agents
    ///         .record_agent_runs(
    ///             &RecordAgentRunsRequest {
    ///                 runs: vec![RecordAgentRunsRequestRunsItem {
    ///                     artifact_id: "artifactId".to_string(),
    ///                     call_options_hash: None,
    ///                     config_hash: "configHash".to_string(),
    ///                     duration_ms: None,
    ///                     error: None,
    ///                     finished_at: None,
    ///                     input_hash: None,
    ///                     operation: RecordAgentRunsRequestRunsItemOperation::Generate,
    ///                     prepared_call: None,
    ///                     response: None,
    ///                     run_id: "runId".to_string(),
    ///                     schema_version: "schemaVersion".to_string(),
    ///                     started_at: DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z").unwrap(),
    ///                     status: RecordAgentRunsRequestRunsItemStatus::Started,
    ///                     tool_calls: vec![RecordAgentRunsRequestRunsItemToolCallsItem {
    ///                         args_hash: None,
    ///                         args_preview: None,
    ///                         call_id: "callId".to_string(),
    ///                         duration_ms: None,
    ///                         error: None,
    ///                         finished_at: None,
    ///                         result_hash: None,
    ///                         result_preview: None,
    ///                         started_at: DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z").unwrap(),
    ///                         status: RecordAgentRunsRequestRunsItemToolCallsItemStatus::Started,
    ///                         tool_name: "toolName".to_string(),
    ///                     }],
    ///                     trace: None,
    ///                 }],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn record_agent_runs(
        &self,
        request: &RecordAgentRunsRequest,
        options: Option<RequestOptions>,
    ) -> Result<RecordAgentRunsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/agents/runs/batch",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
