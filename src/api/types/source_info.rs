pub use crate::prelude::*;

/// A connected source and what has been seen from it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SourceInfo {
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub event_types: Vec<String>,
    #[serde(default)]
    pub event_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<DateTime<FixedOffset>>,
}

impl SourceInfo {
    pub fn builder() -> SourceInfoBuilder {
        <SourceInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourceInfoBuilder {
    source: Option<String>,
    event_types: Option<Vec<String>>,
    event_count: Option<i64>,
    first_seen: Option<DateTime<FixedOffset>>,
    last_seen: Option<DateTime<FixedOffset>>,
}

impl SourceInfoBuilder {
    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn event_types(mut self, value: Vec<String>) -> Self {
        self.event_types = Some(value);
        self
    }

    pub fn event_count(mut self, value: i64) -> Self {
        self.event_count = Some(value);
        self
    }

    pub fn first_seen(mut self, value: DateTime<FixedOffset>) -> Self {
        self.first_seen = Some(value);
        self
    }

    pub fn last_seen(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_seen = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SourceInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source`](SourceInfoBuilder::source)
    /// - [`event_types`](SourceInfoBuilder::event_types)
    /// - [`event_count`](SourceInfoBuilder::event_count)
    pub fn build(self) -> Result<SourceInfo, BuildError> {
        Ok(SourceInfo {
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            event_types: self
                .event_types
                .ok_or_else(|| BuildError::missing_field("event_types"))?,
            event_count: self
                .event_count
                .ok_or_else(|| BuildError::missing_field("event_count"))?,
            first_seen: self.first_seen,
            last_seen: self.last_seen,
        })
    }
}
