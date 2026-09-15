use chroniclelabs::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_links_add_entity_ref_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client
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

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/v1/entity-refs", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_links_create_event_link_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client
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

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/v1/event-links", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_links_link_entities_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client
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

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/v1/link-entity", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_links_traverse_graph_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client
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

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/v1/graph", None, 1)
        .await
        .unwrap();
}
