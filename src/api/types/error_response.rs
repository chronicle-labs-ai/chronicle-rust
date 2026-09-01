pub use crate::prelude::*;

/// RFC 9457 problem details, served as `application/problem+json`. The `code` member is the stable machine-readable slug to branch on; `type` and `title` are stable per problem class, and `detail` varies per occurrence. The `error` and `message` members are retained for existing clients and carry the same values as `code` and `detail`. The request identifier appears both in the `x-request-id` response header, for code to read and log, and in the `request_id` member, so it survives being copied into a support thread.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ErrorResponse {
    /// URI identifying the problem class
    #[serde(default)]
    pub r#type: String,
    /// Short summary of the problem class, stable across occurrences
    #[serde(default)]
    pub title: String,
    /// HTTP status code, always equal to the response status
    #[serde(default)]
    pub status: i64,
    /// Explanation specific to this occurrence
    #[serde(default)]
    pub detail: String,
    /// Stable machine-readable slug to branch on
    pub code: ErrorResponseCode,
    /// Retained for existing clients. Same value as `code`.
    #[serde(default)]
    pub error: String,
    /// Retained for existing clients. Same value as `detail`.
    #[serde(default)]
    pub message: String,
    /// Whether the same request may succeed when retried without modification
    #[serde(default)]
    pub retryable: bool,
    /// Same value as the `x-request-id` response header. Quote it when reporting a problem; it identifies the exact request in our logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl ErrorResponse {
    pub fn builder() -> ErrorResponseBuilder {
        <ErrorResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ErrorResponseBuilder {
    r#type: Option<String>,
    title: Option<String>,
    status: Option<i64>,
    detail: Option<String>,
    code: Option<ErrorResponseCode>,
    error: Option<String>,
    message: Option<String>,
    retryable: Option<bool>,
    request_id: Option<String>,
    details: Option<serde_json::Value>,
}

impl ErrorResponseBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn status(mut self, value: i64) -> Self {
        self.status = Some(value);
        self
    }

    pub fn detail(mut self, value: impl Into<String>) -> Self {
        self.detail = Some(value.into());
        self
    }

    pub fn code(mut self, value: ErrorResponseCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn retryable(mut self, value: bool) -> Self {
        self.retryable = Some(value);
        self
    }

    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    pub fn details(mut self, value: serde_json::Value) -> Self {
        self.details = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ErrorResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ErrorResponseBuilder::r#type)
    /// - [`title`](ErrorResponseBuilder::title)
    /// - [`status`](ErrorResponseBuilder::status)
    /// - [`detail`](ErrorResponseBuilder::detail)
    /// - [`code`](ErrorResponseBuilder::code)
    /// - [`error`](ErrorResponseBuilder::error)
    /// - [`message`](ErrorResponseBuilder::message)
    /// - [`retryable`](ErrorResponseBuilder::retryable)
    pub fn build(self) -> Result<ErrorResponse, BuildError> {
        Ok(ErrorResponse {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            detail: self
                .detail
                .ok_or_else(|| BuildError::missing_field("detail"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            retryable: self
                .retryable
                .ok_or_else(|| BuildError::missing_field("retryable"))?,
            request_id: self.request_id,
            details: self.details,
        })
    }
}
