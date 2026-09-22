use chroniclelabs::prelude::*;

mod wire_test_utils;

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_backtests_get_backtests_availability_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client.backtests.get_backtests_availability(None).await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/v1/backtests/availability", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_backtests_list_backtest_jobs_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client
        .backtests
        .list_backtest_jobs(
            &ListBacktestJobsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/v1/backtests/jobs", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_backtests_create_backtest_job_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client
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

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/v1/backtests/jobs", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_backtests_get_backtest_job_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client
        .backtests
        .get_backtest_job(&"job_id".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/v1/backtests/jobs/job_id", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_backtests_list_backtest_job_trials_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client
        .backtests
        .list_backtest_job_trials(
            &"job_id".to_string(),
            &ListBacktestJobTrialsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/v1/backtests/jobs/job_id/trials", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_backtests_get_backtest_trial_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client
        .backtests
        .get_backtest_trial(&"job_id".to_string(), &"trial_id".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count(
        "GET",
        "/v1/backtests/jobs/job_id/trials/trial_id",
        None,
        1,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_backtests_cancel_backtest_job_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client
        .backtests
        .cancel_backtest_job(&"job_id".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("POST", "/v1/backtests/jobs/job_id/cancel", None, 1)
        .await
        .unwrap();
}

#[tokio::test]
#[allow(unused_variables, unreachable_code)]
async fn test_backtests_stream_backtest_job_events_with_wiremock() {
    wire_test_utils::reset_wiremock_requests().await.unwrap();
    let wiremock_base_url = wire_test_utils::get_wiremock_base_url();

    let mut config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    config.base_url = wiremock_base_url.to_string();
    let client = Chronicle::new(config).expect("Failed to build client");

    let result = client
        .backtests
        .stream_backtest_job_events(&"job_id".to_string(), None)
        .await;

    assert!(result.is_ok(), "Client method call should succeed");

    wire_test_utils::verify_request_count("GET", "/v1/backtests/jobs/job_id/stream", None, 1)
        .await
        .unwrap();
}
