pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IngestResponse {
    #[serde(default)]
    pub event_ids: Vec<String>,
    #[serde(default)]
    pub count: i64,
}

impl IngestResponse {
    pub fn builder() -> IngestResponseBuilder {
        <IngestResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IngestResponseBuilder {
    event_ids: Option<Vec<String>>,
    count: Option<i64>,
}

impl IngestResponseBuilder {
    pub fn event_ids(mut self, value: Vec<String>) -> Self {
        self.event_ids = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IngestResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_ids`](IngestResponseBuilder::event_ids)
    /// - [`count`](IngestResponseBuilder::count)
    pub fn build(self) -> Result<IngestResponse, BuildError> {
        Ok(IngestResponse {
            event_ids: self
                .event_ids
                .ok_or_else(|| BuildError::missing_field("event_ids"))?,
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
        })
    }
}
