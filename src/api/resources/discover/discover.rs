use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct DiscoverClient {
    pub http_client: HttpClient,
}

impl DiscoverClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Requires scope events:read or events:write. Returns the complete source metadata set without pagination.
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
    ///     client.discover.list_sources(None).await;
    /// }
    /// ```
    pub async fn list_sources(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<SourceListResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/discover/sources", None, None, options)
            .await
    }

    /// Requires scope events:read or events:write. Returns the complete entity-type metadata set without pagination.
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
    ///     client.discover.list_entity_types(None).await;
    /// }
    /// ```
    pub async fn list_entity_types(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<EntityTypeListResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/discover/entity-types", None, None, options)
            .await
    }

    /// Requires scope events:read or events:write. Entities are ordered by event count and entity ID. The limit is capped at 200; pass `next_cursor` as `cursor`.
    ///
    /// # Arguments
    ///
    /// * `limit` - Page size. Values above 200 are clamped to 200.
    /// * `cursor` - Opaque position returned as `next_cursor` by the preceding page.
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
    ///         .discover
    ///         .list_entities(
    ///             &"entity_type".to_string(),
    ///             &ListEntitiesQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_entities(
        &self,
        entity_type: &str,
        request: &ListEntitiesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<EntityListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/discover/entities/{}", entity_type),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Requires scope events:read or events:write.
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
    ///         .discover
    ///         .get_event_schema(&"source".to_string(), &"event_type".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_event_schema(
        &self,
        source: &str,
        event_type: &str,
        options: Option<RequestOptions>,
    ) -> Result<SourceSchema, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/discover/schema/{}/{}", source, event_type),
                None,
                None,
                options,
            )
            .await
    }
}
