pub use crate::prelude::*;

/// Stable machine-readable slug to branch on
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ErrorResponseCode {
    NotFound,
    BadRequest,
    Unauthorized,
    ValidationError,
    UnsupportedMediaType,
    PayloadTooLarge,
    RateLimited,
    StreamReplayLimitExceeded,
    StreamUnavailable,
    ServiceOverloaded,
    RequestTimeout,
    StreamError,
    StoreError,
    InternalError,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ErrorResponseCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NotFound => serializer.serialize_str("not_found"),
            Self::BadRequest => serializer.serialize_str("bad_request"),
            Self::Unauthorized => serializer.serialize_str("unauthorized"),
            Self::ValidationError => serializer.serialize_str("validation_error"),
            Self::UnsupportedMediaType => serializer.serialize_str("unsupported_media_type"),
            Self::PayloadTooLarge => serializer.serialize_str("payload_too_large"),
            Self::RateLimited => serializer.serialize_str("rate_limited"),
            Self::StreamReplayLimitExceeded => {
                serializer.serialize_str("stream_replay_limit_exceeded")
            }
            Self::StreamUnavailable => serializer.serialize_str("stream_unavailable"),
            Self::ServiceOverloaded => serializer.serialize_str("service_overloaded"),
            Self::RequestTimeout => serializer.serialize_str("request_timeout"),
            Self::StreamError => serializer.serialize_str("stream_error"),
            Self::StoreError => serializer.serialize_str("store_error"),
            Self::InternalError => serializer.serialize_str("internal_error"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ErrorResponseCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "not_found" => Ok(Self::NotFound),
            "bad_request" => Ok(Self::BadRequest),
            "unauthorized" => Ok(Self::Unauthorized),
            "validation_error" => Ok(Self::ValidationError),
            "unsupported_media_type" => Ok(Self::UnsupportedMediaType),
            "payload_too_large" => Ok(Self::PayloadTooLarge),
            "rate_limited" => Ok(Self::RateLimited),
            "stream_replay_limit_exceeded" => Ok(Self::StreamReplayLimitExceeded),
            "stream_unavailable" => Ok(Self::StreamUnavailable),
            "service_overloaded" => Ok(Self::ServiceOverloaded),
            "request_timeout" => Ok(Self::RequestTimeout),
            "stream_error" => Ok(Self::StreamError),
            "store_error" => Ok(Self::StoreError),
            "internal_error" => Ok(Self::InternalError),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ErrorResponseCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "not_found"),
            Self::BadRequest => write!(f, "bad_request"),
            Self::Unauthorized => write!(f, "unauthorized"),
            Self::ValidationError => write!(f, "validation_error"),
            Self::UnsupportedMediaType => write!(f, "unsupported_media_type"),
            Self::PayloadTooLarge => write!(f, "payload_too_large"),
            Self::RateLimited => write!(f, "rate_limited"),
            Self::StreamReplayLimitExceeded => write!(f, "stream_replay_limit_exceeded"),
            Self::StreamUnavailable => write!(f, "stream_unavailable"),
            Self::ServiceOverloaded => write!(f, "service_overloaded"),
            Self::RequestTimeout => write!(f, "request_timeout"),
            Self::StreamError => write!(f, "stream_error"),
            Self::StoreError => write!(f, "store_error"),
            Self::InternalError => write!(f, "internal_error"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
