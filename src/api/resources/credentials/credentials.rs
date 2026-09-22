use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct CredentialsClient {
    pub http_client: HttpClient,
}

impl CredentialsClient {
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
    ///     client.credentials.list_sdk_keys(None).await;
    /// }
    /// ```
    pub async fn list_sdk_keys(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<SdkKeyListResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/sdk-keys", None, None, options)
            .await
    }

    /// The bearer secret is returned once and is not stored in plaintext.
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
    ///         .credentials
    ///         .create_sdk_key(
    ///             &CreateSdkKeyRequest {
    ///                 name: "name".to_string(),
    ///                 scopes: vec![CreateSdkKeyRequestScopesItem::TracesWrite],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_sdk_key(
        &self,
        request: &CreateSdkKeyRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreatedSdkKey, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sdk-keys",
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
    ///         .credentials
    ///         .revoke_sdk_key(&"key_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn revoke_sdk_key(
        &self,
        key_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/sdk-keys/{}", key_id),
                None,
                None,
                options,
            )
            .await
    }
}
