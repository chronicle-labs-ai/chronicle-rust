use chroniclelabs::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_search_events_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client
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

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/v1/search", None, 1)
        .await
        .unwrap();
}
