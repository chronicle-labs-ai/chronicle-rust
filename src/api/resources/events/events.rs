use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions, SseStream};
use reqwest::Method;

pub struct EventsClient {
    pub http_client: HttpClient,
}

impl EventsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Requires scope events:read or events:write. Results are scoped to the tenant of the API key and ordered newest first by event time and event ID. Pass the opaque `next_cursor` as `cursor` to continue without an offset scan.
    ///
    /// # Arguments
    ///
    /// * `limit` - Page size. Values above 200 are clamped to 200.
    /// * `cursor` - Opaque position returned as `next_cursor` by the preceding page.
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
    ///         .events
    ///         .query_events(
    ///             &QueryEventsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn query_events(
        &self,
        request: &QueryEventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<EventListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/events",
                None,
                QueryBuilder::new()
                    .string("source", request.source.clone())
                    .string("topic", request.topic.clone())
                    .string("event_type", request.event_type.clone())
                    .string("entity_type", request.entity_type.clone())
                    .string("entity_id", request.entity_id.clone())
                    .int("limit", request.limit.clone())
                    .string("cursor", request.cursor.clone())
                    .string("since", request.since.clone())
                    .build(),
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
    ///         .events
    ///         .ingest_event(
    ///             &IngestRequest {
    ///                 source: "my-agent".to_string(),
    ///                 topic: "conversations".to_string(),
    ///                 event_type: "message.sent".to_string(),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn ingest_event(
        &self,
        request: &IngestRequest,
        options: Option<RequestOptions>,
    ) -> Result<IngestResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/events",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Requires scope events:write. Maximum 1000 events per batch; larger batches are rejected with 422. Request bodies over the size limit are rejected with 413. Each request consumes 10 rate-limit units.
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
    ///         .events
    ///         .ingest_event_batch(
    ///             &vec![IngestRequest {
    ///                 source: "my-agent".to_string(),
    ///                 topic: "conversations".to_string(),
    ///                 event_type: "message.sent".to_string(),
    ///                 ..Default::default()
    ///             }],
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn ingest_event_batch(
        &self,
        request: &Vec<IngestRequest>,
        options: Option<RequestOptions>,
    ) -> Result<IngestResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/events/batch",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Requires scope events:read or events:write. A Server-Sent Events stream of events matching the optional filters, held open indefinitely.
    ///
    /// Opening a stream consumes 5 rate-limit units.
    ///
    /// Each message has `event: event` and a `data` field carrying one EventResult as JSON. A comment line arrives every 15 seconds so intermediaries do not close an idle connection.
    ///
    /// Every message carries an opaque, stream-specific `id` backed by a monotonic per-tenant delivery sequence. It records ingestion order, independently of the source event's `event_time`. Record the last id you processed and do not parse or construct it.
    ///
    /// When `Last-Event-ID` is present, the server first establishes the live subscription, replays matching stored events strictly after that position in ascending order, and then continues with live delivery. Events committed at the history-to-live boundary may be delivered more than once, so consumers should deduplicate by `event_id`. This provides at-least-once delivery across a reconnect without leaving a gap.
    ///
    /// Replay is limited to 1000 matching events. An older position returns 409 before the stream opens. Slow consumers are disconnected when the bounded live buffer fills and should reconnect with their last processed id. Concurrent streams are limited per tenant and may return 429 with `Retry-After`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Server-Sent Events stream (use futures::StreamExt to iterate)
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
    ///         .events
    ///         .stream_events(
    ///             &StreamEventsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn stream_events(
        &self,
        request: &StreamEventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SseStream<EventResult>, ApiError> {
        self.http_client
            .execute_sse_request(
                Method::GET,
                "v1/events/stream",
                None,
                QueryBuilder::new()
                    .string("source", request.source.clone())
                    .string("event_type", request.event_type.clone())
                    .string("entity_type", request.entity_type.clone())
                    .string("entity_id", request.entity_id.clone())
                    .build(),
                options,
                None,
            )
            .await
    }
}
