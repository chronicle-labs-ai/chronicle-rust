use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct DatasetsClient {
    pub http_client: HttpClient,
}

impl DatasetsClient {
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
    ///     client
    ///         .datasets
    ///         .list_datasets(
    ///             &ListDatasetsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_datasets(
        &self,
        request: &ListDatasetsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TaskSuitePage, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/task-suites",
                None,
                QueryBuilder::new()
                    .bool("includeArchived", request.include_archived.clone())
                    .structured_query("query", request.query.clone())
                    .int("limit", request.limit.clone())
                    .string("cursor", request.cursor.clone())
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
    ///         .datasets
    ///         .create_dataset(
    ///             &CreateTaskSuitePayload {
    ///                 name: "name".to_string(),
    ///                 description: None,
    ///                 purpose: None,
    ///                 tags: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_dataset(
        &self,
        request: &CreateTaskSuitePayload,
        options: Option<RequestOptions>,
    ) -> Result<TaskSuite, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/task-suites",
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
    ///         .datasets
    ///         .create_dataset_with_trace(
    ///             &CreateTaskSuiteWithTraceRequest {
    ///                 dataset: CreateTaskSuiteWithTraceRequestDataset {
    ///                     name: "name".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///                 trace: CreateTaskSuiteWithTraceRequestTrace {
    ///                     trace_id: "traceId".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_dataset_with_trace(
        &self,
        request: &CreateTaskSuiteWithTraceRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateTaskSuiteWithTraceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/task-suites/with-trace",
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
    ///         .datasets
    ///         .get_dataset(&"dataset_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_dataset(
        &self,
        dataset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<TaskSuiteDetail, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/task-suites/{}", dataset_id),
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
    ///         .datasets
    ///         .archive_dataset(
    ///             &"dataset_id".to_string(),
    ///             &ArchiveDatasetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn archive_dataset(
        &self,
        dataset_id: &str,
        request: &ArchiveDatasetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/task-suites/{}", dataset_id),
                None,
                QueryBuilder::new()
                    .bool("cascade", request.cascade.clone())
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
    ///         .datasets
    ///         .update_dataset(
    ///             &"dataset_id".to_string(),
    ///             &TaskSuitePatch {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_dataset(
        &self,
        dataset_id: &str,
        request: &TaskSuitePatch,
        options: Option<RequestOptions>,
    ) -> Result<TaskSuite, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/task-suites/{}", dataset_id),
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
    ///         .datasets
    ///         .get_dataset_snapshot(&"dataset_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_dataset_snapshot(
        &self,
        dataset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<TaskSuiteSnapshot, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/task-suites/{}/snapshot", dataset_id),
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
    ///         .datasets
    ///         .list_dataset_traces(
    ///             &"dataset_id".to_string(),
    ///             &ListDatasetTracesQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_dataset_traces(
        &self,
        dataset_id: &str,
        request: &ListDatasetTracesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TaskPage, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/task-suites/{}/traces", dataset_id),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .string("cursor", request.cursor.clone())
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
    ///         .datasets
    ///         .add_trace_to_dataset(
    ///             &"dataset_id".to_string(),
    ///             &AddTaskFromTraceRequest {
    ///                 trace_id: "traceId".to_string(),
    ///                 event_ids: None,
    ///                 add_task_from_trace_request_idempotency_key: None,
    ///                 notes: None,
    ///                 split: None,
    ///                 task: None,
    ///                 trace_synthesized: None,
    ///                 verifiers: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_trace_to_dataset(
        &self,
        dataset_id: &str,
        request: &AddTaskFromTraceRequest,
        options: Option<RequestOptions>,
    ) -> Result<AddTaskFromTraceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/task-suites/{}/traces", dataset_id),
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
    ///         .datasets
    ///         .update_dataset_traces(
    ///             &"dataset_id".to_string(),
    ///             &UpdateTracesRequest {
    ///                 patch: UpdateTracesRequestPatch {
    ///                     ..Default::default()
    ///                 },
    ///                 trace_ids: vec!["traceIds".to_string()],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_dataset_traces(
        &self,
        dataset_id: &str,
        request: &UpdateTracesRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/task-suites/{}/traces", dataset_id),
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
    ///         .datasets
    ///         .remove_trace_from_dataset(
    ///             &"dataset_id".to_string(),
    ///             &"membership_id".to_string(),
    ///             &RemoveTraceFromDatasetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn remove_trace_from_dataset(
        &self,
        dataset_id: &str,
        membership_id: &str,
        request: &RemoveTraceFromDatasetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/task-suites/{}/traces/{}", dataset_id, membership_id),
                None,
                QueryBuilder::new()
                    .string("reason", request.reason.clone())
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
    ///         .datasets
    ///         .refresh_dataset_trace(
    ///             &"dataset_id".to_string(),
    ///             &"membership_id".to_string(),
    ///             &RefreshMembershipRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn refresh_dataset_trace(
        &self,
        dataset_id: &str,
        membership_id: &str,
        request: &RefreshMembershipRequest,
        options: Option<RequestOptions>,
    ) -> Result<TaskMembership, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/task-suites/{}/traces/{}/refresh",
                    dataset_id, membership_id
                ),
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
    ///         .datasets
    ///         .list_dataset_trace_events(
    ///             &"dataset_id".to_string(),
    ///             &"membership_id".to_string(),
    ///             &ListDatasetTraceEventsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_dataset_trace_events(
        &self,
        dataset_id: &str,
        membership_id: &str,
        request: &ListDatasetTraceEventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TaskEventPage, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/task-suites/{}/traces/{}/events",
                    dataset_id, membership_id
                ),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .string("cursor", request.cursor.clone())
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
    ///         .datasets
    ///         .list_trace_dataset_memberships(&"trace_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_trace_dataset_memberships(
        &self,
        trace_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Vec<TaskMembership>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/traces/{}/task-suite-memberships", trace_id),
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
    ///         .datasets
    ///         .list_dataset_tasks(
    ///             &"dataset_id".to_string(),
    ///             &ListDatasetTasksQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_dataset_tasks(
        &self,
        dataset_id: &str,
        request: &ListDatasetTasksQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TaskPage, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/task-suites/{}/tasks", dataset_id),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .string("cursor", request.cursor.clone())
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
    ///         .datasets
    ///         .create_dataset_task(
    ///             &"dataset_id".to_string(),
    ///             &CreateTaskRequest(serde_json::json!({"key":"value"})),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_dataset_task(
        &self,
        dataset_id: &str,
        request: &CreateTaskRequest,
        options: Option<RequestOptions>,
    ) -> Result<Task, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/task-suites/{}/tasks", dataset_id),
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
    ///         .datasets
    ///         .get_dataset_task(
    ///             &"dataset_id".to_string(),
    ///             &"membership_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_dataset_task(
        &self,
        dataset_id: &str,
        membership_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Task, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/task-suites/{}/tasks/{}", dataset_id, membership_id),
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
    ///         .datasets
    ///         .delete_dataset_task(
    ///             &"dataset_id".to_string(),
    ///             &"membership_id".to_string(),
    ///             &DeleteDatasetTaskQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_dataset_task(
        &self,
        dataset_id: &str,
        membership_id: &str,
        request: &DeleteDatasetTaskQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/task-suites/{}/tasks/{}", dataset_id, membership_id),
                None,
                QueryBuilder::new()
                    .string("reason", request.reason.clone())
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
    ///         .datasets
    ///         .update_dataset_task(
    ///             &"dataset_id".to_string(),
    ///             &"membership_id".to_string(),
    ///             &UpdateTaskRequest(serde_json::json!({"key":"value"})),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_dataset_task(
        &self,
        dataset_id: &str,
        membership_id: &str,
        request: &UpdateTaskRequest,
        options: Option<RequestOptions>,
    ) -> Result<Task, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/task-suites/{}/tasks/{}", dataset_id, membership_id),
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
    ///         .datasets
    ///         .set_dataset_task_verifiers(
    ///             &"dataset_id".to_string(),
    ///             &"membership_id".to_string(),
    ///             &SetTaskVerifiersRequest {
    ///                 verifiers: vec![SetTaskVerifiersRequestVerifiersItem {
    ///                     scorer_id: "scorerId".to_string(),
    ///                     ..Default::default()
    ///                 }],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn set_dataset_task_verifiers(
        &self,
        dataset_id: &str,
        membership_id: &str,
        request: &SetTaskVerifiersRequest,
        options: Option<RequestOptions>,
    ) -> Result<Task, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!(
                    "v1/task-suites/{}/tasks/{}/verifiers",
                    dataset_id, membership_id
                ),
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
    ///         .datasets
    ///         .list_dataset_task_events(
    ///             &"dataset_id".to_string(),
    ///             &"membership_id".to_string(),
    ///             &ListDatasetTaskEventsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_dataset_task_events(
        &self,
        dataset_id: &str,
        membership_id: &str,
        request: &ListDatasetTaskEventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TaskEventPage, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/task-suites/{}/tasks/{}/events",
                    dataset_id, membership_id
                ),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .string("cursor", request.cursor.clone())
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
    ///         .datasets
    ///         .refresh_dataset_task(
    ///             &"dataset_id".to_string(),
    ///             &"membership_id".to_string(),
    ///             &RefreshMembershipRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn refresh_dataset_task(
        &self,
        dataset_id: &str,
        membership_id: &str,
        request: &RefreshMembershipRequest,
        options: Option<RequestOptions>,
    ) -> Result<TaskMembership, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/task-suites/{}/tasks/{}/refresh",
                    dataset_id, membership_id
                ),
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
    ///         .datasets
    ///         .list_dataset_clusters(&"dataset_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_dataset_clusters(
        &self,
        dataset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Vec<DatasetCluster>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/task-suites/{}/clusters", dataset_id),
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
    ///         .datasets
    ///         .create_dataset_cluster(
    ///             &"dataset_id".to_string(),
    ///             &CreateClusterRequest {
    ///                 color: "color".to_string(),
    ///                 label: "label".to_string(),
    ///                 description: None,
    ///                 create_cluster_request_idempotency_key: None,
    ///                 similarity_center: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_dataset_cluster(
        &self,
        dataset_id: &str,
        request: &CreateClusterRequest,
        options: Option<RequestOptions>,
    ) -> Result<DatasetCluster, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/task-suites/{}/clusters", dataset_id),
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
    ///         .datasets
    ///         .delete_dataset_cluster(&"dataset_id".to_string(), &"cluster_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_dataset_cluster(
        &self,
        dataset_id: &str,
        cluster_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/task-suites/{}/clusters/{}", dataset_id, cluster_id),
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
    ///         .datasets
    ///         .update_dataset_cluster(
    ///             &"dataset_id".to_string(),
    ///             &"cluster_id".to_string(),
    ///             &UpdateClusterRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_dataset_cluster(
        &self,
        dataset_id: &str,
        cluster_id: &str,
        request: &UpdateClusterRequest,
        options: Option<RequestOptions>,
    ) -> Result<DatasetCluster, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/task-suites/{}/clusters/{}", dataset_id, cluster_id),
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
    ///         .datasets
    ///         .list_dataset_saved_views(&"dataset_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_dataset_saved_views(
        &self,
        dataset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Vec<DatasetSavedView>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/task-suites/{}/saved-views", dataset_id),
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
    ///         .datasets
    ///         .create_dataset_saved_view(
    ///             &"dataset_id".to_string(),
    ///             &CreateSavedViewRequest {
    ///                 name: "name".to_string(),
    ///                 scope: CreateSavedViewRequestScope::Personal,
    ///                 state: CreateSavedViewRequestState {
    ///                     ..Default::default()
    ///                 },
    ///                 description: None,
    ///                 create_saved_view_request_idempotency_key: None,
    ///                 schema_version: None,
    ///                 shortcut: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_dataset_saved_view(
        &self,
        dataset_id: &str,
        request: &CreateSavedViewRequest,
        options: Option<RequestOptions>,
    ) -> Result<DatasetSavedView, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/task-suites/{}/saved-views", dataset_id),
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
    ///         .datasets
    ///         .delete_dataset_saved_view(&"dataset_id".to_string(), &"view_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_dataset_saved_view(
        &self,
        dataset_id: &str,
        view_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/task-suites/{}/saved-views/{}", dataset_id, view_id),
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
    ///         .datasets
    ///         .update_dataset_saved_view(
    ///             &"dataset_id".to_string(),
    ///             &"view_id".to_string(),
    ///             &DatasetSavedViewPatch {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_dataset_saved_view(
        &self,
        dataset_id: &str,
        view_id: &str,
        request: &DatasetSavedViewPatch,
        options: Option<RequestOptions>,
    ) -> Result<DatasetSavedView, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/task-suites/{}/saved-views/{}", dataset_id, view_id),
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
    ///         .datasets
    ///         .list_dataset_versions(&"dataset_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_dataset_versions(
        &self,
        dataset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Vec<TaskSuiteVersion>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/task-suites/{}/versions", dataset_id),
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
    ///         .datasets
    ///         .publish_dataset_version(
    ///             &"dataset_id".to_string(),
    ///             &PublishVersionRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn publish_dataset_version(
        &self,
        dataset_id: &str,
        request: &PublishVersionRequest,
        options: Option<RequestOptions>,
    ) -> Result<TaskSuiteVersion, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/task-suites/{}/versions", dataset_id),
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
    ///         .datasets
    ///         .get_dataset_version(&"dataset_id".to_string(), &"version_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_dataset_version(
        &self,
        dataset_id: &str,
        version_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<TaskSuiteSnapshot, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/task-suites/{}/versions/{}", dataset_id, version_id),
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
    ///         .datasets
    ///         .list_dataset_evaluation_runs(&"dataset_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_dataset_evaluation_runs(
        &self,
        dataset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Vec<TaskSuiteEvalRun>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/task-suites/{}/eval-runs", dataset_id),
                None,
                None,
                options,
            )
            .await
    }
}
