use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct LinksClient {
    pub http_client: HttpClient,
}

impl LinksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Requires scope events:write.
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
    ///         .links
    ///         .add_entity_ref(
    ///             &AddEntityRefRequest {
    ///                 event_id: "event_id".to_string(),
    ///                 entity_type: "entity_type".to_string(),
    ///                 entity_id: "entity_id".to_string(),
    ///                 created_by: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_entity_ref(
        &self,
        request: &AddEntityRefRequest,
        options: Option<RequestOptions>,
    ) -> Result<StatusResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/entity-refs",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Requires scope events:write.
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
    ///         .links
    ///         .create_event_link(
    ///             &CreateLinkRequest {
    ///                 source_event_id: "source_event_id".to_string(),
    ///                 target_event_id: "target_event_id".to_string(),
    ///                 link_type: "link_type".to_string(),
    ///                 confidence: 1.1,
    ///                 reasoning: None,
    ///                 created_by: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_event_link(
        &self,
        request: &CreateLinkRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateLinkResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/event-links",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Requires scope events:write.
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
    ///         .links
    ///         .link_entities(
    ///             &LinkEntityRequest {
    ///                 from_entity_type: "from_entity_type".to_string(),
    ///                 from_entity_id: "from_entity_id".to_string(),
    ///                 to_entity_type: "to_entity_type".to_string(),
    ///                 to_entity_id: "to_entity_id".to_string(),
    ///                 created_by: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn link_entities(
        &self,
        request: &LinkEntityRequest,
        options: Option<RequestOptions>,
    ) -> Result<LinkEntityResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/link-entity",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Requires scope events:read or events:write. The traversal is bounded by `max_depth`, is not cursor-paginated, and consumes 5 rate-limit units.
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
    ///         .links
    ///         .traverse_graph(
    ///             &GraphRequest {
    ///                 start_event_id: "start_event_id".to_string(),
    ///                 direction: GraphRequestDirection::Outgoing,
    ///                 link_types: None,
    ///                 max_depth: None,
    ///                 min_confidence: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn traverse_graph(
        &self,
        request: &GraphRequest,
        options: Option<RequestOptions>,
    ) -> Result<EventListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/graph",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
