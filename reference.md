# Reference
## events
<details><summary><code>client.events.<a href="/src/api/resources/events/client.rs">query_events</a>(source: Option&lt;Option&lt;String&gt;&gt;, topic: Option&lt;Option&lt;String&gt;&gt;, event_type: Option&lt;Option&lt;String&gt;&gt;, entity_type: Option&lt;Option&lt;String&gt;&gt;, entity_id: Option&lt;Option&lt;String&gt;&gt;, limit: Option&lt;Option&lt;i64&gt;&gt;, cursor: Option&lt;Option&lt;String&gt;&gt;, since: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;EventListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires scope events:read or events:write. Results are scoped to the tenant of the API key and ordered newest first by event time and event ID. Pass the opaque `next_cursor` as `cursor` to continue without an offset scan.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .events
        .query_events(
            &QueryEventsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**source:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**topic:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**event_type:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**entity_type:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**entity_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` — Page size. Values above 200 are clamped to 200.
    
</dd>
</dl>

<dl>
<dd>

**cursor:** `Option<String>` — Opaque position returned as `next_cursor` by the preceding page.
    
</dd>
</dl>

<dl>
<dd>

**since:** `Option<String>` — Relative time window, for example last_7d.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.events.<a href="/src/api/resources/events/client.rs">ingest_event</a>(request: IngestRequest) -> Result&lt;IngestResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires scope events:write.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .events
        .ingest_event(
            &IngestRequest {
                source: "support-agent".to_string(),
                topic: "conversations".to_string(),
                event_type: "message.sent".to_string(),
                entities: Some(HashMap::from([("user".to_string(), "usr_123".to_string())])),
                payload: Some(
                    serde_json::json!({"role":"assistant","content":"Your refund is approved."}),
                ),
                timestamp: Some(DateTime::parse_from_rfc3339("2026-09-24T14:30:00Z").unwrap()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.events.<a href="/src/api/resources/events/client.rs">ingest_event_batch</a>(request: Vec&lt;IngestRequest&gt;) -> Result&lt;IngestResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires scope events:write. Maximum 1000 events per batch; larger batches are rejected with 422. Request bodies over the size limit are rejected with 413. Each request consumes 10 rate-limit units.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .events
        .ingest_event_batch(
            &vec![IngestRequest {
                source: "my-agent".to_string(),
                topic: "conversations".to_string(),
                event_type: "message.sent".to_string(),
                ..Default::default()
            }],
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.events.<a href="/src/api/resources/events/client.rs">stream_events</a>(source: Option&lt;Option&lt;String&gt;&gt;, event_type: Option&lt;Option&lt;String&gt;&gt;, entity_type: Option&lt;Option&lt;String&gt;&gt;, entity_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;Stream&lt;Vec&lt;u8&gt;&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires scope events:read or events:write. A Server-Sent Events stream of events matching the optional filters, held open indefinitely.

Opening a stream consumes 5 rate-limit units.

Each message has `event: event` and a `data` field carrying one EventResult as JSON. A comment line arrives every 15 seconds so intermediaries do not close an idle connection.

Every message carries an opaque, stream-specific `id` backed by a monotonic per-tenant delivery sequence. It records ingestion order, independently of the source event's `event_time`. Record the last id you processed and do not parse or construct it.

When `Last-Event-ID` is present, the server first establishes the live subscription, replays matching stored events strictly after that position in ascending order, and then continues with live delivery. Events committed at the history-to-live boundary may be delivered more than once, so consumers should deduplicate by `event_id`. This provides at-least-once delivery across a reconnect without leaving a gap.

Replay is limited to 1000 matching events. An older position returns 409 before the stream opens. Slow consumers are disconnected when the bounded live buffer fills and should reconnect with their last processed id. Concurrent streams are limited per tenant and may return 429 with `Retry-After`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .events
        .stream_events(
            &StreamEventsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**source:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**event_type:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**entity_type:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**entity_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## timeline
<details><summary><code>client.timeline.<a href="/src/api/resources/timeline/client.rs">get_timeline</a>(entity_type: String, entity_id: String, limit: Option&lt;Option&lt;i64&gt;&gt;, cursor: Option&lt;Option&lt;String&gt;&gt;, since: Option&lt;Option&lt;String&gt;&gt;, include_linked: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;EventPage, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires scope events:read or events:write. Cursor paginated, newest first.

Pass `cursor` from `next_cursor` to read the following page, and stop when `has_more` is false. The cursor is opaque: it is a keyset over `(event_time, event_id)`, it is exclusive so a row cannot repeat across pages, and its encoding may change without notice. Do not parse or construct one.

`include_linked=true` selects a different read that also returns causally linked events. That read is not paginated: it returns one page with `has_more` false, and it cannot be combined with `limit` or `cursor`. `since` is only available on that read, because the paginated read has no time filter.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .timeline
        .get_timeline(
            &"entity_type".to_string(),
            &"entity_id".to_string(),
            &GetTimelineQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entity_type:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**entity_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` — Page size. Values above the maximum are reduced to it, not rejected.
    
</dd>
</dl>

<dl>
<dd>

**cursor:** `Option<String>` — Opaque cursor from a previous response's next_cursor
    
</dd>
</dl>

<dl>
<dd>

**since:** `Option<String>` — Relative time window, for example last_7d.
    
</dd>
</dl>

<dl>
<dd>

**include_linked:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## search
<details><summary><code>client.search.<a href="/src/api/resources/search/client.rs">events</a>(request: SearchRequest) -> Result&lt;EventListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires scope events:read or events:write. The page size is capped at 200 and a cursor can advance through at most 1,000 relevance-ranked results. Each request consumes 5 rate-limit units.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .search
        .events(
            &SearchRequest {
                query: "query".to_string(),
                source: None,
                entity_type: None,
                entity_id: None,
                limit: None,
                cursor: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**query:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**source:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**entity_type:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**entity_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**cursor:** `Option<Option<String>>` — Opaque position returned by the preceding search page.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## discover
<details><summary><code>client.discover.<a href="/src/api/resources/discover/client.rs">list_sources</a>() -> Result&lt;SourceListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires scope events:read or events:write. Returns the complete source metadata set without pagination.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client.discover.list_sources(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.discover.<a href="/src/api/resources/discover/client.rs">list_entity_types</a>() -> Result&lt;EntityTypeListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires scope events:read or events:write. Returns the complete entity-type metadata set without pagination.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client.discover.list_entity_types(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.discover.<a href="/src/api/resources/discover/client.rs">list_entities</a>(entity_type: String, limit: Option&lt;Option&lt;i64&gt;&gt;, cursor: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;EntityListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires scope events:read or events:write. Entities are ordered by event count and entity ID. The limit is capped at 200; pass `next_cursor` as `cursor`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .discover
        .list_entities(
            &"entity_type".to_string(),
            &ListEntitiesQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entity_type:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` — Page size. Values above 200 are clamped to 200.
    
</dd>
</dl>

<dl>
<dd>

**cursor:** `Option<String>` — Opaque position returned as `next_cursor` by the preceding page.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.discover.<a href="/src/api/resources/discover/client.rs">get_event_schema</a>(source: String, event_type: String) -> Result&lt;SourceSchema, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires scope events:read or events:write.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .discover
        .get_event_schema(&"source".to_string(), &"event_type".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**source:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**event_type:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## links
<details><summary><code>client.links.<a href="/src/api/resources/links/client.rs">add_entity_ref</a>(request: AddEntityRefRequest) -> Result&lt;StatusResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires scope events:write.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .links
        .add_entity_ref(
            &AddEntityRefRequest {
                event_id: "event_id".to_string(),
                entity_type: "entity_type".to_string(),
                entity_id: "entity_id".to_string(),
                created_by: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**event_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**entity_type:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**entity_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**created_by:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.links.<a href="/src/api/resources/links/client.rs">create_event_link</a>(request: CreateLinkRequest) -> Result&lt;CreateLinkResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires scope events:write.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .links
        .create_event_link(
            &CreateLinkRequest {
                source_event_id: "source_event_id".to_string(),
                target_event_id: "target_event_id".to_string(),
                link_type: "link_type".to_string(),
                confidence: 1.1,
                reasoning: None,
                created_by: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**source_event_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**target_event_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**link_type:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**confidence:** `f64` 
    
</dd>
</dl>

<dl>
<dd>

**reasoning:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**created_by:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.links.<a href="/src/api/resources/links/client.rs">link_entities</a>(request: LinkEntityRequest) -> Result&lt;LinkEntityResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires scope events:write.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .links
        .link_entities(
            &LinkEntityRequest {
                from_entity_type: "from_entity_type".to_string(),
                from_entity_id: "from_entity_id".to_string(),
                to_entity_type: "to_entity_type".to_string(),
                to_entity_id: "to_entity_id".to_string(),
                created_by: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from_entity_type:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**from_entity_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_entity_type:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_entity_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**created_by:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.links.<a href="/src/api/resources/links/client.rs">traverse_graph</a>(request: GraphRequest) -> Result&lt;EventListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires scope events:read or events:write. The traversal is bounded by `max_depth`, is not cursor-paginated, and consumes 5 rate-limit units.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .links
        .traverse_graph(
            &GraphRequest {
                start_event_id: "start_event_id".to_string(),
                direction: GraphRequestDirection::Outgoing,
                link_types: None,
                max_depth: None,
                min_confidence: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**start_event_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**direction:** `GraphRequestDirection` 
    
</dd>
</dl>

<dl>
<dd>

**link_types:** `Option<Option<Vec<String>>>` 
    
</dd>
</dl>

<dl>
<dd>

**max_depth:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**min_confidence:** `Option<f64>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## sdk
<details><summary><code>client.sdk.<a href="/src/api/resources/sdk/client.rs">identify_user</a>(request: IdentifyUserRequest) -> Result&lt;AcceptedResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires scope users:write.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .sdk
        .identify_user(
            &IdentifyUserRequest {
                user_id: "user_id".to_string(),
                traits: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**user_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**traits:** `Option<std::collections::HashMap<String, serde_json::Value>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sdk.<a href="/src/api/resources/sdk/client.rs">track_signals</a>(request: TrackSignalsRequest) -> Result&lt;AcceptedResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires scope signals:write. Maximum 1000 signals per request; larger batches are rejected with 422. Each request consumes 10 rate-limit units.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .sdk
        .track_signals(
            &TrackSignalsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**signals:** `Option<Vec<SignalRequest>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sdk.<a href="/src/api/resources/sdk/client.rs">track_traces</a>(request: TrackTracesRequest) -> Result&lt;AcceptedResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires scope traces:write. Maximum 1000 traces or total spans per request; larger batches are rejected with 422. Each request consumes 10 rate-limit units.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .sdk
        .track_traces(
            &TrackTracesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**traces:** `Option<Vec<TraceRequest>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## agents
<details><summary><code>client.agents.<a href="/src/api/resources/agents/client.rs">list_agents</a>() -> Result&lt;Vec&lt;AgentSummary&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client.agents.list_agents(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agents.<a href="/src/api/resources/agents/client.rs">search_agent_hash_index</a>(q: Option&lt;Option&lt;String&gt;&gt;, domains: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;Vec&lt;HashIndexEntry&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .agents
        .search_agent_hash_index(
            &SearchAgentHashIndexQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**q:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**domains:** `Option<String>` — Comma-separated hash domains.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agents.<a href="/src/api/resources/agents/client.rs">subscribe_to_agent_changes</a>() -> Result&lt;Stream&lt;Vec&lt;u8&gt;&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client.agents.subscribe_to_agent_changes(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agents.<a href="/src/api/resources/agents/client.rs">update_agent</a>(name: String, request: UpdateAgentRequest) -> Result&lt;AgentSummary, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .agents
        .update_agent(
            &"name".to_string(),
            &UpdateAgentRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**environment:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**owner:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**purpose:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agents.<a href="/src/api/resources/agents/client.rs">get_agent_snapshot</a>(name: String) -> Result&lt;Option&lt;AgentSnapshot&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .agents
        .get_agent_snapshot(&"name".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agents.<a href="/src/api/resources/agents/client.rs">pin_latest_agent_version</a>(name: String) -> Result&lt;AgentSummary, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .agents
        .pin_latest_agent_version(&"name".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agents.<a href="/src/api/resources/agents/client.rs">create_agent_chat_session</a>(name: String) -> Result&lt;CreateAgentChatSessionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .agents
        .create_agent_chat_session(&"name".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agents.<a href="/src/api/resources/agents/client.rs">get_agent_chat_session</a>(name: String, session_id: String) -> Result&lt;AgentChatSession, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .agents
        .get_agent_chat_session(&"name".to_string(), &"session_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agents.<a href="/src/api/resources/agents/client.rs">send_agent_chat_message</a>(name: String, session_id: String, request: SendAgentChatMessageRequest) -> Result&lt;SendAgentChatMessageResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .agents
        .send_agent_chat_message(
            &"name".to_string(),
            &"session_id".to_string(),
            &SendAgentChatMessageRequest {
                text: "text".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**text:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agents.<a href="/src/api/resources/agents/client.rs">register_agent_artifact</a>(request: RegisterAgentArtifactRequest) -> Result&lt;AgentVersionSummary, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires scope agents:write.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .agents
        .register_agent_artifact(
            &RegisterAgentArtifactRequest {
                artifact: RegisterAgentArtifactRequestArtifact {
                    artifact_id: "artifactId".to_string(),
                    config_hash: "configHash".to_string(),
                    description: None,
                    framework: RegisterAgentArtifactRequestArtifactFramework::VercelAiSdk,
                    input_contract_preview: None,
                    instructions: None,
                    instructions_hash: None,
                    knowledge_sources: None,
                    metadata: None,
                    model: RegisterAgentArtifactRequestArtifactModel {
                        label: "label".to_string(),
                        ..Default::default()
                    },
                    name: "name".to_string(),
                    output_contract_preview: None,
                    policy: None,
                    provenance: RegisterAgentArtifactRequestArtifactProvenance {
                        created_at: DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z").unwrap(),
                        ..Default::default()
                    },
                    provider_options: None,
                    provider_options_hash: None,
                    schema_version: "schemaVersion".to_string(),
                    tools: vec![RegisterAgentArtifactRequestArtifactToolsItem {
                        name: "name".to_string(),
                        ..Default::default()
                    }],
                    version: "version".to_string(),
                    workflow_graph_preview: None,
                },
                metadata: None,
                status: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**artifact:** `RegisterAgentArtifactRequestArtifact` 
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<Option<RegisterAgentArtifactRequestMetadata>>` — Mutable, human-authored metadata attached to a logical Agent identity. Artifact configuration remains immutable inside `AgentRegistryVersionRecord`.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<Option<RegisterAgentArtifactRequestStatus>>` — Defaults to `current`. Registering a new current version atomically demotes the previous current version to stable.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agents.<a href="/src/api/resources/agents/client.rs">record_agent_runs</a>(request: RecordAgentRunsRequest) -> Result&lt;RecordAgentRunsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Requires scope agents:write.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .agents
        .record_agent_runs(
            &RecordAgentRunsRequest {
                runs: vec![RecordAgentRunsRequestRunsItem {
                    artifact_id: "artifactId".to_string(),
                    call_options_hash: None,
                    config_hash: "configHash".to_string(),
                    duration_ms: None,
                    error: None,
                    finished_at: None,
                    input_hash: None,
                    operation: RecordAgentRunsRequestRunsItemOperation::Generate,
                    prepared_call: None,
                    response: None,
                    run_id: "runId".to_string(),
                    schema_version: "schemaVersion".to_string(),
                    started_at: DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z").unwrap(),
                    status: RecordAgentRunsRequestRunsItemStatus::Started,
                    tool_calls: vec![RecordAgentRunsRequestRunsItemToolCallsItem {
                        args_hash: None,
                        args_preview: None,
                        call_id: "callId".to_string(),
                        duration_ms: None,
                        error: None,
                        finished_at: None,
                        result_hash: None,
                        result_preview: None,
                        started_at: DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z").unwrap(),
                        status: RecordAgentRunsRequestRunsItemToolCallsItemStatus::Started,
                        tool_name: "toolName".to_string(),
                    }],
                    trace: None,
                }],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**runs:** `Vec<RecordAgentRunsRequestRunsItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## datasets
<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">list_datasets</a>(include_archived: Option&lt;Option&lt;bool&gt;&gt;, query: Option&lt;Option&lt;String&gt;&gt;, limit: Option&lt;Option&lt;i64&gt;&gt;, cursor: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;TaskSuitePage, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .list_datasets(
            &ListDatasetsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**include_archived:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**query:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**cursor:** `Option<String>` — Opaque position returned as `next_cursor` by the preceding page.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">create_dataset</a>(request: CreateTaskSuitePayload) -> Result&lt;TaskSuite, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .create_dataset(
            &CreateTaskSuitePayload {
                name: "name".to_string(),
                description: None,
                purpose: None,
                tags: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**description:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**purpose:** `Option<Option<CreateTaskSuitePayloadPurpose>>` — Intended use of a dataset — drives the colored badge on the picker and lets apps route additions to the right backend (eval suite, training set, replay corpus, manual review queue).
    
</dd>
</dl>

<dl>
<dd>

**tags:** `Option<Option<Vec<String>>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">create_dataset_with_trace</a>(request: CreateTaskSuiteWithTraceRequest) -> Result&lt;CreateTaskSuiteWithTraceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .create_dataset_with_trace(
            &CreateTaskSuiteWithTraceRequest {
                dataset: CreateTaskSuiteWithTraceRequestDataset {
                    name: "name".to_string(),
                    ..Default::default()
                },
                trace: CreateTaskSuiteWithTraceRequestTrace {
                    trace_id: "traceId".to_string(),
                    ..Default::default()
                },
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset:** `CreateTaskSuiteWithTraceRequestDataset` 
    
</dd>
</dl>

<dl>
<dd>

**trace:** `CreateTaskSuiteWithTraceRequestTrace` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">get_dataset</a>(dataset_id: String) -> Result&lt;TaskSuiteDetail, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .get_dataset(&"dataset_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">archive_dataset</a>(dataset_id: String, cascade: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .archive_dataset(
            &"dataset_id".to_string(),
            &ArchiveDatasetQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**cascade:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">update_dataset</a>(dataset_id: String, request: TaskSuitePatch) -> Result&lt;TaskSuite, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .update_dataset(
            &"dataset_id".to_string(),
            &TaskSuitePatch {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**purpose:** `Option<Option<TaskSuitePatchPurpose>>` — Intended use of a dataset — drives the colored badge on the picker and lets apps route additions to the right backend (eval suite, training set, replay corpus, manual review queue).
    
</dd>
</dl>

<dl>
<dd>

**tags:** `Option<Option<Vec<String>>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">get_dataset_snapshot</a>(dataset_id: String) -> Result&lt;TaskSuiteSnapshot, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .get_dataset_snapshot(&"dataset_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">list_dataset_traces</a>(dataset_id: String, limit: Option&lt;Option&lt;i64&gt;&gt;, cursor: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;TaskPage, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .list_dataset_traces(
            &"dataset_id".to_string(),
            &ListDatasetTracesQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**cursor:** `Option<String>` — Opaque position returned as `next_cursor` by the preceding page.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">add_trace_to_dataset</a>(dataset_id: String, request: AddTaskFromTraceRequest) -> Result&lt;AddTaskFromTraceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .add_trace_to_dataset(
            &"dataset_id".to_string(),
            &AddTaskFromTraceRequest {
                trace_id: "traceId".to_string(),
                event_ids: None,
                add_task_from_trace_request_idempotency_key: None,
                notes: None,
                split: None,
                task: None,
                trace_synthesized: None,
                verifiers: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**event_ids:** `Option<Vec<String>>` — Accepted for compatibility but never trusted as the authoritative capture. The service re-reads the canonical store by subject.
    
</dd>
</dl>

<dl>
<dd>

**add_task_from_trace_request_idempotency_key:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**split:** `Option<Option<AddTaskFromTraceRequestSplit>>` — Train / validation / test split assignment.
    
</dd>
</dl>

<dl>
<dd>

**task:** `Option<Option<AddTaskFromTraceRequestTask>>` — Optional task fields. Anything left unset is derived from the captured trace (title from the label, instruction from the first message, expected outcome from the events after the cutoff).
    
</dd>
</dl>

<dl>
<dd>

**trace_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**trace_synthesized:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**verifiers:** `Option<Vec<AddTaskFromTraceRequestVerifiersItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">update_dataset_traces</a>(dataset_id: String, request: UpdateTracesRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .update_dataset_traces(
            &"dataset_id".to_string(),
            &UpdateTracesRequest {
                patch: UpdateTracesRequestPatch {
                    ..Default::default()
                },
                trace_ids: vec!["traceIds".to_string()],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**patch:** `UpdateTracesRequestPatch` — Patch to apply to one or more memberships. Nullable annotations preserve the same three states as [`PatchField`]: explicit JSON `null` clears while omission is a no-op.
    
</dd>
</dl>

<dl>
<dd>

**trace_ids:** `Vec<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">remove_trace_from_dataset</a>(dataset_id: String, membership_id: String, reason: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .remove_trace_from_dataset(
            &"dataset_id".to_string(),
            &"membership_id".to_string(),
            &RemoveTraceFromDatasetQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**membership_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**reason:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">refresh_dataset_trace</a>(dataset_id: String, membership_id: String, request: RefreshMembershipRequest) -> Result&lt;TaskMembership, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .refresh_dataset_trace(
            &"dataset_id".to_string(),
            &"membership_id".to_string(),
            &RefreshMembershipRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**membership_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">list_dataset_trace_events</a>(dataset_id: String, membership_id: String, limit: Option&lt;Option&lt;i64&gt;&gt;, cursor: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;TaskEventPage, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .list_dataset_trace_events(
            &"dataset_id".to_string(),
            &"membership_id".to_string(),
            &ListDatasetTraceEventsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**membership_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**cursor:** `Option<String>` — Opaque position returned as `next_cursor` by the preceding page.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">list_trace_dataset_memberships</a>(trace_id: String) -> Result&lt;Vec&lt;TaskMembership&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .list_trace_dataset_memberships(&"trace_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**trace_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">list_dataset_tasks</a>(dataset_id: String, limit: Option&lt;Option&lt;i64&gt;&gt;, cursor: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;TaskPage, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .list_dataset_tasks(
            &"dataset_id".to_string(),
            &ListDatasetTasksQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**cursor:** `Option<String>` — Opaque position returned as `next_cursor` by the preceding page.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">create_dataset_task</a>(dataset_id: String, request: CreateTaskRequest) -> Result&lt;Task, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .create_dataset_task(
            &"dataset_id".to_string(),
            &CreateTaskRequest(serde_json::json!({"key":"value"})),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">get_dataset_task</a>(dataset_id: String, membership_id: String) -> Result&lt;Task, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .get_dataset_task(
            &"dataset_id".to_string(),
            &"membership_id".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**membership_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">delete_dataset_task</a>(dataset_id: String, membership_id: String, reason: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .delete_dataset_task(
            &"dataset_id".to_string(),
            &"membership_id".to_string(),
            &DeleteDatasetTaskQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**membership_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**reason:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">update_dataset_task</a>(dataset_id: String, membership_id: String, request: UpdateTaskRequest) -> Result&lt;Task, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .update_dataset_task(
            &"dataset_id".to_string(),
            &"membership_id".to_string(),
            &UpdateTaskRequest(serde_json::json!({"key":"value"})),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**membership_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">set_dataset_task_verifiers</a>(dataset_id: String, membership_id: String, request: SetTaskVerifiersRequest) -> Result&lt;Task, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .set_dataset_task_verifiers(
            &"dataset_id".to_string(),
            &"membership_id".to_string(),
            &SetTaskVerifiersRequest {
                verifiers: vec![SetTaskVerifiersRequestVerifiersItem {
                    scorer_id: "scorerId".to_string(),
                    ..Default::default()
                }],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**membership_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**verifiers:** `Vec<SetTaskVerifiersRequestVerifiersItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">list_dataset_task_events</a>(dataset_id: String, membership_id: String, limit: Option&lt;Option&lt;i64&gt;&gt;, cursor: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;TaskEventPage, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .list_dataset_task_events(
            &"dataset_id".to_string(),
            &"membership_id".to_string(),
            &ListDatasetTaskEventsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**membership_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**cursor:** `Option<String>` — Opaque position returned as `next_cursor` by the preceding page.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">refresh_dataset_task</a>(dataset_id: String, membership_id: String, request: RefreshMembershipRequest) -> Result&lt;TaskMembership, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .refresh_dataset_task(
            &"dataset_id".to_string(),
            &"membership_id".to_string(),
            &RefreshMembershipRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**membership_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">list_dataset_clusters</a>(dataset_id: String) -> Result&lt;Vec&lt;DatasetCluster&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .list_dataset_clusters(&"dataset_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">create_dataset_cluster</a>(dataset_id: String, request: CreateClusterRequest) -> Result&lt;DatasetCluster, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .create_dataset_cluster(
            &"dataset_id".to_string(),
            &CreateClusterRequest {
                color: "color".to_string(),
                label: "label".to_string(),
                description: None,
                create_cluster_request_idempotency_key: None,
                similarity_center: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**color:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**create_cluster_request_idempotency_key:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**label:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**similarity_center:** `Option<Option<Vec<f64>>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">delete_dataset_cluster</a>(dataset_id: String, cluster_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .delete_dataset_cluster(&"dataset_id".to_string(), &"cluster_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**cluster_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">update_dataset_cluster</a>(dataset_id: String, cluster_id: String, request: UpdateClusterRequest) -> Result&lt;DatasetCluster, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .update_dataset_cluster(
            &"dataset_id".to_string(),
            &"cluster_id".to_string(),
            &UpdateClusterRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**cluster_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**color:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**label:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**similarity_center:** `Option<Option<Vec<f64>>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">list_dataset_saved_views</a>(dataset_id: String) -> Result&lt;Vec&lt;DatasetSavedView&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .list_dataset_saved_views(&"dataset_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">create_dataset_saved_view</a>(dataset_id: String, request: CreateSavedViewRequest) -> Result&lt;DatasetSavedView, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .create_dataset_saved_view(
            &"dataset_id".to_string(),
            &CreateSavedViewRequest {
                name: "name".to_string(),
                scope: CreateSavedViewRequestScope::Personal,
                state: CreateSavedViewRequestState {
                    ..Default::default()
                },
                description: None,
                create_saved_view_request_idempotency_key: None,
                schema_version: None,
                shortcut: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**create_saved_view_request_idempotency_key:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**schema_version:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**scope:** `CreateSavedViewRequestScope` 
    
</dd>
</dl>

<dl>
<dd>

**shortcut:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**state:** `CreateSavedViewRequestState` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">delete_dataset_saved_view</a>(dataset_id: String, view_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .delete_dataset_saved_view(&"dataset_id".to_string(), &"view_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**view_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">update_dataset_saved_view</a>(dataset_id: String, view_id: String, request: DatasetSavedViewPatch) -> Result&lt;DatasetSavedView, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .update_dataset_saved_view(
            &"dataset_id".to_string(),
            &"view_id".to_string(),
            &DatasetSavedViewPatch {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**view_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**created_by:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**scope:** `Option<Option<DatasetSavedViewPatchScope>>` 
    
</dd>
</dl>

<dl>
<dd>

**shortcut:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**state:** `Option<Option<DatasetSavedViewPatchState>>` 
    
</dd>
</dl>

<dl>
<dd>

**updated_at:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">list_dataset_versions</a>(dataset_id: String) -> Result&lt;Vec&lt;TaskSuiteVersion&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .list_dataset_versions(&"dataset_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">publish_dataset_version</a>(dataset_id: String, request: PublishVersionRequest) -> Result&lt;TaskSuiteVersion, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .publish_dataset_version(
            &"dataset_id".to_string(),
            &PublishVersionRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**publish_version_request_idempotency_key:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**label:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">get_dataset_version</a>(dataset_id: String, version_id: String) -> Result&lt;TaskSuiteSnapshot, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .get_dataset_version(&"dataset_id".to_string(), &"version_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**version_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.datasets.<a href="/src/api/resources/datasets/client.rs">list_dataset_evaluation_runs</a>(dataset_id: String) -> Result&lt;Vec&lt;TaskSuiteEvalRun&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .datasets
        .list_dataset_evaluation_runs(&"dataset_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dataset_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## environments
<details><summary><code>client.environments.<a href="/src/api/resources/environments/client.rs">list_environments</a>() -> Result&lt;ListEnvironmentsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client.environments.list_environments(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.environments.<a href="/src/api/resources/environments/client.rs">create_environment</a>(request: CreateEnvironmentRequest) -> Result&lt;EnvironmentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .environments
        .create_environment(
            &CreateEnvironmentRequest {
                slug: "support-sandbox".to_string(),
                label: "Support sandbox".to_string(),
                description: Some("Isolated environment for support-agent backtests.".to_string()),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**slug:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**label:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.environments.<a href="/src/api/resources/environments/client.rs">get_environment</a>(environment_id: String) -> Result&lt;EnvironmentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .environments
        .get_environment(&"environment_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**environment_id:** `String` — Environment ID or slug.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.environments.<a href="/src/api/resources/environments/client.rs">list_environment_versions</a>(environment_id: String) -> Result&lt;Vec&lt;EnvironmentVersionRecord&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .environments
        .list_environment_versions(&"environment_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**environment_id:** `String` — Environment ID or slug.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.environments.<a href="/src/api/resources/environments/client.rs">create_environment_version</a>(environment_id: String, request: CreateEnvironmentVersionRequest) -> Result&lt;EnvironmentVersionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .environments
        .create_environment_version(
            &"environment_id".to_string(),
            &CreateEnvironmentVersionRequest {
                version: "version".to_string(),
                spec: None,
                status: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**environment_id:** `String` — Environment ID or slug.
    
</dd>
</dl>

<dl>
<dd>

**version:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**spec:** `Option<EnvironmentSpec>` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<EnvironmentVersionStatus>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.environments.<a href="/src/api/resources/environments/client.rs">get_environment_version</a>(environment_id: String, version_selector: String) -> Result&lt;EnvironmentVersionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .environments
        .get_environment_version(
            &"environment_id".to_string(),
            &"version_selector".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**environment_id:** `String` — Environment ID or slug.
    
</dd>
</dl>

<dl>
<dd>

**version_selector:** `String` — Environment-version ID or version label.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.environments.<a href="/src/api/resources/environments/client.rs">compile_environment_version</a>(environment_id: String, version_selector: String, request: CompileEnvironmentRequest) -> Result&lt;CompileEnvironmentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .environments
        .compile_environment_version(
            &"environment_id".to_string(),
            &"version_selector".to_string(),
            &CompileEnvironmentRequest {
                dataset_snapshot_id: "datasetSnapshotId".to_string(),
                scenario_id: "scenarioId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**environment_id:** `String` — Environment ID or slug.
    
</dd>
</dl>

<dl>
<dd>

**version_selector:** `String` — Environment-version ID or version label.
    
</dd>
</dl>

<dl>
<dd>

**dataset_snapshot_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**scenario_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## backtests
<details><summary><code>client.backtests.<a href="/src/api/resources/backtests/client.rs">get_backtests_availability</a>() -> Result&lt;BacktestsAvailability, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client.backtests.get_backtests_availability(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.backtests.<a href="/src/api/resources/backtests/client.rs">list_backtest_jobs</a>(mode: Option&lt;Option&lt;String&gt;&gt;, status: Option&lt;Option&lt;String&gt;&gt;, limit: Option&lt;Option&lt;i64&gt;&gt;, cursor: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListBacktestJobsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .backtests
        .list_backtest_jobs(
            &ListBacktestJobsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**mode:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**cursor:** `Option<String>` — Opaque position returned as `next_cursor` by the preceding page.
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Deprecated compatibility input. Pass the opaque `cursor` instead.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.backtests.<a href="/src/api/resources/backtests/client.rs">create_backtest_job</a>(request: CreateBacktestJobRequest) -> Result&lt;CreateBacktestJobResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns 202 after the durable job and its trials have been admitted.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .backtests
        .create_backtest_job(
            &CreateBacktestJobRequest {
                name: "name".to_string(),
                recipe: CreateBacktestJobRequestRecipe {
                    agents: vec![CreateBacktestJobRequestRecipeAgentsItem {
                        hue: "hue".to_string(),
                        id: "id".to_string(),
                        label: "label".to_string(),
                        notes: "notes".to_string(),
                        ..Default::default()
                    }],
                    data: CreateBacktestJobRequestRecipeData {
                        dataset: None,
                        dataset_label: None,
                        kind: CreateBacktestJobRequestRecipeDataKind::Composed,
                        saved_as: None,
                        scenarios: vec![CreateBacktestJobRequestRecipeDataScenariosItem {
                            accepted: None,
                            bucket: None,
                            cluster_id: None,
                            cluster_label: None,
                            confidence: None,
                            count: 1,
                            id: "id".to_string(),
                            kind: CreateBacktestJobRequestRecipeDataScenariosItemKind::Adversarial,
                            label: "label".to_string(),
                        }],
                        sources: vec![CreateBacktestJobRequestRecipeDataSourcesItem {
                            count: 1,
                            filters: None,
                            id: "id".to_string(),
                            kind: CreateBacktestJobRequestRecipeDataSourcesItemKind::Prod,
                            label: "label".to_string(),
                        }],
                    },
                    environment: None,
                    graders: vec![CreateBacktestJobRequestRecipeGradersItem {
                        code: None,
                        evidence: None,
                        id: "id".to_string(),
                        judge: None,
                        kind: CreateBacktestJobRequestRecipeGradersItemKind::Rubric,
                        label: "label".to_string(),
                        pass_threshold: None,
                        scorer_id: None,
                        source: CreateBacktestJobRequestRecipeGradersItemSource::Proposed,
                        weight: CreateBacktestJobRequestRecipeGradersItemWeight::Low,
                    }],
                    mode: CreateBacktestJobRequestRecipeMode::Replay,
                    name: "name".to_string(),
                    seed: None,
                },
                cases: None,
                evaluator_profile_id: None,
                n_concurrent: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**cases:** `Option<Option<Vec<CreateBacktestJobRequestCasesItem>>>` 
    
</dd>
</dl>

<dl>
<dd>

**evaluator_profile_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**n_concurrent:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**recipe:** `CreateBacktestJobRequestRecipe` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.backtests.<a href="/src/api/resources/backtests/client.rs">get_backtest_job</a>(job_id: String) -> Result&lt;BacktestJobDetailResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .backtests
        .get_backtest_job(&"job_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**job_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.backtests.<a href="/src/api/resources/backtests/client.rs">list_backtest_job_trials</a>(job_id: String, limit: Option&lt;Option&lt;i64&gt;&gt;, cursor: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListBacktestJobTrialsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .backtests
        .list_backtest_job_trials(
            &"job_id".to_string(),
            &ListBacktestJobTrialsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**job_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**cursor:** `Option<String>` — Opaque position returned as `next_cursor` by the preceding page.
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Deprecated compatibility input. Pass the opaque `cursor` instead.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.backtests.<a href="/src/api/resources/backtests/client.rs">get_backtest_trial</a>(job_id: String, trial_id: String) -> Result&lt;BacktestTrialDetailResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .backtests
        .get_backtest_trial(&"job_id".to_string(), &"trial_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**job_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**trial_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.backtests.<a href="/src/api/resources/backtests/client.rs">cancel_backtest_job</a>(job_id: String) -> Result&lt;CancelBacktestJobResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .backtests
        .cancel_backtest_job(&"job_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**job_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.backtests.<a href="/src/api/resources/backtests/client.rs">stream_backtest_job_events</a>(job_id: String) -> Result&lt;Stream&lt;Vec&lt;u8&gt;&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .backtests
        .stream_backtest_job_events(&"job_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**job_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## credentials
<details><summary><code>client.credentials.<a href="/src/api/resources/credentials/client.rs">list_sdk_keys</a>() -> Result&lt;SdkKeyListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client.credentials.list_sdk_keys(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.credentials.<a href="/src/api/resources/credentials/client.rs">create_sdk_key</a>(request: CreateSdkKeyRequest) -> Result&lt;CreatedSdkKey, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

The bearer secret is returned once and is not stored in plaintext.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .credentials
        .create_sdk_key(
            &CreateSdkKeyRequest {
                name: "name".to_string(),
                scopes: vec![CreateSdkKeyRequestScopesItem::TracesWrite],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**scopes:** `Vec<CreateSdkKeyRequestScopesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.credentials.<a href="/src/api/resources/credentials/client.rs">revoke_sdk_key</a>(key_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chroniclelabs::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Chronicle::new(config).expect("Failed to build client");
    client
        .credentials
        .revoke_sdk_key(&"key_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**key_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

