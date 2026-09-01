use chroniclelabs::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_discover_list_sources_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client.discover.list_sources(None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/v1/discover/sources", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_discover_list_entity_types_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client.discover.list_entity_types(None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/v1/discover/entity-types", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_discover_list_entities_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client
        .discover
        .list_entities(
            &"entity_type".to_string(),
            &ListEntitiesQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/v1/discover/entities/entity_type", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_discover_get_event_schema_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client
        .discover
        .get_event_schema(&"source".to_string(), &"event_type".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/v1/discover/schema/source/event_type", None, 1)
        .await
        .unwrap();
}
