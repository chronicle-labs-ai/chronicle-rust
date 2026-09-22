use chroniclelabs::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_agents_list_agents_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client.agents.list_agents(None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/v1/agents", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_agents_search_agent_hash_index_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client
        .agents
        .search_agent_hash_index(
            &SearchAgentHashIndexQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/v1/agents/hash-index", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_agents_subscribe_to_agent_changes_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client.agents.subscribe_to_agent_changes(None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/v1/agents/subscribe", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_agents_update_agent_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client
        .agents
        .update_agent(
            &"name".to_string(),
            &UpdateAgentRequest {
                ..Default::default()
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("PATCH", "/v1/agents/name", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_agents_get_agent_snapshot_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client
        .agents
        .get_agent_snapshot(&"name".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/v1/agents/name/snapshot", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_agents_pin_latest_agent_version_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client
        .agents
        .pin_latest_agent_version(&"name".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/v1/agents/name/pin-latest", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_agents_create_agent_chat_session_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client
        .agents
        .create_agent_chat_session(&"name".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/v1/agents/name/chat/sessions", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_agents_get_agent_chat_session_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client
        .agents
        .get_agent_chat_session(&"name".to_string(), &"session_id".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/v1/agents/name/chat/sessions/session_id",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_agents_send_agent_chat_message_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client
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

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "POST",
        "/v1/agents/name/chat/sessions/session_id/messages",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_agents_register_agent_artifact_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client
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

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/v1/agents/register", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_agents_record_agent_runs_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client
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

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/v1/agents/runs/batch", None, 1)
        .await
        .unwrap();
}
