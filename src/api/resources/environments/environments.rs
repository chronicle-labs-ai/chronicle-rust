use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct EnvironmentsClient {
    pub http_client: HttpClient,
}

impl EnvironmentsClient {
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
    ///     client.environments.list_environments(None).await;
    /// }
    /// ```
    pub async fn list_environments(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<ListEnvironmentsResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/environments", None, None, options)
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
    ///         .environments
    ///         .create_environment(
    ///             &CreateEnvironmentRequest {
    ///                 slug: "slug".to_string(),
    ///                 label: "label".to_string(),
    ///                 description: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_environment(
        &self,
        request: &CreateEnvironmentRequest,
        options: Option<RequestOptions>,
    ) -> Result<EnvironmentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/environments",
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
    ///         .environments
    ///         .get_environment(&"environment_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_environment(
        &self,
        environment_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<EnvironmentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/environments/{}", environment_id),
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
    ///         .environments
    ///         .list_environment_versions(&"environment_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_environment_versions(
        &self,
        environment_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Vec<EnvironmentVersionRecord>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/environments/{}/versions", environment_id),
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
    ///         .environments
    ///         .create_environment_version(
    ///             &"environment_id".to_string(),
    ///             &CreateEnvironmentVersionRequest {
    ///                 version: "version".to_string(),
    ///                 spec: None,
    ///                 status: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_environment_version(
        &self,
        environment_id: &str,
        request: &CreateEnvironmentVersionRequest,
        options: Option<RequestOptions>,
    ) -> Result<EnvironmentVersionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/environments/{}/versions", environment_id),
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
    ///         .environments
    ///         .get_environment_version(
    ///             &"environment_id".to_string(),
    ///             &"version_selector".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_environment_version(
        &self,
        environment_id: &str,
        version_selector: &str,
        options: Option<RequestOptions>,
    ) -> Result<EnvironmentVersionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/environments/{}/versions/{}",
                    environment_id, version_selector
                ),
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
    ///         .environments
    ///         .compile_environment_version(
    ///             &"environment_id".to_string(),
    ///             &"version_selector".to_string(),
    ///             &CompileEnvironmentRequest {
    ///                 dataset_snapshot_id: "datasetSnapshotId".to_string(),
    ///                 scenario_id: "scenarioId".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn compile_environment_version(
        &self,
        environment_id: &str,
        version_selector: &str,
        request: &CompileEnvironmentRequest,
        options: Option<RequestOptions>,
    ) -> Result<CompileEnvironmentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/environments/{}/versions/{}/compile",
                    environment_id, version_selector
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
