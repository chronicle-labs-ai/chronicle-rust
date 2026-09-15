use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TimelineClient {
    pub http_client: HttpClient,
}

impl TimelineClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Requires scope events:read or events:write. Cursor paginated, newest first.
    ///
    /// Pass `cursor` from `next_cursor` to read the following page, and stop when `has_more` is false. The cursor is opaque: it is a keyset over `(event_time, event_id)`, it is exclusive so a row cannot repeat across pages, and its encoding may change without notice. Do not parse or construct one.
    ///
    /// `include_linked=true` selects a different read that also returns causally linked events. That read is not paginated: it returns one page with `has_more` false, and it cannot be combined with `limit` or `cursor`. `since` is only available on that read, because the paginated read has no time filter.
    ///
    /// # Arguments
    ///
    /// * `limit` - Page size. Values above the maximum are reduced to it, not rejected.
    /// * `cursor` - Opaque cursor from a previous response's next_cursor
    /// * `since` - Relative time window, for example last_7d.
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
    ///         .timeline
    ///         .get_timeline(
    ///             &"entity_type".to_string(),
    ///             &"entity_id".to_string(),
    ///             &GetTimelineQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_timeline(
        &self,
        entity_type: &str,
        entity_id: &str,
        request: &GetTimelineQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<EventPage, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/timeline/{}/{}", entity_type, entity_id),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .string("cursor", request.cursor.clone())
                    .string("since", request.since.clone())
                    .bool("include_linked", request.include_linked.clone())
                    .build(),
                options,
            )
            .await
    }
}
