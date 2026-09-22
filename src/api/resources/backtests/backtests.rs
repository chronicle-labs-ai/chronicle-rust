use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions, SseStream};
use reqwest::Method;

pub struct BacktestsClient {
    pub http_client: HttpClient,
}

impl BacktestsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

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
    ///     client.backtests.get_backtests_availability(None).await;
    /// }
    /// ```
    pub async fn get_backtests_availability(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<BacktestsAvailability, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/backtests/availability",
                None,
                None,
                options,
            )
            .await
    }

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
    ///         .backtests
    ///         .list_backtest_jobs(
    ///             &ListBacktestJobsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_backtest_jobs(
        &self,
        request: &ListBacktestJobsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListBacktestJobsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/backtests/jobs",
                None,
                QueryBuilder::new()
                    .string("mode", request.mode.clone())
                    .string("status", request.status.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns 202 after the durable job and its trials have been admitted.
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
    ///         .backtests
    ///         .create_backtest_job(
    ///             &CreateBacktestJobRequest {
    ///                 name: "name".to_string(),
    ///                 recipe: CreateBacktestJobRequestRecipe {
    ///                     agents: vec![CreateBacktestJobRequestRecipeAgentsItem {
    ///                         hue: "hue".to_string(),
    ///                         id: "id".to_string(),
    ///                         label: "label".to_string(),
    ///                         notes: "notes".to_string(),
    ///                         ..Default::default()
    ///                     }],
    ///                     data: CreateBacktestJobRequestRecipeData {
    ///                         dataset: None,
    ///                         dataset_label: None,
    ///                         kind: CreateBacktestJobRequestRecipeDataKind::Composed,
    ///                         saved_as: None,
    ///                         scenarios: vec![CreateBacktestJobRequestRecipeDataScenariosItem {
    ///                             accepted: None,
    ///                             bucket: None,
    ///                             cluster_id: None,
    ///                             cluster_label: None,
    ///                             confidence: None,
    ///                             count: 1,
    ///                             id: "id".to_string(),
    ///                             kind: CreateBacktestJobRequestRecipeDataScenariosItemKind::Adversarial,
    ///                             label: "label".to_string(),
    ///                         }],
    ///                         sources: vec![CreateBacktestJobRequestRecipeDataSourcesItem {
    ///                             count: 1,
    ///                             filters: None,
    ///                             id: "id".to_string(),
    ///                             kind: CreateBacktestJobRequestRecipeDataSourcesItemKind::Prod,
    ///                             label: "label".to_string(),
    ///                         }],
    ///                     },
    ///                     environment: None,
    ///                     graders: vec![CreateBacktestJobRequestRecipeGradersItem {
    ///                         code: None,
    ///                         evidence: None,
    ///                         id: "id".to_string(),
    ///                         judge: None,
    ///                         kind: CreateBacktestJobRequestRecipeGradersItemKind::Rubric,
    ///                         label: "label".to_string(),
    ///                         pass_threshold: None,
    ///                         scorer_id: None,
    ///                         source: CreateBacktestJobRequestRecipeGradersItemSource::Proposed,
    ///                         weight: CreateBacktestJobRequestRecipeGradersItemWeight::Low,
    ///                     }],
    ///                     mode: CreateBacktestJobRequestRecipeMode::Replay,
    ///                     name: "name".to_string(),
    ///                     seed: None,
    ///                 },
    ///                 cases: None,
    ///                 evaluator_profile_id: None,
    ///                 n_concurrent: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_backtest_job(
        &self,
        request: &CreateBacktestJobRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateBacktestJobResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/backtests/jobs",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

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
    ///         .backtests
    ///         .get_backtest_job(&"job_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_backtest_job(
        &self,
        job_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<BacktestJobDetailResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/backtests/jobs/{}", job_id),
                None,
                None,
                options,
            )
            .await
    }

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
    ///         .backtests
    ///         .list_backtest_job_trials(
    ///             &"job_id".to_string(),
    ///             &ListBacktestJobTrialsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_backtest_job_trials(
        &self,
        job_id: &str,
        request: &ListBacktestJobTrialsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListBacktestJobTrialsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/backtests/jobs/{}/trials", job_id),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

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
    ///         .backtests
    ///         .get_backtest_trial(&"job_id".to_string(), &"trial_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_backtest_trial(
        &self,
        job_id: &str,
        trial_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<BacktestTrialDetailResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/backtests/jobs/{}/trials/{}", job_id, trial_id),
                None,
                None,
                options,
            )
            .await
    }

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
    ///         .backtests
    ///         .cancel_backtest_job(&"job_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn cancel_backtest_job(
        &self,
        job_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CancelBacktestJobResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/backtests/jobs/{}/cancel", job_id),
                None,
                None,
                options,
            )
            .await
    }

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
    ///         .backtests
    ///         .stream_backtest_job_events(&"job_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn stream_backtest_job_events(
        &self,
        job_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SseStream<TrialEvent>, ApiError> {
        self.http_client
            .execute_sse_request(
                Method::GET,
                &format!("v1/backtests/jobs/{}/stream", job_id),
                None,
                None,
                options,
                None,
            )
            .await
    }
}
