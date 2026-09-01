use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SearchClient {
    pub http_client: HttpClient,
}

impl SearchClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Requires scope events:read or events:write. The page size is capped at 200 and a cursor can advance through at most 1,000 relevance-ranked results. Each request consumes 5 rate-limit units.
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
    ///         .search
    ///         .events(
    ///             &SearchRequest {
    ///                 query: "query".to_string(),
    ///                 source: None,
    ///                 entity_type: None,
    ///                 entity_id: None,
    ///                 limit: None,
    ///                 cursor: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn events(
        &self,
        request: &SearchRequest,
        options: Option<RequestOptions>,
    ) -> Result<EventListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/search",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
