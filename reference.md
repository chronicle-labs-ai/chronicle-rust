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
                source: "my-agent".to_string(),
                topic: "conversations".to_string(),
                event_type: "message.sent".to_string(),
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

